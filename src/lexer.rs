#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,
    Ident(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    EOF,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            return match c {
                ' ' | '\n' | '\t' => {
                    self.advance();
                    continue;
                }
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Star }
                '/' => { self.advance(); Token::Slash }
                '=' => { self.advance(); Token::Equal }
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                '0'..='9' => return self.number(),
                'a'..='z' | 'A'..='Z' | '_' => return self.ident(),
                _ => panic!("Unexpected character: {}", c),
            };
        }
        Token::EOF
    }

    fn number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(num.parse().unwrap())
    }

    fn ident(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            _ => Token::Ident(ident),
        }
    }
}
