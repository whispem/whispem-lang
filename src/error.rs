use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line:   usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn unknown() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn is_known(self) -> bool {
        self.line > 0
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_known() {
            write!(f, "line {}, col {}", self.line, self.column)
        } else {
            write!(f, "<unknown location>")
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhispemError {
    pub kind: ErrorKind,
    pub span: Span,
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
    FileRead  { path: String, reason: String },
    FileWrite { path: String, reason: String },
    BreakOutsideLoop,
    ContinueOutsideLoop,
    TooManyConstants,
    StackUnderflow,
    InvalidOpcode(u8),
}

impl fmt::Display for WhispemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match &self.kind {
            ErrorKind::UnexpectedCharacter(c) =>
                format!("Unexpected character: '{}'", c),
            ErrorKind::UnterminatedString =>
                "Unterminated string literal".to_string(),
            ErrorKind::UnexpectedToken { expected, found } =>
                format!("Expected {}, found {}", expected, found),
            ErrorKind::UnexpectedEof =>
                "Unexpected end of file".to_string(),
            ErrorKind::UndefinedVariable(n) =>
                format!("Undefined variable: '{}'", n),
            ErrorKind::UndefinedFunction(n) =>
                format!("Undefined function: '{}'", n),
            ErrorKind::TypeError { expected, found } =>
                format!("Type error: expected {}, found {}", expected, found),
            ErrorKind::IndexOutOfBounds { index, length } =>
                format!("Array index {} out of bounds (array length: {})", index, length),
            ErrorKind::InvalidIndex =>
                "Array index must be a number".to_string(),
            ErrorKind::DivisionByZero =>
                "Division by zero".to_string(),
            ErrorKind::ArgumentCount { name, expected, got } =>
                format!(
                    "Function '{}' expected {} argument{}, got {}",
                    name, expected,
                    if *expected == 1 { "" } else { "s" },
                    got
                ),
            ErrorKind::EmptyArray =>
                "Cannot pop from an empty array".to_string(),
            ErrorKind::SliceOutOfBounds { end, length } =>
                format!("slice() end index {} out of bounds (array length: {})", end, length),
            ErrorKind::InvalidSlice { start, end } =>
                format!("slice() start index {} cannot be greater than end index {}", start, end),
            ErrorKind::FileRead { path, reason } =>
                format!("Failed to read file '{}': {}", path, reason),
            ErrorKind::FileWrite { path, reason } =>
                format!("Failed to write file '{}': {}", path, reason),
            ErrorKind::BreakOutsideLoop =>
                "'break' used outside of a loop".to_string(),
            ErrorKind::ContinueOutsideLoop =>
                "'continue' used outside of a loop".to_string(),
            ErrorKind::TooManyConstants =>
                "Too many constants in one function (max 256). Split it into smaller functions.".to_string(),
            ErrorKind::StackUnderflow =>
                "Internal error: stack underflow (compiler bug)".to_string(),
            ErrorKind::InvalidOpcode(b) =>
                format!("Internal error: unknown opcode {:#04x}", b),
        };

        if self.span.is_known() {
            write!(f, "[{}] Error: {}", self.span, msg)
        } else {
            write!(f, "Error: {}", msg)
        }
    }
}

impl WhispemError {
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn runtime(kind: ErrorKind) -> Self {
        Self { kind, span: Span::unknown() }
    }
}

pub type WhispemResult<T> = Result<T, WhispemError>;