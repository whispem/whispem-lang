use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' => self.advance(),

                '\n' => {
                    self.advance();
                    return Token::Newline;
                }

                '#' => {
                    // Comment: skip until end of line
                    while let Some(c) = self.current_char() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }

                '=' => {
                    self.advance();
                    return Token::Equals;
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

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() || c == '.' {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::Number(number.parse().unwrap())
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
            _ => Token::Identifier(ident),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening quote
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c == '"' {
                self.advance();
                break;
            }
            value.push(c);
            self.advance();
        }

        Token::String(value)
    }
}
