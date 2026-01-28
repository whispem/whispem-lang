use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            stmts.push(self.statement());
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        match self.current.clone() {
            Token::Let => {
                self.advance();
                let name = if let Token::Ident(n) = self.current.clone() {
                    self.advance();
                    n
                } else {
                    panic!("Expected identifier");
                };

                self.advance(); // =
                let expr = self.expr();
                Stmt::Let { name, value: expr }
            }
            Token::Print => {
                self.advance();
                let expr = self.expr();
                Stmt::Print(expr)
            }
            _ => panic!("Unexpected token {:?}", self.current),
        }
    }

    fn expr(&mut self) -> Expr {
        let mut left = self.term();

        while matches!(self.current, Token::Plus | Token::Minus) {
            let op = match self.current {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.term();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        while matches!(self.current, Token::Star | Token::Slash) {
            let op = match self.current {
                Token::Star => Operator::Star,
                Token::Slash => Operator::Slash,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.factor();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    fn factor(&mut self) -> Expr {
        match self.current.clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::Ident(name) => {
                self.advance();
                Expr::Variable(name)
            }
            Token::LParen => {
                self.advance();
                let expr = self.expr();
                self.advance(); // )
                expr
            }
            _ => panic!("Unexpected token {:?}", self.current),
        }
    }
}
