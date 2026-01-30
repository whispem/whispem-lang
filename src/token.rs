#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,
    If,
    Else,

    True,
    False,

    Identifier(String),
    Number(f64),
    String(String),

    Plus,
    Minus,
    Star,
    Slash,

    Equals,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LParen,
    RParen,

    LeftBrace,
    RightBrace,

    Newline,
    EOF,
}
