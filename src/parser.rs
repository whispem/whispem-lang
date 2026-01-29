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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !matches!(self.current(), Token::EOF) {
            match self.current() {
                Token::Let => statements.push(self.parse_let()),
                Token::Print => statements.push(self.parse_print()),
                Token::Newline => self.advance(),
                _ => self.advance(),
            }
        }

        statements
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance(); // let

        let name = if let Token::Identifier(name) = self.current() {
            name.clone()
        } else {
            panic!("Expected identifier");
        };

        self.advance(); 
        self.advance(); 

        let expr = self.parse_expr();
        Stmt::Let(name, expr)
    }

    fn parse_print(&mut self) -> Stmt {
        self.advance(); // print
        let expr = self.parse_expr();
        Stmt::Print(expr)
    }

    fn parse_expr(&mut self) -> Expr {
        match self.current() {
            Token::Number(n) => {
                let v = *n;
                self.advance();
                Expr::Number(v)
            }
            Token::String(s) => {
                let v = s.clone();
                self.advance();
                Expr::String(v)
            }
            Token::Identifier(name) => {
                let v = name.clone();
                self.advance();
                Expr::Variable(v)
            }
            _ => panic!("Unexpected token"),
        }
    }
}
