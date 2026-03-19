use crate::chunk::Chunk;
use crate::error::{ErrorKind, Span, WhispemError, WhispemResult};
use crate::opcode::OpCode;
use crate::value::{Upvalue, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::rc::Rc;

struct CallFrame {
    chunk:    Rc<Chunk>,
    ip:       usize,
    locals:   HashMap<String, Value>,
    // Upvalues captured from an enclosing frame (for closures).
    upvalues: Vec<Rc<RefCell<Upvalue>>>,
    // Open upvalue cells for locals in *this* frame that have been captured
    // by at least one closure.  Shared with those closures via Rc.
    open_upvalues: HashMap<String, Rc<RefCell<Upvalue>>>,
}

impl CallFrame {
    fn new(chunk: Rc<Chunk>, upvalues: Vec<Rc<RefCell<Upvalue>>>) -> Self {
        Self {
            chunk,
            ip: 0,
            locals: HashMap::new(),
            upvalues,
            open_upvalues: HashMap::new(),
        }
    }

    #[inline]
    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }

    #[inline]
    fn read_u16(&mut self) -> u16 {
        let hi = self.chunk.code[self.ip]     as u16;
        let lo = self.chunk.code[self.ip + 1] as u16;
        self.ip += 2;
        (hi << 8) | lo
    }

    #[inline]
    fn const_val(&self, idx: u8) -> &Value {
        &self.chunk.constants[idx as usize]
    }

    fn current_line(&self) -> usize {
        self.chunk.lines.get(self.ip.saturating_sub(1)).copied().unwrap_or(0)
    }
}

