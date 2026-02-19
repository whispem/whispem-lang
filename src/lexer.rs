use crate::error::{ErrorKind, WhispemError, WhispemResult};
use crate::token::{Spanned, Token};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input, returning a Vec of Spanned tokens or an error.
    pub fn tokenize(&mut self) -> WhispemResult<Vec<Spanned>> {
        let mut tokens = Vec::new();
        loop {
            let spanned = self.next_spanned()?;
            let is_eof = spanned.token == Token::Eof;
            tokens.push(spanned);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn advance(&mut self) {
        if let Some('\n') = self.current_char() {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }

    fn next_spanned(&mut self) -> WhispemResult<Spanned> {
        // Skip spaces and tabs (not newlines)
        while matches!(self.current_char(), Some(' ') | Some('\t') | Some('\r')) {
            self.advance();
        }

        let line = self.line;
        let column = self.column;

        let token = match self.current_char() {
            None => Token::Eof,

            Some('\n') => {
                self.advance();
                Token::Newline
            }

            Some('#') => {
                while !matches!(self.current_char(), Some('\n') | None) {
                    self.advance();
                }
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
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equals
                }
            }

            Some('!') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }

            Some('<') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }

            Some('>') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }

            Some('"') => self.read_string(line, column)?,

            Some(c) if c.is_ascii_digit() => self.read_number(),

            Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),

            Some(c) => {
                let ch = c;
                self.advance();
                return Err(WhispemError::new(
                    ErrorKind::UnexpectedCharacter(ch),
                    line,
                    column,
                ));
            }
        };

        Ok(Spanned { token, line, column })
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else if c == '.'
                && !has_dot
                && self.peek_char().map_or(false, |ch| ch.is_ascii_digit())
            {
                has_dot = true;
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::Number(number.parse().unwrap_or(0.0))
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
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
            "true"       => Token::True,
            "false"      => Token::False,
            _            => Token::Identifier(ident),
        }
    }

    fn read_string(&mut self, line: usize, column: usize) -> WhispemResult<Token> {
        self.advance(); // skip opening quote
        let mut value = String::new();

        loop {
            match self.current_char() {
                None | Some('\n') => {
                    return Err(WhispemError::new(
                        ErrorKind::UnterminatedString,
                        line,
                        column,
                    ));
                }
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.current_char() {
                        Some('n')  => { value.push('\n'); self.advance(); }
                        Some('t')  => { value.push('\t'); self.advance(); }
                        Some('r')  => { value.push('\r'); self.advance(); }
                        Some('\\') => { value.push('\\'); self.advance(); }
                        Some('"')  => { value.push('"');  self.advance(); }
                        Some(c) => {
                            value.push('\\');
                            value.push(c);
                            self.advance();
                        }
                        None => {
                            return Err(WhispemError::new(
                                ErrorKind::UnterminatedString,
                                line,
                                column,
                            ));
                        }
                    }
                }
                Some(c) => {
                    value.push(c);
                    self.advance();
                }
            }
        }

        Ok(Token::Str(value))
    }
}