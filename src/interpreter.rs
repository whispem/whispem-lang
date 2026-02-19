use crate::ast::{BinaryOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::error::{ErrorKind, WhispemError, WhispemResult};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{self, Write};

// ─────────────────────────────────────────────
// Value
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Value {
    pub fn format(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Array(elements) => {
                let parts: Vec<String> = elements.iter().map(|v| v.format()).collect();
                format!("[{}]", parts.join(", "))
            }
            Value::Dict(map) => {
                let mut parts: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.format()))
                    .collect();
                parts.sort();
                format!("{{{}}}", parts.join(", "))
            }
            Value::None => String::new(),
        }
    }

    fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Bool(_)   => "bool",
            Value::Str(_)    => "string",
            Value::Array(_)  => "array",
            Value::Dict(_)   => "dict",
            Value::None      => "none",
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b)   => *b,
            Value::Number(n) => *n != 0.0,
            Value::Str(s)    => !s.is_empty(),
            Value::Array(a)  => !a.is_empty(),
            Value::Dict(d)   => !d.is_empty(),
            Value::None      => false,
        }
    }
}

// ─────────────────────────────────────────────
// Control flow signals
// ─────────────────────────────────────────────

enum Signal {
    None,
    Return(Value),
    Break,
    Continue,
}

// ─────────────────────────────────────────────
// Function definition
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
struct FunctionDef {
    params: Vec<String>,
    body: Vec<Stmt>,
}

// ─────────────────────────────────────────────
// Call frame
// ─────────────────────────────────────────────

struct CallFrame {
    locals: HashMap<String, Value>,
}

// ─────────────────────────────────────────────
// Interpreter
// ─────────────────────────────────────────────

pub struct Interpreter {
    globals: HashMap<String, Value>,
    functions: HashMap<String, FunctionDef>,
    call_stack: Vec<CallFrame>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            functions: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    // ── public entry point ────────────────────────────────────────────────