pub struct Vm {
    stack:           Vec<Value>,
    frames:          Vec<CallFrame>,
    globals:         HashMap<String, Value>,
    pub functions:   HashMap<String, Chunk>,
    pub script_args: Vec<String>,
    output:          Box<dyn Write + Send>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack:       Vec::with_capacity(256),
            frames:      Vec::with_capacity(64),
            globals:     HashMap::new(),
            functions:   HashMap::new(),
            script_args: Vec::new(),
            output:      Box::new(io::stdout()),
        }
    }

    #[cfg(test)]
    pub fn capturing(buf: std::sync::Arc<std::sync::Mutex<Vec<u8>>>) -> Self {
        struct ArcWriter(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);
        impl Write for ArcWriter {
            fn write(&mut self, data: &[u8]) -> io::Result<usize> {
                self.0.lock().unwrap().extend_from_slice(data);
                Ok(data.len())
            }
            fn flush(&mut self) -> io::Result<()> { Ok(()) }
        }
        unsafe impl Send for ArcWriter {}

        Self {
            stack:       Vec::with_capacity(256),
            frames:      Vec::with_capacity(64),
            globals:     HashMap::new(),
            functions:   HashMap::new(),
            script_args: Vec::new(),
            output:      Box::new(ArcWriter(buf)),
        }
    }

    pub fn run(&mut self, main_chunk: Chunk) -> WhispemResult<()> {
        let chunk = Rc::new(main_chunk);
        self.frames.push(CallFrame::new(chunk, vec![]));
        self.execute()
    }

    fn execute(&mut self) -> WhispemResult<()> {
        loop {
            let byte = self.frame_mut().read_byte();
            let op = OpCode::from_byte(byte).ok_or_else(|| {
                WhispemError::new(
                    ErrorKind::InvalidOpcode(byte),
                    Span::new(self.frame().current_line(), 0),
                )
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
                    let val  = self.lookup_local(&name).ok_or_else(|| {
                        WhispemError::new(
                            ErrorKind::UndefinedVariable(name.clone()),
                            Span::new(self.frame().current_line(), 0),
                        )
                    })?;
                    self.stack.push(val);
                }

                OpCode::LoadGlobal => {
                    let idx  = self.frame_mut().read_byte();
                    let name = self.const_str(idx);
                    let val  = self.globals.get(&name).cloned().ok_or_else(|| {
                        WhispemError::new(
                            ErrorKind::UndefinedVariable(name.clone()),
                            Span::new(self.frame().current_line(), 0),
                        )
                    })?;
                    self.stack.push(val);
                }

                OpCode::Store => {
                    let idx  = self.frame_mut().read_byte();
                    let name = self.const_str(idx);
                    let val  = self.pop()?;
                    self.store(name, val);
                }

                OpCode::LoadUpvalue => {
                    let slot = self.frame_mut().read_byte() as usize;
                    let val  = self.load_upvalue(slot)?;
                    self.stack.push(val);
                }

                OpCode::StoreUpvalue => {
                    let slot = self.frame_mut().read_byte() as usize;
                    let val  = self.pop()?;
                    self.store_upvalue(slot, val)?;
                }

                OpCode::CloseUpvalue => {
                    let _slot = self.frame_mut().read_byte();
                }

                OpCode::Add => {
                    let (a, b) = self.pop2()?;
                    let r = self.add(a, b)?;
                    self.stack.push(r);
                }
                OpCode::Sub => {
                    let (a, b) = self.pop2()?;
                    let r = self.numeric(a, b, |x, y| x - y)?;
                    self.stack.push(r);
                }
                OpCode::Mul => {
                    let (a, b) = self.pop2()?;
                    let r = self.numeric(a, b, |x, y| x * y)?;
                    self.stack.push(r);
                }
                OpCode::Div => {
                    let (a, b) = self.pop2()?;
                    let r = self.divmod(a, b, false)?;
                    self.stack.push(r);
                }
                OpCode::Mod => {
                    let (a, b) = self.pop2()?;
                    let r = self.divmod(a, b, true)?;
                    self.stack.push(r);
                }
                OpCode::Neg => {
                    let a = self.pop()?;
                    match a {
                        Value::Number(n) => self.stack.push(Value::Number(-n)),
                        other => return Err(self.type_err("number", other.type_name())),
                    }
                }

                OpCode::Eq => {
                    let (a, b) = self.pop2()?;
                    self.stack.push(Value::Bool(self.eq_val(&a, &b)));
                }
                OpCode::Neq => {
                    let (a, b) = self.pop2()?;
                    self.stack.push(Value::Bool(!self.eq_val(&a, &b)));
                }
                OpCode::Lt => {
                    let (a, b) = self.pop2()?;
                    let r = self.cmp(a, b, |x, y| x < y, |x, y| x < y)?;
                    self.stack.push(r);
                }
                OpCode::Lte => {
                    let (a, b) = self.pop2()?;
                    let r = self.cmp(a, b, |x, y| x <= y, |x, y| x <= y)?;
                    self.stack.push(r);
                }
                OpCode::Gt => {
                    let (a, b) = self.pop2()?;
                    let r = self.cmp(a, b, |x, y| x > y, |x, y| x > y)?;
                    self.stack.push(r);
                }
                OpCode::Gte => {
                    let (a, b) = self.pop2()?;
                    let r = self.cmp(a, b, |x, y| x >= y, |x, y| x >= y)?;
                    self.stack.push(r);
                }
                OpCode::Not => {
                    let a = self.pop()?;
                    self.stack.push(Value::Bool(!a.is_truthy()));
                }

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
                OpCode::PeekJumpIfFalse => {
                    let target = self.frame_mut().read_u16() as usize;
                    let cond   = self.stack.last()
                        .ok_or_else(|| WhispemError::runtime(ErrorKind::StackUnderflow))?;
                    if !cond.is_truthy() { self.frame_mut().ip = target; }
                }
                OpCode::PeekJumpIfTrue => {
                    let target = self.frame_mut().read_u16() as usize;
                    let cond   = self.stack.last()
                        .ok_or_else(|| WhispemError::runtime(ErrorKind::StackUnderflow))?;
                    if cond.is_truthy() { self.frame_mut().ip = target; }
                }

                OpCode::MakeClosure => {
                    let name_idx  = self.frame_mut().read_byte();
                    let uv_count  = self.frame_mut().read_byte() as usize;
                    let fn_name   = self.const_str(name_idx);

                    // Read upvalue descriptors: each is (is_local: u8, name_len: u8, name_bytes...).
                    let mut uv_descs: Vec<(bool, String)> = Vec::with_capacity(uv_count);
                    for _ in 0..uv_count {
                        let is_local  = self.frame_mut().read_byte() != 0;
                        let name_len  = self.frame_mut().read_byte() as usize;
                        let mut name_bytes = Vec::with_capacity(name_len);
                        for _ in 0..name_len {
                            name_bytes.push(self.frame_mut().read_byte());
                        }
                        let name = String::from_utf8(name_bytes).unwrap_or_default();
                        uv_descs.push((is_local, name));
                    }

                    let proto = self.functions.get(&fn_name).cloned().ok_or_else(|| {
                        WhispemError::new(
                            ErrorKind::UndefinedFunction(fn_name.clone()),
                            Span::new(self.frame().current_line(), 0),
                        )
                    })?;

                    let mut upvalues: Vec<Rc<RefCell<Upvalue>>> = Vec::with_capacity(uv_count);
                    for (is_local, name) in uv_descs {
                        if is_local {
                            let cell = if let Some(existing) =
                                self.frames.last().and_then(|f| f.open_upvalues.get(&name)).cloned()
                            {
                                existing
                            } else {
                                let val = self.lookup_local(&name).unwrap_or(Value::None);
                                let cell = Rc::new(RefCell::new(Upvalue::new(val)));
                                if let Some(frame) = self.frames.last_mut() {
                                    frame.open_upvalues.insert(name, cell.clone());
                                }
                                cell
                            };
                            upvalues.push(cell);
                        } else {
                            let slot: usize = name.parse().unwrap_or(0);
                            let uv = self.frame().upvalues.get(slot)
                                .cloned()
                                .ok_or_else(|| WhispemError::runtime(
                                    ErrorKind::UpvalueError(
                                        format!("parent upvalue slot {} out of range", slot)
                                    )
                                ))?;
                            upvalues.push(uv);
                        }
                    }

                    self.stack.push(Value::Closure {
                        chunk:    Rc::new(proto),
                        upvalues,
                    });
                }

                OpCode::Call => {
                    let name_idx = self.frame_mut().read_byte();
                    let argc     = self.frame_mut().read_byte() as usize;
                    let name     = self.const_str(name_idx);

                    let mut args: Vec<Value> = (0..argc)
                        .map(|_| self.pop())
                        .collect::<WhispemResult<_>>()?;
                    args.reverse();

                    if name == "__callee__" {
                        let callee = self.pop()?;
                        self.call_value(callee, args, argc)?;
                        continue;
                    }

                    if let Some(result) = self.call_builtin(&name, &args)? {
                        self.stack.push(result);
                        continue;
                    }

                    if let Some(closure_val) = self.lookup_local(&name) {
                        if matches!(closure_val, Value::Closure { .. }) {
                            self.call_value(closure_val, args, argc)?;
                            continue;
                        }
                    }

                    let chunk = self.functions.get(&name).cloned().ok_or_else(|| {
                        WhispemError::new(
                            ErrorKind::UndefinedFunction(name.clone()),
                            Span::new(self.frame().current_line(), 0),
                        )
                    })?;

                    if argc != chunk.param_count {
                        return Err(WhispemError::new(
                            ErrorKind::ArgumentCount {
                                name:     name.clone(),
                                expected: chunk.param_count,
                                got:      argc,
                            },
                            Span::new(self.frame().current_line(), 0),
                        ));
                    }

                    let new_frame = CallFrame::new(Rc::new(chunk), vec![]);
                    for arg in args { self.stack.push(arg); }
                    self.frames.push(new_frame);
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
                    let mut elems: Vec<Value> = (0..n)
                        .map(|_| self.pop())
                        .collect::<WhispemResult<_>>()?;
                    elems.reverse();
                    self.stack.push(Value::Array(Rc::new(elems)));
                }
                OpCode::MakeDict => {
                    let n = self.frame_mut().read_byte() as usize;
                    let mut pairs: Vec<(String, Value)> = Vec::with_capacity(n);
                    for _ in 0..n {
                        let val = self.pop()?;
                        let key = self.pop()?;
                        pairs.push((self.to_dict_key(key)?, val));
                    }
                    pairs.reverse();
                    let map: HashMap<String, Value> = pairs.into_iter().collect();
                    self.stack.push(Value::Dict(Rc::new(map)));
                }
                OpCode::GetIndex => {
                    let idx = self.pop()?;
                    let obj = self.pop()?;
                    let v   = self.get_index(obj, idx)?;
                    self.stack.push(v);
                }
                OpCode::SetIndex => {
                    let new_val = self.pop()?;
                    let idx     = self.pop()?;
                    let obj     = self.pop()?;
                    let mutated = self.set_index(obj, idx, new_val)?;
                    self.stack.push(mutated);
                }

                OpCode::Print => {
                    let val  = self.pop()?;
                    let line = format!("{}\n", val.format());
                    let _    = self.output.write_all(line.as_bytes());
                }
                OpCode::Pop  => { self.pop()?; }
                OpCode::Halt => {
                    self.frames.pop();
                    return Ok(());
                }
            }
        }
    }

    fn call_value(&mut self, callee: Value, args: Vec<Value>, argc: usize) -> WhispemResult<()> {
        match callee {
            Value::Closure { chunk, upvalues } => {
                if argc != chunk.param_count {
                    return Err(WhispemError::new(
                        ErrorKind::ArgumentCount {
                            name:     chunk.name.clone(),
                            expected: chunk.param_count,
                            got:      argc,
                        },
                        Span::new(self.frame().current_line(), 0),
                    ));
                }
                let new_frame = CallFrame::new(chunk, upvalues);
                for arg in args { self.stack.push(arg); }
                self.frames.push(new_frame);
                Ok(())
            }
            other => Err(WhispemError::new(
                ErrorKind::TypeError {
                    expected: "function".to_string(),
                    found:    other.type_name().to_string(),
                },
                Span::new(self.frame().current_line(), 0),
            )),
        }
    }


    fn load_upvalue(&self, slot: usize) -> WhispemResult<Value> {
        let uv = self.frame().upvalues.get(slot).ok_or_else(|| {
            WhispemError::runtime(ErrorKind::UpvalueError(
                format!("upvalue slot {} out of range", slot),
            ))
        })?;
        Ok(uv.borrow().get().clone())
    }

    fn store_upvalue(&mut self, slot: usize, val: Value) -> WhispemResult<()> {
        let uv = self.frame().upvalues.get(slot).cloned().ok_or_else(|| {
            WhispemError::runtime(ErrorKind::UpvalueError(
                format!("upvalue slot {} out of range", slot),
            ))
        })?;
        uv.borrow_mut().set(val);
        Ok(())
    }


    fn call_builtin(&mut self, name: &str, args: &[Value]) -> WhispemResult<Option<Value>> {
        let line = self.frame().current_line();

        let result = match name {
            "length" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Array(a) => Value::Number(a.len() as f64),
                    Value::Str(s)   => Value::Number(s.chars().count() as f64),
                    Value::Dict(d)  => Value::Number(d.len() as f64),
                    other => return Err(self.type_err_at(
                        "array, string, or dict", other.type_name(), line,
                    )),
                }
            }
            "push" => {
                self.arity(name, 2, args.len(), line)?;
                match args[0].clone() {
                    Value::Array(mut a) => {
                        Rc::make_mut(&mut a).push(args[1].clone());
                        Value::Array(a)
                    }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "pop" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Array(mut a) => {
                        if a.is_empty() {
                            return Err(WhispemError::new(ErrorKind::EmptyArray, Span::new(line, 0)));
                        }
                        Rc::make_mut(&mut a).pop().unwrap()
                    }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "reverse" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Array(mut a) => {
                        Rc::make_mut(&mut a).reverse();
                        Value::Array(a)
                    }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "slice" => {
                self.arity(name, 3, args.len(), line)?;
                let start = self.to_usize(&args[1], line)?;
                let end   = self.to_usize(&args[2], line)?;
                match &args[0] {
                    Value::Array(a) => {
                        if start > end {
                            return Err(WhispemError::new(
                                ErrorKind::InvalidSlice { start, end }, Span::new(line, 0),
                            ));
                        }
                        if end > a.len() {
                            return Err(WhispemError::new(
                                ErrorKind::SliceOutOfBounds { end, length: a.len() },
                                Span::new(line, 0),
                            ));
                        }
                        Value::Array(Rc::new(a[start..end].to_vec()))
                    }
                    other => return Err(self.type_err_at("array", other.type_name(), line)),
                }
            }
            "range" => {
                self.arity(name, 2, args.len(), line)?;
                let start = self.to_i64(&args[0], line)?;
                let end   = self.to_i64(&args[1], line)?;
                Value::Array(Rc::new(
                    (start..end).map(|i| Value::Number(i as f64)).collect(),
                ))
            }
            "input" => {
                if args.len() > 1 {
                    return Err(WhispemError::new(
                        ErrorKind::ArgumentCount { name: "input".into(), expected: 1, got: args.len() },
                        Span::new(line, 0),
                    ));
                }
                let prompt = if args.is_empty() {
                    String::new()
                } else {
                    match &args[0] {
                        Value::Str(s) => s.clone(),
                        other => return Err(self.type_err_at("string", other.type_name(), line)),
                    }
                };
                if !prompt.is_empty() {
                    print!("{}", prompt);
                    io::stdout().flush().unwrap();
                }
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                Value::Str(buf.trim_end_matches('\n').trim_end_matches('\r').to_string())
            }
            "read_file" => {
                self.arity(name, 1, args.len(), line)?;
                let path = match &args[0] {
                    Value::Str(s) => s.clone(),
                    other => return Err(self.type_err_at("string", other.type_name(), line)),
                };
                fs::read_to_string(&path)
                    .map(Value::Str)
                    .map_err(|e| WhispemError::new(
                        ErrorKind::FileRead { path: path.clone(), reason: e.to_string() },
                        Span::new(line, 0),
                    ))?
            }
            "write_file" => {
                self.arity(name, 2, args.len(), line)?;
                let path = match &args[0] {
                    Value::Str(s) => s.clone(),
                    other => return Err(self.type_err_at("string", other.type_name(), line)),
                };
                let content = args[1].format();
                fs::write(&path, content)
                    .map(|_| Value::None)
                    .map_err(|e| WhispemError::new(
                        ErrorKind::FileWrite { path: path.clone(), reason: e.to_string() },
                        Span::new(line, 0),
                    ))?
            }
            "keys" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Dict(map) => {
                        let mut ks: Vec<Value> =
                            map.keys().map(|k| Value::Str(k.clone())).collect();
                        ks.sort_by(|a, b| a.format().cmp(&b.format()));
                        Value::Array(Rc::new(ks))
                    }
                    other => return Err(self.type_err_at("dict", other.type_name(), line)),
                }
            }
            "values" => {
                self.arity(name, 1, args.len(), line)?;
                match args[0].clone() {
                    Value::Dict(map) => {
                        let inner = Rc::try_unwrap(map).unwrap_or_else(|rc| (*rc).clone());
                        let mut pairs: Vec<(String, Value)> = inner.into_iter().collect();
                        pairs.sort_by(|(a, _), (b, _)| a.cmp(b));
                        Value::Array(Rc::new(pairs.into_iter().map(|(_, v)| v).collect()))
                    }
                    other => return Err(self.type_err_at("dict", other.type_name(), line)),
                }
            }
            "has_key" => {
                self.arity(name, 2, args.len(), line)?;
                match &args[0] {
                    Value::Dict(map) => {
                        let k = self.to_dict_key(args[1].clone())?;
                        Value::Bool(map.contains_key(&k))
                    }
                    other => return Err(self.type_err_at("dict", other.type_name(), line)),
                }
            }
            "char_at" => {
                self.arity(name, 2, args.len(), line)?;
                match (&args[0], &args[1]) {
                    (Value::Str(s), Value::Number(n)) => {
                        let i  = *n as usize;
                        let ch = s.chars().nth(i).ok_or_else(|| WhispemError::new(
                            ErrorKind::IndexOutOfBounds { index: i, length: s.chars().count() },
                            Span::new(line, 0),
                        ))?;
                        Value::Str(ch.to_string())
                    }
                    _ => return Err(self.type_err_at("string, number", "wrong types", line)),
                }
            }
            "substr" => {
                self.arity(name, 3, args.len(), line)?;
                match (&args[0], &args[1], &args[2]) {
                    (Value::Str(s), Value::Number(start), Value::Number(len)) => {
                        let st    = *start as usize;
                        let ln    = *len as usize;
                        let chars: Vec<char> = s.chars().collect();
                        let end   = (st + ln).min(chars.len());
                        let result: String = chars[st.min(chars.len())..end].iter().collect();
                        Value::Str(result)
                    }
                    _ => return Err(self.type_err_at("string, number, number", "wrong types", line)),
                }
            }
            "ord" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Str(s) => {
                        let ch = s.chars().next().ok_or_else(|| WhispemError::new(
                            ErrorKind::TypeError {
                                expected: "non-empty string".into(),
                                found:    "empty string".into(),
                            },
                            Span::new(line, 0),
                        ))?;
                        Value::Number(ch as u32 as f64)
                    }
                    other => return Err(self.type_err_at("string", other.type_name(), line)),
                }
            }
            "num_to_str" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Number(n) => Value::Str(
                        if n.fract() == 0.0 { format!("{}", *n as i64) }
                        else { format!("{}", n) }
                    ),
                    other => return Err(self.type_err_at("number", other.type_name(), line)),
                }
            }
            "str_to_num" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Str(s) => {
                        let n = s.trim().parse::<f64>().map_err(|_| WhispemError::new(
                            ErrorKind::TypeError {
                                expected: "numeric string".into(),
                                found:    format!("\"{}\"", s),
                            },
                            Span::new(line, 0),
                        ))?;
                        Value::Number(n)
                    }
                    other => return Err(self.type_err_at("string", other.type_name(), line)),
                }
            }
            "args" => {
                self.arity(name, 0, args.len(), line)?;
                Value::Array(Rc::new(
                    self.script_args.iter().map(|s| Value::Str(s.clone())).collect()
                ))
            }
            "num_to_hex" => {
                self.arity(name, 1, args.len(), line)?;
                match &args[0] {
                    Value::Number(n) => Value::Str(format!("{:016x}", n.to_bits())),
                    other => return Err(self.type_err_at("number", other.type_name(), line)),
                }
            }
            "write_hex" => {
                self.arity(name, 2, args.len(), line)?;
                let path = match &args[0] {
                    Value::Str(s) => s.clone(),
                    other => return Err(self.type_err_at("string", other.type_name(), line)),
                };
                let hex = match &args[1] {
                    Value::Str(s) => s.clone(),
                    other => return Err(self.type_err_at("string", other.type_name(), line)),
                };
                let bytes: Vec<u8> = (0..hex.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap_or(0))
                    .collect();
                fs::write(&path, &bytes).map_err(|e| WhispemError::new(
                    ErrorKind::FileWrite { path: path.clone(), reason: e.to_string() },
                    Span::new(line, 0),
                ))?;
                Value::None
            }
            "assert" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(WhispemError::new(
                        ErrorKind::ArgumentCount { name: "assert".into(), expected: 2, got: args.len() },
                        Span::new(line, 0),
                    ));
                }
                if !args[0].is_truthy() {
                    let msg = if args.len() == 2 {
                        args[1].format()
                    } else {
                        "assertion failed".to_string()
                    };
                    return Err(WhispemError::new(
                        ErrorKind::AssertionFailed(msg),
                        Span::new(line, 0),
                    ));
                }
                Value::None
            }
            "type_of" => {
                self.arity(name, 1, args.len(), line)?;
                Value::Str(args[0].type_name().to_string())
            }
            "exit" => {
                if args.len() > 1 {
                    return Err(WhispemError::new(
                        ErrorKind::ArgumentCount { name: "exit".into(), expected: 1, got: args.len() },
                        Span::new(line, 0),
                    ));
                }
                let code = if args.is_empty() {
                    0i64
                } else {
                    self.to_i64(&args[0], line)?
                };
                return Err(WhispemError::new(ErrorKind::Exit(code), Span::unknown()));
            }

            _ => return Ok(None),
        };
        Ok(Some(result))
    }


    fn lookup_local(&self, name: &str) -> Option<Value> {
        self.frames.last()
            .and_then(|f| f.locals.get(name).cloned())
            .or_else(|| self.globals.get(name).cloned())
    }

    fn store(&mut self, name: String, value: Value) {
        // If a closure has captured this local, update the shared cell so the
        // closure sees the new value.
        if let Some(cell) = self.frames.last().and_then(|f| f.open_upvalues.get(&name)).cloned() {
            cell.borrow_mut().set(value.clone());
        }
        if self.frames.len() > 1 {
            if let Some(frame) = self.frames.last_mut() {
                frame.locals.insert(name, value);
                return;
            }
        }
        self.globals.insert(name, value);
    }


    fn get_index(&self, obj: Value, idx: Value) -> WhispemResult<Value> {
        let line = self.frame().current_line();
        match obj {
            Value::Array(a) => {
                let i = self.to_usize(&idx, line)?;
                if i >= a.len() {
                    return Err(WhispemError::new(
                        ErrorKind::IndexOutOfBounds { index: i, length: a.len() },
                        Span::new(line, 0),
                    ));
                }
                Ok(a[i].clone())
            }
            Value::Dict(map) => {
                let key = self.to_dict_key(idx)?;
                map.get(&key).cloned().ok_or_else(|| {
                    WhispemError::new(
                        ErrorKind::UndefinedVariable(format!("key \"{}\" not found in dict", key)),
                        Span::new(line, 0),
                    )
                })
            }
            other => Err(self.type_err_at("array or dict", other.type_name(), line)),
        }
    }

    fn set_index(&self, obj: Value, idx: Value, new_val: Value) -> WhispemResult<Value> {
        let line = self.frame().current_line();
        match obj {
            Value::Array(mut a) => {
                let i = self.to_usize(&idx, line)?;
                if i >= a.len() {
                    return Err(WhispemError::new(
                        ErrorKind::IndexOutOfBounds { index: i, length: a.len() },
                        Span::new(line, 0),
                    ));
                }
                Rc::make_mut(&mut a)[i] = new_val;
                Ok(Value::Array(a))
            }
            Value::Dict(mut map) => {
                let key = self.to_dict_key(idx)?;
                Rc::make_mut(&mut map).insert(key, new_val);
                Ok(Value::Dict(map))
            }
            other => Err(self.type_err_at("array or dict", other.type_name(), line)),
        }
    }

    fn to_dict_key(&self, v: Value) -> WhispemResult<String> {
        let line = self.frame().current_line();
        match v {
            Value::Str(s)    => Ok(s),
            Value::Number(n) => Ok(if n.fract() == 0.0 {
                format!("{}", n as i64)
            } else {
                format!("{}", n)
            }),
            other => Err(self.type_err_at(
                "string or number (as dict key)", other.type_name(), line,
            )),
        }
    }

    fn to_usize(&self, v: &Value, line: usize) -> WhispemResult<usize> {
        match v {
            Value::Number(n) => Ok(*n as usize),
            _ => Err(WhispemError::new(ErrorKind::InvalidIndex, Span::new(line, 0))),
        }
    }

    fn to_i64(&self, v: &Value, line: usize) -> WhispemResult<i64> {
        match v {
            Value::Number(n) => Ok(*n as i64),
            other => Err(self.type_err_at("number", other.type_name(), line)),
        }
    }


    fn add(&self, a: Value, b: Value) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
            (Value::Str(x),    Value::Str(y))    => Ok(Value::Str(format!("{}{}", x, y))),
            (Value::Str(x),    other)             => Ok(Value::Str(format!("{}{}", x, other.format()))),
            (other,            Value::Str(y))     => Ok(Value::Str(format!("{}{}", other.format(), y))),
            (a, b) => Err(self.type_err(
                "number or string",
                &format!("{} and {}", a.type_name(), b.type_name()),
            )),
        }
    }

    fn numeric(&self, a: Value, b: Value, f: impl Fn(f64, f64) -> f64) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(f(x, y))),
            (a, b) => Err(self.type_err(
                "number",
                &format!("{} and {}", a.type_name(), b.type_name()),
            )),
        }
    }

    fn divmod(&self, a: Value, b: Value, is_mod: bool) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => {
                if y == 0.0 {
                    return Err(WhispemError::runtime(ErrorKind::DivisionByZero));
                }
                Ok(Value::Number(if is_mod { x % y } else { x / y }))
            }
            (a, b) => Err(self.type_err(
                "number",
                &format!("{} and {}", a.type_name(), b.type_name()),
            )),
        }
    }

    fn eq_val(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Str(x),    Value::Str(y))    => x == y,
            (Value::Bool(x),   Value::Bool(y))   => x == y,
            (Value::None,      Value::None)       => true,
            _                                    => false,
        }
    }

    fn cmp(
        &self,
        a:  Value,
        b:  Value,
        nf: impl Fn(f64, f64) -> bool,
        sf: impl Fn(&str, &str) -> bool,
    ) -> WhispemResult<Value> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(nf(x, y))),
            (Value::Str(x),    Value::Str(y))    => Ok(Value::Bool(sf(&x, &y))),
            (a, b) => Err(self.type_err(
                "number or string",
                &format!("{} and {}", a.type_name(), b.type_name()),
            )),
        }
    }


    fn pop(&mut self) -> WhispemResult<Value> {
        self.stack.pop().ok_or_else(|| WhispemError::runtime(ErrorKind::StackUnderflow))
    }

    fn pop2(&mut self) -> WhispemResult<(Value, Value)> {
        let b = self.pop()?;
        let a = self.pop()?;
        Ok((a, b))
    }

    fn frame(&self) -> &CallFrame {
        self.frames.last().expect("empty call stack")
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().expect("empty call stack")
    }

    fn const_str(&self, idx: u8) -> String {
        match self.frame().const_val(idx) {
            Value::Str(s) => s.clone(),
            _ => panic!("expected string constant at index {}", idx),
        }
    }

    fn type_err(&self, expected: &str, found: &str) -> WhispemError {
        WhispemError::new(
            ErrorKind::TypeError { expected: expected.into(), found: found.into() },
            Span::new(self.frame().current_line(), 0),
        )
    }

    fn type_err_at(&self, expected: &str, found: &str, line: usize) -> WhispemError {
        WhispemError::new(
            ErrorKind::TypeError { expected: expected.into(), found: found.into() },
            Span::new(line, 0),
        )
    }

    fn arity(&self, name: &str, expected: usize, got: usize, line: usize) -> WhispemResult<()> {
        if got != expected {
            Err(WhispemError::new(
                ErrorKind::ArgumentCount { name: name.into(), expected, got },
                Span::new(line, 0),
            ))
        } else {
            Ok(())
        }
    }
}