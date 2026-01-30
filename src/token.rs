#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,

    Identifier(String),
    Number(f64),
    String(String),

    Plus,
    Minus,
    Star,
    Slash,
    Equals,

    LParen,
    RParen,

    Newline,
    EOF,
}