    pub fn execute(&mut self, program: Vec<Stmt>) -> WhispemResult<()> {
        for stmt in &program {
            if let Stmt::Function { name, params, body, .. } = stmt {
                self.functions.insert(
                    name.clone(),
                    FunctionDef {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
            }
        }

        for stmt in program {
            if !matches!(stmt, Stmt::Function { .. }) {
                match self.exec_stmt(stmt)? {
                    Signal::Break    => return Err(WhispemError::runtime(ErrorKind::BreakOutsideLoop)),
                    Signal::Continue => return Err(WhispemError::runtime(ErrorKind::ContinueOutsideLoop)),
                    Signal::Return(_) | Signal::None => {}
                }
            }
        }
        Ok(())
    }

    // ── statement execution ───────────────────────────────────────────────

    fn exec_stmt(&mut self, stmt: Stmt) -> WhispemResult<Signal> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let v = self.eval(value)?;
                self.set_var(name, v);
                Ok(Signal::None)
            }

            Stmt::Print { value, .. } => {
                let v = self.eval(value)?;
                println!("{}", v.format());
                Ok(Signal::None)
            }

            Stmt::If { condition, then_branch, else_branch, .. } => {
                let cond = self.eval(condition)?;
                if cond.is_truthy() {
                    self.exec_block(then_branch)
                } else if let Some(branch) = else_branch {
                    self.exec_block(branch)
                } else {
                    Ok(Signal::None)
                }
            }

            Stmt::While { condition, body, .. } => {
                loop {
                    let cond = self.eval(condition.clone())?;
                    if !cond.is_truthy() {
                        break;
                    }
                    match self.exec_block(body.clone())? {
                        Signal::Return(v) => return Ok(Signal::Return(v)),
                        Signal::Break     => break,
                        Signal::Continue | Signal::None => {}
                    }
                }
                Ok(Signal::None)
            }

            Stmt::For { variable, iterable, body, line } => {
                let iter_val = self.eval(iterable)?;
                let items = match iter_val {
                    Value::Array(a) => a,
                    other => {
                        return Err(WhispemError::new(
                            ErrorKind::TypeError {
                                expected: "array".to_string(),
                                found: other.type_name().to_string(),
                            },
                            line,
                            0,
                        ))
                    }
                };

                for item in items {
                    self.set_var(variable.clone(), item);
                    match self.exec_block(body.clone())? {
                        Signal::Return(v) => return Ok(Signal::Return(v)),
                        Signal::Break     => break,
                        Signal::Continue | Signal::None => {}
                    }
                }
                Ok(Signal::None)
            }

            Stmt::Function { .. } => Ok(Signal::None),

            Stmt::Return { value, .. } => {
                let v = if let Some(e) = value {
                    self.eval(e)?
                } else {
                    Value::None
                };
                Ok(Signal::Return(v))
            }

            Stmt::Break { .. }    => Ok(Signal::Break),
            Stmt::Continue { .. } => Ok(Signal::Continue),

            Stmt::IndexAssign { object, index, value, line } => {
                let idx_val = self.eval(index)?;
                let new_val = self.eval(value)?;

                let stored = self
                    .get_var_mut(&object)
                    .ok_or_else(|| {
                        WhispemError::new(ErrorKind::UndefinedVariable(object.clone()), line, 0)
                    })?;

                match stored {
                    Value::Array(arr) => {
                        let idx = to_index(&idx_val, line)?;
                        let len = arr.len();
                        if idx >= len {
                            return Err(WhispemError::new(
                                ErrorKind::IndexOutOfBounds { index: idx, length: len },
                                line,
                                0,
                            ));
                        }
                        arr[idx] = new_val;
                    }
                    Value::Dict(map) => {
                        let key = dict_key(&idx_val, line)?;
                        map.insert(key, new_val);
                    }
                    _ => {
                        return Err(WhispemError::new(
                            ErrorKind::TypeError {
                                expected: "array or dict".to_string(),
                                found: stored.type_name().to_string(),
                            },
                            line,
                            0,
                        ))
                    }
                }
                Ok(Signal::None)
            }

            Stmt::Expression { expr, .. } => {
                self.eval(expr)?;
                Ok(Signal::None)
            }
        }
    }

    fn exec_block(&mut self, stmts: Vec<Stmt>) -> WhispemResult<Signal> {
        for stmt in stmts {
            match self.exec_stmt(stmt)? {
                Signal::None => {}
                sig => return Ok(sig),
            }
        }
        Ok(Signal::None)
    }

    // ── variable storage ──────────────────────────────────────────────────

    fn set_var(&mut self, name: String, value: Value) {
        if let Some(frame) = self.call_stack.last_mut() {
            frame.locals.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    fn get_var(&self, name: &str) -> Option<&Value> {
        if let Some(frame) = self.call_stack.last() {
            if let Some(v) = frame.locals.get(name) {
                return Some(v);
            }
        }
        self.globals.get(name)
    }

    fn get_var_mut(&mut self, name: &str) -> Option<&mut Value> {
        if let Some(frame) = self.call_stack.last_mut() {
            if frame.locals.contains_key(name) {
                return frame.locals.get_mut(name);
            }
        }
        self.globals.get_mut(name)
    }

    // ── expression evaluation ─────────────────────────────────────────────

    fn eval(&mut self, expr: Expr) -> WhispemResult<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::Str(s)    => Ok(Value::Str(s)),
            Expr::Bool(b)   => Ok(Value::Bool(b)),

            Expr::Array(elements) => {
                let values: WhispemResult<Vec<Value>> =
                    elements.into_iter().map(|e| self.eval(e)).collect();
                Ok(Value::Array(values?))
            }

            Expr::Dict(pairs) => {
                let mut map = HashMap::new();
                for (k_expr, v_expr) in pairs {
                    let k = self.eval(k_expr)?;
                    let key = dict_key(&k, 0)?;
                    let val = self.eval(v_expr)?;
                    map.insert(key, val);
                }
                Ok(Value::Dict(map))
            }

            Expr::Variable(name) => {
                self.get_var(&name)
                    .cloned()
                    .ok_or_else(|| WhispemError::runtime(ErrorKind::UndefinedVariable(name)))
            }

            Expr::Index { object, index } => {
                let obj = self.eval(*object)?;
                let idx = self.eval(*index)?;
                eval_index(obj, idx, 0)
            }

            Expr::Binary { left, op, right } => {
                let l = self.eval(*left)?;
                let r = self.eval(*right)?;
                eval_binary(l, op, r)
            }

            Expr::Logical { left, op, right } => {
                let l = self.eval(*left)?;
                match op {
                    LogicalOp::Or => {
                        if l.is_truthy() { return Ok(l); }
                        self.eval(*right)
                    }
                    LogicalOp::And => {
                        if !l.is_truthy() { return Ok(l); }
                        self.eval(*right)
                    }
                }
            }

            Expr::Unary { op, operand } => {
                let v = self.eval(*operand)?;
                match op {
                    UnaryOp::Not => Ok(Value::Bool(!v.is_truthy())),
                    UnaryOp::Neg => match v {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        other => Err(WhispemError::runtime(ErrorKind::TypeError {
                            expected: "number".to_string(),
                            found: other.type_name().to_string(),
                        })),
                    },
                }
            }

            Expr::Call { name, arguments, line } => {
                self.eval_call(name, arguments, line)
            }
        }
    }

    fn eval_call(&mut self, name: String, arguments: Vec<Expr>, line: usize) -> WhispemResult<Value> {
        match name.as_str() {
            "length" => {
                self.check_arity("length", 1, arguments.len(), line)?;
                let arg = self.eval(arguments.into_iter().next().unwrap())?;
                return match arg {
                    Value::Array(a) => Ok(Value::Number(a.len() as f64)),
                    Value::Str(s)   => Ok(Value::Number(s.len() as f64)),
                    Value::Dict(d)  => Ok(Value::Number(d.len() as f64)),
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "array, string, or dict".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "push" => {
                self.check_arity("push", 2, arguments.len(), line)?;
                let mut args = self.eval_args(arguments)?;
                let item = args.pop().unwrap();
                let arr  = args.pop().unwrap();
                return match arr {
                    Value::Array(mut elements) => {
                        elements.push(item);
                        Ok(Value::Array(elements))
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "array".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "pop" => {
                self.check_arity("pop", 1, arguments.len(), line)?;
                let arg = self.eval(arguments.into_iter().next().unwrap())?;
                return match arg {
                    Value::Array(mut elements) => {
                        if elements.is_empty() {
                            Err(WhispemError::new(ErrorKind::EmptyArray, line, 0))
                        } else {
                            Ok(elements.pop().unwrap())
                        }
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "array".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "reverse" => {
                self.check_arity("reverse", 1, arguments.len(), line)?;
                let arg = self.eval(arguments.into_iter().next().unwrap())?;
                return match arg {
                    Value::Array(mut elements) => {
                        elements.reverse();
                        Ok(Value::Array(elements))
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "array".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "slice" => {
                self.check_arity("slice", 3, arguments.len(), line)?;
                let mut args = self.eval_args(arguments)?;
                let end_val   = args.remove(2);
                let start_val = args.remove(1);
                let arr_val   = args.remove(0);

                let start = match start_val {
                    Value::Number(n) => n as usize,
                    other => return Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "number".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
                let end = match end_val {
                    Value::Number(n) => n as usize,
                    other => return Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "number".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };

                return match arr_val {
                    Value::Array(elements) => {
                        if start > end {
                            return Err(WhispemError::new(
                                ErrorKind::InvalidSlice { start, end },
                                line, 0,
                            ));
                        }
                        if end > elements.len() {
                            return Err(WhispemError::new(
                                ErrorKind::SliceOutOfBounds { end, length: elements.len() },
                                line, 0,
                            ));
                        }
                        Ok(Value::Array(elements[start..end].to_vec()))
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "array".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "range" => {
                self.check_arity("range", 2, arguments.len(), line)?;
                let mut args = self.eval_args(arguments)?;
                let end_val   = args.remove(1);
                let start_val = args.remove(0);
                let start = match start_val {
                    Value::Number(n) => n as i64,
                    other => return Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "number".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
                let end = match end_val {
                    Value::Number(n) => n as i64,
                    other => return Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "number".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
                let result: Vec<Value> = (start..end).map(|i| Value::Number(i as f64)).collect();
                return Ok(Value::Array(result));
            }

            "input" => {
                if arguments.len() > 1 {
                    return Err(WhispemError::new(
                        ErrorKind::ArgumentCount {
                            name: "input".to_string(),
                            expected: 1,
                            got: arguments.len(),
                        },
                        line, 0,
                    ));
                }
                let prompt = if arguments.is_empty() {
                    String::new()
                } else {
                    match self.eval(arguments.into_iter().next().unwrap())? {
                        Value::Str(s) => s,
                        other => return Err(WhispemError::new(
                            ErrorKind::TypeError {
                                expected: "string".to_string(),
                                found: other.type_name().to_string(),
                            },
                            line, 0,
                        )),
                    }
                };
                if !prompt.is_empty() {
                    print!("{}", prompt);
                    io::stdout().flush().unwrap();
                }
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                return Ok(Value::Str(
                    buf.trim_end_matches('\n')
                       .trim_end_matches('\r')
                       .to_string(),
                ));
            }

            "read_file" => {
                self.check_arity("read_file", 1, arguments.len(), line)?;
                let path = match self.eval(arguments.into_iter().next().unwrap())? {
                    Value::Str(s) => s,
                    other => return Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "string".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
                return fs::read_to_string(&path).map(Value::Str).map_err(|e| {
                    WhispemError::new(
                        ErrorKind::FileRead { path: path.clone(), reason: e.to_string() },
                        line, 0,
                    )
                });
            }

            "write_file" => {
                self.check_arity("write_file", 2, arguments.len(), line)?;
                let args = self.eval_args(arguments)?;
                let path = match &args[0] {
                    Value::Str(s) => s.clone(),
                    other => return Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "string".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
                let content = args[1].format();
                return fs::write(&path, content).map(|_| Value::None).map_err(|e| {
                    WhispemError::new(
                        ErrorKind::FileWrite { path: path.clone(), reason: e.to_string() },
                        line, 0,
                    )
                });
            }

            "keys" => {
                self.check_arity("keys", 1, arguments.len(), line)?;
                let arg = self.eval(arguments.into_iter().next().unwrap())?;
                return match arg {
                    Value::Dict(map) => {
                        let mut ks: Vec<Value> =
                            map.keys().map(|k| Value::Str(k.clone())).collect();
                        ks.sort_by(|a, b| a.format().cmp(&b.format()));
                        Ok(Value::Array(ks))
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "dict".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "values" => {
                self.check_arity("values", 1, arguments.len(), line)?;
                let arg = self.eval(arguments.into_iter().next().unwrap())?;
                return match arg {
                    Value::Dict(map) => {
                        let mut pairs: Vec<(String, Value)> = map.into_iter().collect();
                        pairs.sort_by(|(a, _), (b, _)| a.cmp(b));
                        Ok(Value::Array(pairs.into_iter().map(|(_, v)| v).collect()))
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "dict".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            "has_key" => {
                self.check_arity("has_key", 2, arguments.len(), line)?;
                let args = self.eval_args(arguments)?;
                return match &args[0] {
                    Value::Dict(map) => {
                        let key = dict_key(&args[1], line)?;
                        Ok(Value::Bool(map.contains_key(&key)))
                    }
                    other => Err(WhispemError::new(
                        ErrorKind::TypeError {
                            expected: "dict".to_string(),
                            found: other.type_name().to_string(),
                        },
                        line, 0,
                    )),
                };
            }

            _ => {}
        }

        // ── user-defined functions ────────────────────────────────────────
        let func = self
            .functions
            .get(&name)
            .cloned()
            .ok_or_else(|| {
                WhispemError::new(ErrorKind::UndefinedFunction(name.clone()), line, 0)
            })?;

        self.check_arity(&name, func.params.len(), arguments.len(), line)?;

        let arg_values = self.eval_args(arguments)?;

        let mut frame = CallFrame { locals: HashMap::new() };
        for (param, value) in func.params.iter().zip(arg_values) {
            frame.locals.insert(param.clone(), value);
        }

        self.call_stack.push(frame);

        let mut return_value = Value::None;
        for stmt in func.body {
            match self.exec_stmt(stmt)? {
                Signal::Return(v) => { return_value = v; break; }
                Signal::Break => {
                    self.call_stack.pop();
                    return Err(WhispemError::new(ErrorKind::BreakOutsideLoop, line, 0));
                }
                Signal::Continue => {
                    self.call_stack.pop();
                    return Err(WhispemError::new(ErrorKind::ContinueOutsideLoop, line, 0));
                }
                Signal::None => {}
            }
        }

        self.call_stack.pop();
        Ok(return_value)
    }

    // ── helpers ───────────────────────────────────────────────────────────

    fn eval_args(&mut self, arguments: Vec<Expr>) -> WhispemResult<Vec<Value>> {
        arguments.into_iter().map(|a| self.eval(a)).collect()
    }

    fn check_arity(&self, name: &str, expected: usize, got: usize, line: usize) -> WhispemResult<()> {
        if got != expected {
            Err(WhispemError::new(
                ErrorKind::ArgumentCount {
                    name: name.to_string(),
                    expected,
                    got,
                },
                line,
                0,
            ))
        } else {
            Ok(())
        }
    }
}

// ─────────────────────────────────────────────
// Free helpers
// ─────────────────────────────────────────────

fn eval_index(obj: Value, idx: Value, line: usize) -> WhispemResult<Value> {
    match obj {
        Value::Array(elements) => {
            let i = to_index(&idx, line)?;
            let len = elements.len();
            if i >= len {
                return Err(WhispemError::new(
                    ErrorKind::IndexOutOfBounds { index: i, length: len },
                    line, 0,
                ));
            }
            Ok(elements[i].clone())
        }
        Value::Dict(map) => {
            let key = dict_key(&idx, line)?;
            map.get(&key).cloned().ok_or_else(|| {
                WhispemError::new(
                    ErrorKind::UndefinedVariable(format!("dict key \"{}\"", key)),
                    line, 0,
                )
            })
        }
        other => Err(WhispemError::new(
            ErrorKind::TypeError {
                expected: "array or dict".to_string(),
                found: other.type_name().to_string(),
            },
            line, 0,
        )),
    }
}

fn eval_binary(left: Value, op: BinaryOp, right: Value) -> WhispemResult<Value> {
    match (&left, &op, &right) {
        (Value::Number(a), BinaryOp::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::Number(a), BinaryOp::Sub, Value::Number(b)) => Ok(Value::Number(a - b)),
        (Value::Number(a), BinaryOp::Mul, Value::Number(b)) => Ok(Value::Number(a * b)),
        (Value::Number(a), BinaryOp::Div, Value::Number(b)) => {
            if *b == 0.0 { return Err(WhispemError::runtime(ErrorKind::DivisionByZero)); }
            Ok(Value::Number(a / b))
        }
        (Value::Number(a), BinaryOp::Mod, Value::Number(b)) => {
            if *b == 0.0 { return Err(WhispemError::runtime(ErrorKind::DivisionByZero)); }
            Ok(Value::Number(a % b))
        }
        (Value::Str(a), BinaryOp::Add, Value::Str(b))    => Ok(Value::Str(format!("{}{}", a, b))),
        (Value::Str(a), BinaryOp::Add, other)             => Ok(Value::Str(format!("{}{}", a, other.format()))),
        (other,         BinaryOp::Add, Value::Str(b))     => Ok(Value::Str(format!("{}{}", other.format(), b))),
        (Value::Number(a), BinaryOp::Less,         Value::Number(b)) => Ok(Value::Bool(a < b)),
        (Value::Number(a), BinaryOp::LessEqual,    Value::Number(b)) => Ok(Value::Bool(a <= b)),
        (Value::Number(a), BinaryOp::Greater,      Value::Number(b)) => Ok(Value::Bool(a > b)),
        (Value::Number(a), BinaryOp::GreaterEqual, Value::Number(b)) => Ok(Value::Bool(a >= b)),
        (Value::Number(a), BinaryOp::EqualEqual,   Value::Number(b)) => Ok(Value::Bool(a == b)),
        (Value::Number(a), BinaryOp::BangEqual,    Value::Number(b)) => Ok(Value::Bool(a != b)),
        (Value::Str(a), BinaryOp::EqualEqual,   Value::Str(b)) => Ok(Value::Bool(a == b)),
        (Value::Str(a), BinaryOp::BangEqual,    Value::Str(b)) => Ok(Value::Bool(a != b)),
        (Value::Str(a), BinaryOp::Less,         Value::Str(b)) => Ok(Value::Bool(a < b)),
        (Value::Str(a), BinaryOp::LessEqual,    Value::Str(b)) => Ok(Value::Bool(a <= b)),
        (Value::Str(a), BinaryOp::Greater,      Value::Str(b)) => Ok(Value::Bool(a > b)),
        (Value::Str(a), BinaryOp::GreaterEqual, Value::Str(b)) => Ok(Value::Bool(a >= b)),
        (Value::Bool(a), BinaryOp::EqualEqual, Value::Bool(b)) => Ok(Value::Bool(a == b)),
        (Value::Bool(a), BinaryOp::BangEqual,  Value::Bool(b)) => Ok(Value::Bool(a != b)),
        _ => Err(WhispemError::runtime(ErrorKind::TypeError {
            expected: format!("compatible types for {:?}", op),
            found: format!("{} and {}", left.type_name(), right.type_name()),
        })),
    }
}

fn to_index(v: &Value, line: usize) -> WhispemResult<usize> {
    match v {
        Value::Number(n) => Ok(*n as usize),
        _ => Err(WhispemError::new(ErrorKind::InvalidIndex, line, 0)),
    }
}

fn dict_key(v: &Value, line: usize) -> WhispemResult<String> {
    match v {
        Value::Str(s)    => Ok(s.clone()),
        Value::Number(n) => Ok(format!("{}", if n.fract() == 0.0 { *n as i64 as f64 } else { *n })),
        other => Err(WhispemError::new(
            ErrorKind::TypeError {
                expected: "string or number (as dict key)".to_string(),
                found: other.type_name().to_string(),
            },
            line, 0,
        )),
    }
}