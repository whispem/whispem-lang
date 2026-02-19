use crate::ast::{BinaryOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::error::{ErrorKind, WhispemError, WhispemResult};
use crate::token::{Spanned, Token};

pub struct Parser {
    tokens: Vec<Spanned>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned>) -> Self {
        Self { tokens, position: 0 }
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn current(&self) -> &Spanned {
        // The last token is always Eof, so this is always valid.
        &self.tokens[self.position.min(self.tokens.len() - 1)]
    }

    fn line(&self) -> usize {
        self.current().line
    }

    fn advance(&mut self) {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
    }

    fn skip_newlines(&mut self) {
        while self.current().token == Token::Newline {
            self.advance();
        }
    }

    fn consume(&mut self, expected: Token) -> WhispemResult<()> {
        if self.current().token == expected {
            self.advance();
            Ok(())
        } else {
            Err(WhispemError::new(
                ErrorKind::UnexpectedToken {
                    expected: expected.to_string(),
                    found: self.current().token.to_string(),
                },
                self.current().line,
                self.current().column,
            ))
        }
    }

    fn consume_identifier(&mut self) -> WhispemResult<String> {
        if let Token::Identifier(name) = &self.current().token {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(WhispemError::new(
                ErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current().token.to_string(),
                },
                self.current().line,
                self.current().column,
            ))
        }
    }

    // ── public entry point ────────────────────────────────────────────────────

    pub fn parse_program(&mut self) -> WhispemResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        loop {
            self.skip_newlines();
            if self.current().token == Token::Eof {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    // ── statements ────────────────────────────────────────────────────────────

    fn parse_statement(&mut self) -> WhispemResult<Stmt> {
        match &self.current().token {
            Token::Let      => self.parse_let(),
            Token::Print    => self.parse_print(),
            Token::If       => self.parse_if(),
            Token::While    => self.parse_while(),
            Token::For      => self.parse_for(),
            Token::Fn       => self.parse_function(),
            Token::Return   => self.parse_return(),
            Token::Break    => {
                let line = self.line();
                self.advance();
                Ok(Stmt::Break { line })
            }
            Token::Continue => {
                let line = self.line();
                self.advance();
                Ok(Stmt::Continue { line })
            }
            // Built-ins that can appear as bare statements (write_file, etc.)
            Token::WriteFile | Token::ReadFile => {
                let line = self.line();
                let name = match &self.current().token {
                    Token::WriteFile => "write_file",
                    Token::ReadFile  => "read_file",
                    _ => unreachable!(),
                }
                .to_string();
                self.advance();
                let arguments = self.parse_call_args()?;
                Ok(Stmt::Expression {
                    expr: Expr::Call { name, arguments, line },
                    line,
                })
            }
            Token::Identifier(_) => self.parse_identifier_stmt(),
            _ => {
                Err(WhispemError::new(
                    ErrorKind::UnexpectedToken {
                        expected: "statement".to_string(),
                        found: self.current().token.to_string(),
                    },
                    self.current().line,
                    self.current().column,
                ))
            }
        }
    }

    fn parse_let(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'let'
        let name = self.consume_identifier()?;
        self.consume(Token::Equals)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Let { name, value, line })
    }

    fn parse_print(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'print'
        let value = self.parse_expression()?;
        Ok(Stmt::Print { value, line })
    }

    fn parse_if(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'if'
        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        let else_branch = if self.current().token == Token::Else {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If { condition, then_branch, else_branch, line })
    }

    fn parse_while(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body, line })
    }

    fn parse_for(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'for'
        let variable = self.consume_identifier()?;
        self.consume(Token::In)?;
        let iterable = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::For { variable, iterable, body, line })
    }

    fn parse_function(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'fn'
        let name = self.consume_identifier()?;
        self.consume(Token::LParen)?;

        let mut params = Vec::new();
        if self.current().token != Token::RParen {
            loop {
                params.push(self.consume_identifier()?);
                if self.current().token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.consume(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::Function { name, params, body, line })
    }

    fn parse_return(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        self.advance(); // consume 'return'

        let value = if matches!(
            self.current().token,
            Token::Newline | Token::RightBrace | Token::Eof
        ) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        Ok(Stmt::Return { value, line })
    }

    fn parse_block(&mut self) -> WhispemResult<Vec<Stmt>> {
        self.consume(Token::LeftBrace)?;
        let mut stmts = Vec::new();

        loop {
            self.skip_newlines();
            if self.current().token == Token::RightBrace {
                break;
            }
            if self.current().token == Token::Eof {
                return Err(WhispemError::new(
                    ErrorKind::UnexpectedEof,
                    self.current().line,
                    self.current().column,
                ));
            }
            stmts.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace)?;
        Ok(stmts)
    }

    /// Handle statements that start with an identifier:
    /// - array/dict index assignment:  name[expr] = expr
    /// - function call as statement:   name(args)
    fn parse_identifier_stmt(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        let name = self.consume_identifier()?;

        if self.current().token == Token::LeftBracket {
            // index assignment
            self.advance();
            let index = self.parse_expression()?;
            self.consume(Token::RightBracket)?;
            self.consume(Token::Equals)?;
            let value = self.parse_expression()?;
            return Ok(Stmt::IndexAssign { object: name, index, value, line });
        }

        if self.current().token == Token::LParen {
            let arguments = self.parse_call_args()?;
            return Ok(Stmt::Expression {
                expr: Expr::Call { name, arguments, line },
                line,
            });
        }

        Err(WhispemError::new(
            ErrorKind::UnexpectedToken {
                expected: "'(' or '['".to_string(),
                found: self.current().token.to_string(),
            },
            self.current().line,
            self.current().column,
        ))
    }

    /// Parse `(arg, arg, ...)` — the opening paren must still be current.
    fn parse_call_args(&mut self) -> WhispemResult<Vec<Expr>> {
        self.consume(Token::LParen)?;
        let mut args = Vec::new();

        if self.current().token != Token::RParen {
            loop {
                args.push(self.parse_expression()?);
                if self.current().token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(Token::RParen)?;
        Ok(args)
    }

    // ── expressions (precedence climbing) ────────────────────────────────────

    fn parse_expression(&mut self) -> WhispemResult<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> WhispemResult<Expr> {
        let mut expr = self.parse_and()?;
        while self.current().token == Token::Or {
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op: LogicalOp::Or,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> WhispemResult<Expr> {
        let mut expr = self.parse_comparison()?;
        while self.current().token == Token::And {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op: LogicalOp::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> WhispemResult<Expr> {
        let mut expr = self.parse_addition()?;
        loop {
            let op = match self.current().token {
                Token::Less         => BinaryOp::Less,
                Token::LessEqual    => BinaryOp::LessEqual,
                Token::Greater      => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::EqualEqual   => BinaryOp::EqualEqual,
                Token::BangEqual    => BinaryOp::BangEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_addition(&mut self) -> WhispemResult<Expr> {
        let mut expr = self.parse_multiplication()?;
        loop {
            let op = match self.current().token {
                Token::Plus  => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> WhispemResult<Expr> {
        let mut expr = self.parse_unary()?;
        loop {
            let op = match self.current().token {
                Token::Star    => BinaryOp::Mul,
                Token::Slash   => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> WhispemResult<Expr> {
        match self.current().token {
            Token::Not | Token::Bang => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Not, operand: Box::new(operand) })
            }
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Neg, operand: Box::new(operand) })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> WhispemResult<Expr> {
        let mut expr = self.parse_call()?;

        // Chained indexing: expr[i][j]...
        while self.current().token == Token::LeftBracket {
            self.advance();
            let index = self.parse_expression()?;
            self.consume(Token::RightBracket)?;
            expr = Expr::Index {
                object: Box::new(expr),
                index: Box::new(index),
            };
        }

        Ok(expr)
    }

    fn parse_call(&mut self) -> WhispemResult<Expr> {
        let expr = self.parse_term()?;

        // If the expression is a variable and is immediately followed by '(',
        // treat it as a function call.
        if let Expr::Variable(ref name) = expr {
            if self.current().token == Token::LParen {
                let line = self.current().line;
                let name = name.clone();
                let arguments = self.parse_call_args()?;
                return Ok(Expr::Call { name, arguments, line });
            }
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> WhispemResult<Expr> {
        let spanned = self.current().clone();

        match &spanned.token {
            Token::Number(n) => {
                let v = *n;
                self.advance();
                Ok(Expr::Number(v))
            }
            Token::Str(s) => {
                let v = s.clone();
                self.advance();
                Ok(Expr::Str(v))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }

            // Built-in functions used as expressions (length(x), push(a,b), …)
            Token::Length    => { self.advance(); Ok(Expr::Variable("length".to_string())) }
            Token::Push      => { self.advance(); Ok(Expr::Variable("push".to_string())) }
            Token::Pop       => { self.advance(); Ok(Expr::Variable("pop".to_string())) }
            Token::Reverse   => { self.advance(); Ok(Expr::Variable("reverse".to_string())) }
            Token::Slice     => { self.advance(); Ok(Expr::Variable("slice".to_string())) }
            Token::Range     => { self.advance(); Ok(Expr::Variable("range".to_string())) }
            Token::Input     => { self.advance(); Ok(Expr::Variable("input".to_string())) }
            Token::ReadFile  => { self.advance(); Ok(Expr::Variable("read_file".to_string())) }
            Token::WriteFile => { self.advance(); Ok(Expr::Variable("write_file".to_string())) }
            Token::Keys      => { self.advance(); Ok(Expr::Variable("keys".to_string())) }
            Token::Values    => { self.advance(); Ok(Expr::Variable("values".to_string())) }
            Token::HasKey    => { self.advance(); Ok(Expr::Variable("has_key".to_string())) }

            // Array literal
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                if self.current().token != Token::RightBracket {
                    loop {
                        elements.push(self.parse_expression()?);
                        if self.current().token == Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.consume(Token::RightBracket)?;
                Ok(Expr::Array(elements))
            }

            // Dict literal  {key: value, ...}
            Token::LeftBrace => {
                self.advance();
                self.skip_newlines();
                let mut pairs = Vec::new();

                if self.current().token != Token::RightBrace {
                    loop {
                        self.skip_newlines();
                        let key = self.parse_expression()?;
                        self.consume(Token::Colon)?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                        self.skip_newlines();
                        if self.current().token == Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.skip_newlines();
                self.consume(Token::RightBrace)?;
                Ok(Expr::Dict(pairs))
            }

            Token::Identifier(name) => {
                let v = name.clone();
                self.advance();
                Ok(Expr::Variable(v))
            }

            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            }

            _ => Err(WhispemError::new(
                ErrorKind::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: spanned.token.to_string(),
                },
                spanned.line,
                spanned.column,
            )),
        }
    }
}