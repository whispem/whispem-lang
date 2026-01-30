use crate::ast::{Expr, Stmt};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn consume_newlines(&mut self) {
        while matches!(self.current(), Token::Newline) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !matches!(self.current(), Token::EOF) {
            self.consume_newlines();

            if matches!(self.current(), Token::EOF) {
                break;
            }

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }

        statements
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        match self.current() {
            Token::Let => {
                self.advance();

                let name = if let Token::Identifier(name) = self.current() {
                    name.clone()
                } else {
                    return None;
                };

                self.advance();

                if !matches!(self.current(), Token::Equals) {
                    return None;
                }
                self.advance();

                let expr = self.parse_expression();
                Some(Stmt::Let(name, expr))
            }

            Token::Print => {
                self.advance();
                let expr = self.parse_expression();
                Some(Stmt::Print(expr))
            }

            _ => None,
        }
    }

    // ─────────────────────────────────────────────
    // Expression parsing with precedence (v0.4.0)
    // ─────────────────────────────────────────────

    fn parse_expression(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = match self.current() {
                Token::Plus => '+',
                Token::Minus => '-',
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while matches!(self.current(), Token::Star | Token::Slash) {
            let op = match self.current() {
                Token::Star => '*',
                Token::Slash => '/',
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_primary();

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Token::Number(n) => {
                let value = *n;
                self.advance();
                Expr::Number(value)
            }

            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Expr::String(value)
            }

            Token::Identifier(name) => {
                let ident = name.clone();
                self.advance();
                Expr::Variable(ident)
            }

            _ => {
                self.advance();
                Expr::Number(0.0)
            }
        }
    }
}
