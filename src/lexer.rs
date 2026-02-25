use crate::error::{ErrorKind, WhispemError, WhispemResult};
use crate::token::{Spanned, Token};

pub struct Lexer { input: Vec<char>, position: usize, line: usize, column: usize }

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), position: 0, line: 1, column: 1 }
    }

    pub fn tokenize(&mut self) -> WhispemResult<Vec<Spanned>> {
        let mut tokens = Vec::new();
        loop {
            let s = self.next_spanned()?;
            let eof = s.token == Token::Eof;
            tokens.push(s);
            if eof { break; }
        }
        Ok(tokens)
    }

    fn cur(&self)  -> Option<char> { self.input.get(self.position).copied() }
    fn peek(&self) -> Option<char> { self.input.get(self.position + 1).copied() }

    fn advance(&mut self) {
        if let Some('\n') = self.cur() { self.line += 1; self.column = 1; }
        else { self.column += 1; }
        self.position += 1;
    }

    fn next_spanned(&mut self) -> WhispemResult<Spanned> {
        while matches!(self.cur(), Some(' ') | Some('\t') | Some('\r')) { self.advance(); }
        let line = self.line;
        let col  = self.column;
        let token = match self.cur() {
            None        => Token::Eof,
            Some('\n')  => { self.advance(); Token::Newline }
            Some('#')   => {
                while !matches!(self.cur(), Some('\n') | None) { self.advance(); }
                return self.next_spanned();
            }
            Some('(') => { self.advance(); Token::LParen }
            Some(')') => { self.advance(); Token::RParen }
            Some('{') => { self.advance(); Token::LeftBrace }
            Some('}') => { self.advance(); Token::RightBrace }
            Some('[') => { self.advance(); Token::LeftBracket }
            Some(']') => { self.advance(); Token::RightBracket }
            Some(',') => { self.advance(); Token::Comma }
            Some(':') => { self.advance(); Token::Colon }
            Some('+') => { self.advance(); Token::Plus }
            Some('*') => { self.advance(); Token::Star }
            Some('/') => { self.advance(); Token::Slash }
            Some('%') => { self.advance(); Token::Percent }
            Some('-') => { self.advance(); Token::Minus }
            Some('=') => {
                self.advance();
                if self.cur() == Some('=') { self.advance(); Token::EqualEqual } else { Token::Equals }
            }
            Some('!') => {
                self.advance();
                if self.cur() == Some('=') { self.advance(); Token::BangEqual } else { Token::Bang }
            }
            Some('<') => {
                self.advance();
                if self.cur() == Some('=') { self.advance(); Token::LessEqual } else { Token::Less }
            }
            Some('>') => {
                self.advance();
                if self.cur() == Some('=') { self.advance(); Token::GreaterEqual } else { Token::Greater }
            }
            Some('"')                              => self.read_string(line, col)?,
            Some(c) if c.is_ascii_digit()          => self.read_number(),
            Some(c) if c.is_alphabetic() || c=='_' => self.read_ident(),
            Some(c) => {
                let ch = c;
                self.advance();
                return Err(WhispemError::new(ErrorKind::UnexpectedCharacter(ch), line, col));
            }
        };
        Ok(Spanned { token, line, column: col })
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        let mut dot = false;
        while let Some(c) = self.cur() {
            if c.is_ascii_digit() {
                s.push(c); self.advance();
            } else if c == '.' && !dot && self.peek().map_or(false, |x| x.is_ascii_digit()) {
                dot = true; s.push(c); self.advance();
            } else {
                break;
            }
        }
        Token::Number(s.parse().unwrap_or(0.0))
    }

    fn read_ident(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.cur() {
            if c.is_alphanumeric() || c == '_' { s.push(c); self.advance(); } else { break; }
        }
        // Map keywords and built-in names to their tokens.
        match s.as_str() {
            "let"        => Token::Let,
            "print"      => Token::Print,
            "if"         => Token::If,
            "else"       => Token::Else,
            "while"      => Token::While,
            "for"        => Token::For,
            "in"         => Token::In,
            "and"        => Token::And,
            "or"         => Token::Or,
            "not"        => Token::Not,
            "fn"         => Token::Fn,
            "return"     => Token::Return,
            "break"      => Token::Break,
            "continue"   => Token::Continue,
            "true"       => Token::True,
            "false"      => Token::False,
            "length"     => Token::Length,
            "push"       => Token::Push,
            "pop"        => Token::Pop,
            "reverse"    => Token::Reverse,
            "slice"      => Token::Slice,
            "range"      => Token::Range,
            "input"      => Token::Input,
            "read_file"  => Token::ReadFile,
            "write_file" => Token::WriteFile,
            "keys"       => Token::Keys,
            "values"     => Token::Values,
            "has_key"    => Token::HasKey,
            _            => Token::Identifier(s),
        }
    }

    fn read_string(&mut self, line: usize, col: usize) -> WhispemResult<Token> {
        self.advance(); 
        let mut val = String::new();
        loop {
            match self.cur() {
                None | Some('\n') =>
                    return Err(WhispemError::new(ErrorKind::UnterminatedString, line, col)),
                Some('"') => { self.advance(); break; }
                Some('\\') => {
                    self.advance();
                    match self.cur() {
                        Some('n')  => { val.push('\n'); self.advance(); }
                        Some('t')  => { val.push('\t'); self.advance(); }
                        Some('r')  => { val.push('\r'); self.advance(); }
                        Some('\\') => { val.push('\\'); self.advance(); }
                        Some('"')  => { val.push('"');  self.advance(); }
                        Some(c)    => { val.push('\\'); val.push(c); self.advance(); }
                        None       => return Err(WhispemError::new(ErrorKind::UnterminatedString, line, col)),
                    }
                }
                Some(c) => { val.push(c); self.advance(); }
            }
        }
        Ok(Token::Str(val))
    }
}