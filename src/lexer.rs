use crate::token::Token;

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

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' | '\r' => self.advance(),

                '\n' => {
                    self.advance();
                    return Token::Newline;
                }

                '#' => {
                    while let Some(c) = self.current_char() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }

                '(' => {
                    self.advance();
                    return Token::LParen;
                }

                ')' => {
                    self.advance();
                    return Token::RParen;
                }

                '{' => {
                    self.advance();
                    return Token::LeftBrace;
                }

                '}' => {
                    self.advance();
                    return Token::RightBrace;
                }

                '[' => {
                    self.advance();
                    return Token::LeftBracket;
                }

                ']' => {
                    self.advance();
                    return Token::RightBracket;
                }

                ',' => {
                    self.advance();
                    return Token::Comma;
                }

                '=' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        return Token::EqualEqual;
                    }
                    return Token::Equals;
                }

                '!' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        return Token::BangEqual;
                    }
                    return Token::Bang;
                }

                '<' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        return Token::LessEqual;
                    }
                    return Token::Less;
                }

                '>' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        return Token::GreaterEqual;
                    }
                    return Token::Greater;
                }

                '+' => {
                    self.advance();
                    return Token::Plus;
                }

                '-' => {
                    self.advance();
                    return Token::Minus;
                }

                '*' => {
                    self.advance();
                    return Token::Star;
                }

                '/' => {
                    self.advance();
                    return Token::Slash;
                }

                '"' => return self.read_string(),

                c if c.is_ascii_digit() => return self.read_number(),

                c if c.is_alphabetic() || c == '_' => return self.read_identifier(),

                _ => self.advance(),
            }
        }

        Token::EOF
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else if c == '.' && !has_dot && self.peek_char().map_or(false, |ch| ch.is_ascii_digit()) {
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
            "let" => Token::Let,
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "length" => Token::Length,
            "push" => Token::Push,
            "pop" => Token::Pop,
            "reverse" => Token::Reverse,
            "slice" => Token::Slice,
            "range" => Token::Range,
            "input" => Token::Input,
            "read_file" => Token::ReadFile,
            "write_file" => Token::WriteFile,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(ident),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening quote
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c == '"' {
                self.advance(); // skip closing quote
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => {
                            value.push('\\');
                            value.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                value.push(c);
                self.advance();
            }
        }

        Token::String(value)
    }
}
