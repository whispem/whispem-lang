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

    fn consume(&mut self, expected: Token) {
        if *self.current() == expected {
            self.advance();
        } else {
            panic!(
                "Expected {:?}, found {:?}",
                expected,
                self.current()
            );
        }
    }

    // =========================
    // Entry point
    // =========================

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

    // =========================
    // Statements
    // =========================

    fn parse_statement(&mut self) -> Stmt {
        match self.current() {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            _ => panic!("Unexpected statement: {:?}", self.current()),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance(); // consume 'let'

        let name = if let Token::Identifier(name) = self.current() {
            name.clone()
        } else {
            panic!("Expected identifier after let");
        };

        self.advance();
        self.consume(Token::Equals);

        let expr = self.parse_expression();
        Stmt::Let(name, expr)
    }

    fn parse_print(&mut self) -> Stmt {
        self.advance(); // consume 'print'
        let expr = self.parse_expression();
        Stmt::Print(expr)
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance(); // consume 'if'

        let condition = self.parse_expression();
        let then_branch = self.parse_block();

        let else_branch = if *self.current() == Token::Else {
            self.advance(); // consume 'else'
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

    fn parse_while(&mut self) -> Stmt {
        self.advance(); // consume 'while'

        let condition = self.parse_expression();
        let body = self.parse_block();

        Stmt::While { condition, body }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(Token::LeftBrace);

        let mut statements = Vec::new();

        while *self.current() != Token::RightBrace {
            if *self.current() == Token::Newline {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement());
        }

        self.consume(Token::RightBrace);
        statements
    }

    // =========================
    // Expressions (precedence)
    // =========================

    fn parse_expression(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut expr = self.parse_and();

        while *self.current() == Token::Or {
            self.advance();
            let right = self.parse_and();

            expr = Expr::Logical {
                left: Box::new(expr),
                op: "or".to_string(),
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_and(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while *self.current() == Token::And {
            self.advance();
            let right = self.parse_comparison();

            expr = Expr::Logical {
                left: Box::new(expr),
                op: "and".to_string(),
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_addition();

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
            let right = self.parse_addition();

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_addition(&mut self) -> Expr {
        let mut expr = self.parse_multiplication();

        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = match self.current() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!(),
            }
            .to_string();

            self.advance();
            let right = self.parse_multiplication();

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_multiplication(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while matches!(self.current(), Token::Star | Token::Slash) {
            let op = match self.current() {
                Token::Star => "*",
                Token::Slash => "/",
                _ => unreachable!(),
            }
            .to_string();

            self.advance();
            let right = self.parse_unary();

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if matches!(self.current(), Token::Not | Token::Bang | Token::Minus) {
            let op = match self.current() {
                Token::Not => "not",
                Token::Bang => "!",
                Token::Minus => "-",
                _ => unreachable!(),
            }
            .to_string();

            self.advance();
            let operand = self.parse_unary();

            return Expr::Unary {
                op,
                operand: Box::new(operand),
            };
        }

        self.parse_term()
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
                self.advance(); // '('
                let expr = self.parse_expression();
                self.consume(Token::RParen);
                expr
            }
            _ => panic!("Unexpected expression: {:?}", self.current()),
        }
    }
}
