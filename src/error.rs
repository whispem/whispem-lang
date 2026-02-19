use std::fmt;

#[derive(Debug, Clone)]
pub struct WhispemError {
    pub kind: ErrorKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    UnexpectedCharacter(char),
    UnterminatedString,
    UnexpectedToken { expected: String, found: String },
    UnexpectedEof,
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeError { expected: String, found: String },
    IndexOutOfBounds { index: usize, length: usize },
    InvalidIndex,
    DivisionByZero,
    ArgumentCount { name: String, expected: usize, got: usize },
    EmptyArray,
    SliceOutOfBounds { end: usize, length: usize },
    InvalidSlice { start: usize, end: usize },
    FileRead { path: String, reason: String },
    FileWrite { path: String, reason: String },
    BreakOutsideLoop,
    ContinueOutsideLoop,
}

impl fmt::Display for WhispemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match &self.kind {
            ErrorKind::UnexpectedCharacter(c) => format!("Unexpected character: '{}'", c),
            ErrorKind::UnterminatedString => "Unterminated string literal".to_string(),
            ErrorKind::UnexpectedToken { expected, found } => format!("Expected {}, found {}", expected, found),
            ErrorKind::UnexpectedEof => "Unexpected end of file".to_string(),
            ErrorKind::UndefinedVariable(name) => format!("Undefined variable: '{}'", name),
            ErrorKind::UndefinedFunction(name) => format!("Undefined function: '{}'", name),
            ErrorKind::TypeError { expected, found } => format!("Type error: expected {}, found {}", expected, found),
            ErrorKind::IndexOutOfBounds { index, length } => format!("Array index {} out of bounds (array length: {})", index, length),
            ErrorKind::InvalidIndex => "Array index must be a number".to_string(),
            ErrorKind::DivisionByZero => "Division by zero".to_string(),
            ErrorKind::ArgumentCount { name, expected, got } => format!(
                "Function '{}' expected {} argument{}, got {}",
                name, expected, if *expected == 1 { "" } else { "s" }, got
            ),
            ErrorKind::EmptyArray => "Cannot pop from an empty array".to_string(),
            ErrorKind::SliceOutOfBounds { end, length } => format!("slice() end index {} out of bounds (array length: {})", end, length),
            ErrorKind::InvalidSlice { start, end } => format!("slice() start index {} cannot be greater than end index {}", start, end),
            ErrorKind::FileRead { path, reason } => format!("Failed to read file '{}': {}", path, reason),
            ErrorKind::FileWrite { path, reason } => format!("Failed to write file '{}': {}", path, reason),
            ErrorKind::BreakOutsideLoop => "'break' used outside of a loop".to_string(),
            ErrorKind::ContinueOutsideLoop => "'continue' used outside of a loop".to_string(),
        };

        if self.line > 0 {
            write!(f, "[line {}, col {}] Error: {}", self.line, self.column, msg)
        } else {
            write!(f, "Error: {}", msg)
        }
    }
}

impl WhispemError {
    pub fn new(kind: ErrorKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }

    pub fn runtime(kind: ErrorKind) -> Self {
        Self { kind, line: 0, column: 0 }
    }
}

pub type WhispemResult<T> = Result<T, WhispemError>;