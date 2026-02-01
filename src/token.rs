#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Print,
    If,
    Else,
    While,
    For,
    In,
    And,
    Or,
    Not,
    Fn,
    Return,
    Break,
    Continue,
    
    // Built-in functions
    Length,
    Push,
    Pop,
    Reverse,
    Slice,
    Range,
    Input,
    ReadFile,
    WriteFile,

    // Literals
    True,
    False,
    Identifier(String),
    Number(f64),
    String(String),

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,

    // Comparison operators
    Equals,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Delimiters
    LParen,
    RParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,

    // Special
    Newline,
    EOF,
}
