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
        self.tokens.get(self.position).unwrap()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn consume(&mut self, token: Token) {
        if *self.current() == token {
            self.advance();
        } else {
            panic!("Unexpected token: {:?}", self.current());
        }
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while *self.current() != Token::EOF {
            if *self.current() == Token::Newline {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current() {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            _ => panic!("Unexpected statement: {:?}", self.current()),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance();
        let name = if let Token::Identifier(name) = self.current() {
            name.clone()
        } else {
            panic!("Expected identifier");
        };
        self.advance();
        self.consume(Token::Equals);
        let expr = self.parse_expression();
        Stmt::Let(name, expr)
    }

    fn parse_print(&mut self) -> Stmt {
        self.advance();
        let expr = self.parse_expression();
        Stmt::Print(expr)
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance();
        let condition = self.parse_expression();
        let then_branch = self.parse_block();

        let else_branch = if *self.current() == Token::Else {
            self.advance();
            Some(self.parse_block())
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(Token::LeftBrace);
        let mut stmts = Vec::new();

        while *self.current() != Token::RightBrace {
            if *self.current() == Token::Newline {
                self.advance();
                continue;
            }
            stmts.push(self.parse_statement());
        }

        self.consume(Token::RightBrace);
        stmts
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while matches!(
            self.current(),
            Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
                | Token::EqualEqual
                | Token::BangEqual
        ) {
            let op = format!("{:?}", self.current());
            self.advance();
            let right = self.parse_term();
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
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
        Token::True => {
            self.advance();
            Expr::Bool(true)
        }
        Token::False => {
            self.advance();
            Expr::Bool(false)
        }
        Token::Identifier(name) => {
            let v = name.clone();
            self.advance();
            Expr::Variable(v)
        }
        Token::LParen => {
            self.advance(); 
            let expr = self.parse_expression();
            self.consume(Token::RParen); 
            expr
        }
        _ => panic!("Unexpected expression: {:?}", self.current()),
    }
}
