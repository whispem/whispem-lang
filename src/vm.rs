use crate::chunk::Chunk;
use crate::error::{ErrorKind, WhispemError, WhispemResult};
use crate::opcode::OpCode;
use crate::value::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

// ─────────────────────────────────────────────────────────────────────────────
// CallFrame
// ─────────────────────────────────────────────────────────────────────────────

struct CallFrame {
    chunk:  Chunk,
    ip:     usize,
    locals: HashMap<String, Value>,
}

impl CallFrame {
    fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 0, locals: HashMap::new() }
    }

    #[inline] fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip]; self.ip += 1; b
    }

    #[inline] fn read_u16(&mut self) -> u16 {
        let hi = self.chunk.code[self.ip]     as u16;
        let lo = self.chunk.code[self.ip + 1] as u16;
        self.ip += 2;
        (hi << 8) | lo
    }

    #[inline] fn const_val(&self, idx: u8) -> &Value {
        &self.chunk.constants[idx as usize]
    }

    fn current_line(&self) -> usize {
        self.chunk.lines.get(self.ip.saturating_sub(1)).copied().unwrap_or(0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VM
// ─────────────────────────────────────────────────────────────────────────────

pub struct Vm {
    stack:   Vec<Value>,
    frames:  Vec<CallFrame>,
    globals: HashMap<String, Value>,
    pub functions: HashMap<String, Chunk>,
}

impl Vm {
    pub fn new() -> Self {
        Self { stack: Vec::with_capacity(256), frames: Vec::with_capacity(64),
               globals: HashMap::new(), functions: HashMap::new() }
    }

    pub fn run(&mut self, main_chunk: Chunk) -> WhispemResult<()> {
        self.frames.push(CallFrame::new(main_chunk));
        self.execute()
    }

    fn execute(&mut self) -> WhispemResult<()> {
        loop {
            let byte = self.frame_mut().read_byte();
            let op = OpCode::from_byte(byte).ok_or_else(|| {
                WhispemError::new(ErrorKind::InvalidOpcode(byte), self.frame().current_line(), 0)
            })?;

            match op {
                OpCode::PushConst => {
                    let idx = self.frame_mut().read_byte();
                    let v   = self.frame().const_val(idx).clone();
                    self.stack.push(v);
                }
                OpCode::PushTrue  => self.stack.push(Value::Bool(true)),
                OpCode::PushFalse => self.stack.push(Value::Bool(false)),
                OpCode::PushNone  => self.stack.push(Value::None),

                OpCode::Load => {
                    let idx  = self.frame_mut().read_byte();
                    let name = self.const_str(idx);
                    let val  = self.lookup(&name).ok_or_else(|| {
                        WhispemError::new(ErrorKind::UndefinedVariable(name.clone()),
                                          self.frame().current_line(), 0)
                    })?;
                    self.stack.push(val);
                }
                OpCode::Store => {
                    let idx  = self.frame_mut().read_byte();
                    let name = self.const_str(idx);
                    let val  = self.pop()?;
                    self.store(name, val);
                }

                OpCode::Add => { let (a,b) = self.pop2()?; self.stack.push(self.add(a,b)?); }
                OpCode::Sub => { let (a,b) = self.pop2()?; self.stack.push(self.numeric(a,b,|x,y|x-y)?); }
                OpCode::Mul => { let (a,b) = self.pop2()?; self.stack.push(self.numeric(a,b,|x,y|x*y)?); }
                OpCode::Div => { let (a,b) = self.pop2()?; self.stack.push(self.divmod(a,b,false)?); }
                OpCode::Mod => { let (a,b) = self.pop2()?; self.stack.push(self.divmod(a,b,true)?); }
                OpCode::Neg => {
                    let a = self.pop()?;
                    match a {
                        Value::Number(n) => self.stack.push(Value::Number(-n)),
                        other => return Err(self.type_err("number", other.type_name())),
                    }
                }

                OpCode::Eq  => { let (a,b) = self.pop2()?; self.stack.push(Value::Bool(self.eq(&a,&b))); }
                OpCode::Neq => { let (a,b) = self.pop2()?; self.stack.push(Value::Bool(!self.eq(&a,&b))); }
                OpCode::Lt  => { let (a,b) = self.pop2()?; self.stack.push(self.cmp(a,b,|x,y|x<y,|x,y|x<y)?); }
                OpCode::Lte => { let (a,b) = self.pop2()?; self.stack.push(self.cmp(a,b,|x,y|x<=y,|x,y|x<=y)?); }
                OpCode::Gt  => { let (a,b) = self.pop2()?; self.stack.push(self.cmp(a,b,|x,y|x>y,|x,y|x>y)?); }
                OpCode::Gte => { let (a,b) = self.pop2()?; self.stack.push(self.cmp(a,b,|x,y|x>=y,|x,y|x>=y)?); }
                OpCode::Not => { let a = self.pop()?; self.stack.push(Value::Bool(!a.is_truthy())); }

                OpCode::Jump => {
                    let target = self.frame_mut().read_u16() as usize;
                    self.frame_mut().ip = target;
                }
                OpCode::JumpIfFalse => {
                    let target = self.frame_mut().read_u16() as usize;
                    let cond   = self.pop()?;
                    if !cond.is_truthy() { self.frame_mut().ip = target; }
                }
                OpCode::JumpIfTrue => {
                    let target = self.frame_mut().read_u16() as usize;
                    let cond   = self.pop()?;
                    if cond.is_truthy() { self.frame_mut().ip = target; }
                }

                OpCode::Call => {
                    let name_idx = self.frame_mut().read_byte();
                    let argc     = self.frame_mut().read_byte() as usize;
                    let name     = self.const_str(name_idx);

                    // Collect args (pushed left-to-right; top of stack = last arg).
                    let mut args: Vec<Value> = (0..argc).map(|_| self.pop()).collect::<WhispemResult<_>>()?;
                    args.reverse(); // restore left-to-right order

                    if let Some(result) = self.call_builtin(&name, &args)? {
                        self.stack.push(result);
                    } else {
                        let chunk = self.functions.get(&name).cloned().ok_or_else(|| {
                            WhispemError::new(ErrorKind::UndefinedFunction(name.clone()),
                                              self.frame().current_line(), 0)
                        })?;

                        let mut new_frame = CallFrame::new(chunk);
                        for (k, v) in &self.globals {
                            new_frame.locals.insert(k.clone(), v.clone());
                        }
                        // push in forward order so preamble pops correctly
                        for arg in args {
                            self.stack.push(arg);
                        }
                        self.frames.push(new_frame);
                    }
                }

                OpCode::Return => {
                    let val = self.pop()?;
                    self.frames.pop();
                    self.stack.push(val);
                }

                OpCode::ReturnNone => {
                    self.frames.pop();
                    self.stack.push(Value::None);
                }

                OpCode::MakeArray => {
                    let n = self.frame_mut().read_byte() as usize;
                    let mut elems: Vec<Value> = (0..n).map(|_| self.pop()).collect::<WhispemResult<_>>()?;
                    elems.reverse();
                    self.stack.push(Value::Array(elems));
                }

                OpCode::MakeDict => {
                    let n = self.frame_mut().read_byte() as usize;
                    // Each pair: key was pushed first, then value. Pop value then key.
                    let mut pairs: Vec<(String, Value)> = Vec::with_capacity(n);
                    for _ in 0..n {
                        let val = self.pop()?;
                        let key = self.pop()?;
                        pairs.push((self.to_dict_key(key)?, val));
                    }
                    pairs.reverse();
                    let map: HashMap<String, Value> = pairs.into_iter().collect();
                    self.stack.push(Value::Dict(map));
                }

                OpCode::GetIndex => {
                    let idx = self.pop()?;
                    let obj = self.pop()?;
                    self.stack.push(self.get_index(obj, idx)?);
                }

                // SET_INDEX: pops new_val, idx, obj; pushes mutated obj for STORE to write back.
                OpCode::SetIndex => {
                    let new_val = self.pop()?;
                    let idx     = self.pop()?;
                    let obj     = self.pop()?;
                    let mutated = self.set_index(obj, idx, new_val)?;
                    self.stack.push(mutated);
                }

                OpCode::Pop => {
                    self.pop()?;
                }

                OpCode::Print => {
                    let val = self.pop()?;
                    println!("{}", val.format());
                }

                OpCode::Halt => {
                    self.frames.pop();
                    return Ok(());
                }
            }
        }
    }

    // ── Built-in functions ────────────────────────────────────────────────

    fn call_builtin(&self, name: &str, args: &[Value]) -> WhispemResult<Option<Value>> {
        let line = self.frame().current_line();

        let result = match name {
            "length" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Array(a) => Value::Number(a.len() as f64),
                    Value::Str(s)   => Value::Number(s.len() as f64),
                    Value::Dict(d)  => Value::Number(d.len() as f64),
                    other => return Err(self.type_err_at("array, string, or dict", other.type_name(), line)),
                }
            }
            "push" => {
                self.arity(name, 2, args.len(), line)?;
                match args[0].clone() {
                    Value::Array(mut a) => { a.push(args[1].clone()); Value::Array(a) }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "pop" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Array(mut a) => {
                        if a.is_empty() { return Err(WhispemError::new(ErrorKind::EmptyArray, line, 0)); }
                        a.pop().unwrap()
                    }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "reverse" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Array(mut a) => { a.reverse(); Value::Array(a) }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "slice" => {
                self.arity(name, 3, args.len(), line)?;
                let start = self.to_usize(&args[1], line)?;
                let end   = self.to_usize(&args[2], line)?;
                match args[0].clone() {
                    Value::Array(a) => {
                        if start > end  { return Err(WhispemError::new(ErrorKind::InvalidSlice { start, end }, line, 0)); }
                        if end > a.len(){ return Err(WhispemError::new(ErrorKind::SliceOutOfBounds { end, length: a.len() }, line, 0)); }
                        Value::Array(a[start..end].to_vec())
                    }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "range" => {
                self.arity(name, 2, args.len(), line)?;
                let start = self.to_i64(&args[0], line)?;
                let end   = self.to_i64(&args[1], line)?;
                Value::Array((start..end).map(|i| Value::Number(i as f64)).collect())
            }
            "input" => {
                if args.len() > 1 { return Err(WhispemError::new(ErrorKind::ArgumentCount { name: "input".into(), expected: 1, got: args.len() }, line, 0)); }
                let prompt = if args.is_empty() { String::new() } else {
                    match &args[0] { Value::Str(s) => s.clone(), other => return Err(self.type_err_at("string", other.type_name(), line)) }
                };
                if !prompt.is_empty() { print!("{}", prompt); io::stdout().flush().unwrap(); }
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                Value::Str(buf.trim_end_matches('\n').trim_end_matches('\r').to_string())
            }
            "read_file" => {
                self.arity(name, 1, args.len(), line)?;
                let path = match &args[0] { Value::Str(s) => s.clone(), other => return Err(self.type_err_at("string", other.type_name(), line)) };
                fs::read_to_string(&path).map(Value::Str)
                    .map_err(|e| WhispemError::new(ErrorKind::FileRead { path: path.clone(), reason: e.to_string() }, line, 0))?
            }
            "write_file" => {
                self.arity(name, 2, args.len(), line)?;
                let path = match &args[0] { Value::Str(s) => s.clone(), other => return Err(self.type_err_at("string", other.type_name(), line)) };
                let content = args[1].format();
                fs::write(&path, content).map(|_| Value::None)
                    .map_err(|e| WhispemError::new(ErrorKind::FileWrite { path: path.clone(), reason: e.to_string() }, line, 0))?
            }
            "keys" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Dict(map) => {
                        let mut ks: Vec<Value> = map.keys().map(|k| Value::Str(k.clone())).collect();
                        ks.sort_by(|a,b| a.format().cmp(&b.format()));
                        Value::Array(ks)
                    }
                    other => return Err(self.type_err_at("dict", other.type_name(), line)),
                }
            }
            "values" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Dict(map) => {
                        let mut pairs: Vec<(String,Value)> = map.into_iter().collect();
                        pairs.sort_by(|(a,_),(b,_)| a.cmp(b));
                        Value::Array(pairs.into_iter().map(|(_,v)| v).collect())
                    }
                    other => return Err(self.type_err_at("dict", other.type_name(), line)),
                }
            }
            "has_key" => {
                self.arity(name, 2, args.len(), line)?;
                match &args[0] {
                    Value::Dict(map) => { let k = self.to_dict_key(args[1].clone())?; Value::Bool(map.contains_key(&k)) }
                    other => return Err(self.type_err_at("dict", other.type_name(), line)),
                }
            }
            _ => return Ok(None),
        };
        Ok(Some(result))
    }

    // ── Variable storage ──────────────────────────────────────────────────

    fn lookup(&self, name: &str) -> Option<Value> {
        if let Some(frame) = self.frames.last() {
            if let Some(v) = frame.locals.get(name) { return Some(v.clone()); }
        }
        self.globals.get(name).cloned()
    }

    fn store(&mut self, name: String, value: Value) {
        if self.frames.len() > 1 {
            if let Some(frame) = self.frames.last_mut() {
                frame.locals.insert(name, value);
                return;
            }
        }
        self.globals.insert(name, value);
    }

    // ── Collection helpers ────────────────────────────────────────────────

    fn get_index(&self, obj: Value, idx: Value) -> WhispemResult<Value> {
        let line = self.frame().current_line();
        match obj {
            Value::Array(a) => {
                let i = self.to_usize(&idx, line)?;
                if i >= a.len() { return Err(WhispemError::new(ErrorKind::IndexOutOfBounds { index: i, length: a.len() }, line, 0)); }
                Ok(a[i].clone())
            }
            Value::Dict(map) => {
                let key = self.to_dict_key(idx)?;
                map.get(&key).cloned().ok_or_else(|| {
                    WhispemError::new(ErrorKind::UndefinedVariable(format!("dict key \"{}\"", key)), line, 0)
                })
            }
            other => Err(self.type_err_at("array or dict", other.type_name(), line)),
        }
    }

    /// Mutate `obj` at `idx` and return the mutated object.
    fn set_index(&self, obj: Value, idx: Value, new_val: Value) -> WhispemResult<Value> {
        let line = self.frame().current_line();
        match obj {
            Value::Array(mut a) => {
                let i = self.to_usize(&idx, line)?;
                if i >= a.len() { return Err(WhispemError::new(ErrorKind::IndexOutOfBounds { index: i, length: a.len() }, line, 0)); }
                a[i] = new_val;
                Ok(Value::Array(a))
            }
            Value::Dict(mut map) => {
                let key = self.to_dict_key(idx)?;
                map.insert(key, new_val);
                Ok(Value::Dict(map))
            }
            other => Err(self.type_err_at("array or dict", other.type_name(), line)),
        }
    }

    fn to_dict_key(&self, v: Value) -> WhispemResult<String> {
        let line = self.frame().current_line();
        match v {
            Value::Str(s)    => Ok(s),
            Value::Number(n) => Ok(if n.fract()==0.0 { format!("{}",n as i64) } else { format!("{}",n) }),
            other => Err(self.type_err_at("string or number (as dict key)", other.type_name(), line)),
        }
    }

    fn to_usize(&self, v: &Value, line: usize) -> WhispemResult<usize> {
        match v {
            Value::Number(n) => Ok(*n as usize),
            _ => Err(WhispemError::new(ErrorKind::InvalidIndex, line, 0)),
        }
    }

    fn to_i64(&self, v: &Value, line: usize) -> WhispemResult<i64> {
        match v {
            Value::Number(n) => Ok(*n as i64),
            other => Err(self.type_err_at("number", other.type_name(), line)),
        }
    }

    // ── Arithmetic helpers ────────────────────────────────────────────────

    fn add(&self, a: Value, b: Value) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x+y)),
            (Value::Str(x),    Value::Str(y))    => Ok(Value::Str(format!("{}{}",x,y))),
            (Value::Str(x),    other)             => Ok(Value::Str(format!("{}{}",x,other.format()))),
            (other,            Value::Str(y))     => Ok(Value::Str(format!("{}{}",other.format(),y))),
            (a, b) => Err(self.type_err("number or string", &format!("{} and {}",a.type_name(),b.type_name()))),
        }
    }

    fn numeric(&self, a: Value, b: Value, f: impl Fn(f64,f64)->f64) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(f(x,y))),
            (a, b) => Err(self.type_err("number", &format!("{} and {}",a.type_name(),b.type_name()))),
        }
    }

    fn divmod(&self, a: Value, b: Value, is_mod: bool) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => {
                if y == 0.0 { return Err(WhispemError::runtime(ErrorKind::DivisionByZero)); }
                Ok(Value::Number(if is_mod { x%y } else { x/y }))
            }
            (a, b) => Err(self.type_err("number", &format!("{} and {}",a.type_name(),b.type_name()))),
        }
    }

    fn eq(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x==y,
            (Value::Str(x),    Value::Str(y))    => x==y,
            (Value::Bool(x),   Value::Bool(y))   => x==y,
            (Value::None,      Value::None)       => true,
            _ => false,
        }
    }

    fn cmp(&self, a: Value, b: Value,
           nf: impl Fn(f64,f64)->bool,
           sf: impl Fn(&str,&str)->bool) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(nf(x,y))),
            (Value::Str(x),    Value::Str(y))    => Ok(Value::Bool(sf(&x,&y))),
            (a, b) => Err(self.type_err("number or string", &format!("{} and {}",a.type_name(),b.type_name()))),
        }
    }

    // ── Stack helpers ─────────────────────────────────────────────────────

    fn pop(&mut self) -> WhispemResult<Value> {
        self.stack.pop().ok_or_else(|| WhispemError::runtime(ErrorKind::StackUnderflow))
    }

    fn pop2(&mut self) -> WhispemResult<(Value, Value)> {
        let b = self.pop()?; let a = self.pop()?; Ok((a, b))
    }

    // ── Frame helpers ─────────────────────────────────────────────────────

    fn frame(&self)     -> &CallFrame      { self.frames.last().expect("empty call stack") }
    fn frame_mut(&mut self) -> &mut CallFrame { self.frames.last_mut().expect("empty call stack") }

    /// Read a string constant from the current frame's pool by index.
    fn const_str(&self, idx: u8) -> String {
        match self.frame().const_val(idx) {
            Value::Str(s) => s.clone(),
            _ => panic!("expected string constant at index {}", idx),
        }
    }

    // ── Error helpers ─────────────────────────────────────────────────────

    fn type_err(&self, expected: &str, found: &str) -> WhispemError {
        WhispemError::new(ErrorKind::TypeError { expected: expected.into(), found: found.into() },
                          self.frame().current_line(), 0)
    }

    fn type_err_at(&self, expected: &str, found: &str, line: usize) -> WhispemError {
        WhispemError::new(ErrorKind::TypeError { expected: expected.into(), found: found.into() }, line, 0)
    }

    fn arity(&self, name: &str, expected: usize, got: usize, line: usize) -> WhispemResult<()> {
        if got != expected {
            Err(WhispemError::new(ErrorKind::ArgumentCount { name: name.into(), expected, got }, line, 0))
        } else { Ok(()) }
    }
}