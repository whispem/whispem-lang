use crate::ast::{Expr, Stmt};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !matches!(self.current(), Token::EOF) {
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
                if let Token::Identifier(name) = self.current().clone() {
                    self.advance();
                    self.advance(); // =
                    let expr = self.parse_expression();
                    Some(Stmt::Let(name, expr))
                } else {
                    None
                }
            }

            Token::Print => {
                self.advance();
                let expr = self.parse_expression();
                Some(Stmt::Print(expr))
            }

            _ => None,
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Expr {
        let mut expr = self.parse_multiplicative();

        loop {
            match self.current() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplicative();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: '+',
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplicative();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: '-',
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            match self.current() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_primary();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: '*',
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_primary();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: '/',
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }

            Token::String(s) => {
                self.advance();
                Expr::String(s)
            }

            Token::Identifier(name) => {
                self.advance();
                Expr::Variable(name)
            }

            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.advance(); // )
                expr
            }

            _ => {
                self.advance();
                Expr::Number(0.0)
            }
        }
    }
}
