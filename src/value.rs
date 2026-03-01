use std::collections::HashMap;
use std::fmt;

/// All runtime values in the Whispem VM.
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
            Value::Bool(b)  => b.to_string(),
            Value::Str(s)   => s.clone(),
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

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Bool(_)   => "bool",
            Value::Str(_)    => "string",
            Value::Array(_)  => "array",
            Value::Dict(_)   => "dict",
            Value::None      => "none",
        }
    }

    pub fn is_truthy(&self) -> bool {
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