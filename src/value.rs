use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::chunk::Chunk;

#[derive(Debug, Clone)]
pub struct Upvalue(pub Box<Value>);

impl Upvalue {
    pub fn new(val: Value) -> Self        { Upvalue(Box::new(val)) }
    pub fn get(&self)      -> &Value      { &self.0 }
    pub fn set(&mut self, val: Value)     { self.0 = Box::new(val); }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
    Array(Rc<Vec<Value>>),
    Dict(Rc<HashMap<String, Value>>),
    Closure {
        chunk:    Rc<Chunk>,
        upvalues: Vec<Rc<RefCell<Upvalue>>>,
    },
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a),  Value::Number(b))  => a == b,
            (Value::Bool(a),    Value::Bool(b))    => a == b,
            (Value::Str(a),     Value::Str(b))     => a == b,
            (Value::Array(a),   Value::Array(b))   => a == b,
            (Value::Dict(a),    Value::Dict(b))    => a == b,
            (Value::None,       Value::None)       => true,
            (Value::Closure { chunk: a, .. }, Value::Closure { chunk: b, .. }) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Value {
    pub fn format(&self) -> String {
        let mut s = String::new();
        let _ = self.format_to(&mut s);
        s
    }

    fn format_to(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 { write!(f, "{}", *n as i64) }
                else { write!(f, "{}", n) }
            }
            Value::Bool(b)   => write!(f, "{}", b),
            Value::Str(s)    => write!(f, "{}", s),
            Value::Array(elements) => {
                write!(f, "[")?;
                for (i, v) in elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    v.format_to(f)?;
                }
                write!(f, "]")
            }
            Value::Dict(map) => {
                write!(f, "{{")?;
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                for (i, k) in keys.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": ", k)?;
                    map.get(*k).unwrap().format_to(f)?;
                }
                write!(f, "}}")
            }
            Value::Closure { chunk, .. } => write!(f, "<fn {}>", chunk.name),
            Value::None => write!(f, "none"),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_)    => "number",
            Value::Bool(_)      => "bool",
            Value::Str(_)       => "string",
            Value::Array(_)     => "array",
            Value::Dict(_)      => "dict",
            Value::Closure {..} => "function",
            Value::None         => "none",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b)      => *b,
            Value::Number(n)    => *n != 0.0,
            Value::Str(s)       => !s.is_empty(),
            Value::Array(a)     => !a.is_empty(),
            Value::Dict(d)      => !d.is_empty(),
            Value::Closure {..} => true,
            Value::None         => false,
        }
    }
}