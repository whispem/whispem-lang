#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,
    Identifier(String),
    Number(f64),
    String(String),
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Newline,
    EOF,
}
