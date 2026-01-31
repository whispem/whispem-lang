#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Print,
    If,
    Else,

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

    // Special
    Newline,
    EOF,
}
