use crate::ast::{BinaryOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::error::{ErrorKind, WhispemError, WhispemResult};
use crate::token::{Spanned, Token};

pub struct Parser { tokens: Vec<Spanned>, position: usize }

impl Parser {
    pub fn new(tokens: Vec<Spanned>) -> Self { Self { tokens, position: 0 } }

    fn cur(&self)    -> &Spanned { &self.tokens[self.position.min(self.tokens.len()-1)] }
    fn line(&self)   -> usize    { self.cur().line }

    fn advance(&mut self) { if self.position+1 < self.tokens.len() { self.position += 1; } }

    fn skip_nl(&mut self) { while self.cur().token == Token::Newline { self.advance(); } }

    fn consume(&mut self, expected: Token) -> WhispemResult<()> {
        if self.cur().token == expected { self.advance(); Ok(()) }
        else { Err(WhispemError::new(ErrorKind::UnexpectedToken { expected: expected.to_string(), found: self.cur().token.to_string() }, self.cur().line, self.cur().column)) }
    }

    fn consume_ident(&mut self) -> WhispemResult<String> {
        if let Token::Identifier(name) = &self.cur().token {
            let n = name.clone(); self.advance(); Ok(n)
        } else {
            Err(WhispemError::new(ErrorKind::UnexpectedToken { expected: "identifier".to_string(), found: self.cur().token.to_string() }, self.cur().line, self.cur().column))
        }
    }

    pub fn parse_program(&mut self) -> WhispemResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        loop {
            self.skip_nl();
            if self.cur().token == Token::Eof { break; }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> WhispemResult<Stmt> {
        match &self.cur().token {
            Token::Let      => self.parse_let(),
            Token::Print    => self.parse_print(),
            Token::If       => self.parse_if(),
            Token::While    => self.parse_while(),
            Token::For      => self.parse_for(),
            Token::Fn       => self.parse_fn(),
            Token::Return   => self.parse_return(),
            Token::Break    => { let l=self.line(); self.advance(); Ok(Stmt::Break { line: l }) }
            Token::Continue => { let l=self.line(); self.advance(); Ok(Stmt::Continue { line: l }) }
            Token::WriteFile | Token::ReadFile => {
                let line = self.line();
                let name = match &self.cur().token { Token::WriteFile=>"write_file", _=>"read_file" }.to_string();
                self.advance();
                let args = self.parse_call_args()?;
                Ok(Stmt::Expression { expr: Expr::Call { name, arguments: args, line }, line })
            }
            Token::Identifier(_) => self.parse_ident_stmt(),
            _ => Err(WhispemError::new(ErrorKind::UnexpectedToken { expected: "statement".to_string(), found: self.cur().token.to_string() }, self.cur().line, self.cur().column)),
        }
    }

    fn parse_let(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        let name = self.consume_ident()?;
        self.consume(Token::Equals)?;
        let value = self.parse_expr()?;
        Ok(Stmt::Let { name, value, line })
    }

    fn parse_print(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        Ok(Stmt::Print { value: self.parse_expr()?, line })
    }

    fn parse_if(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        let cond = self.parse_expr()?;
        let then = self.parse_block()?;
        let else_ = if self.cur().token == Token::Else { self.advance(); Some(self.parse_block()?) } else { None };
        Ok(Stmt::If { condition: cond, then_branch: then, else_branch: else_, line })
    }

    fn parse_while(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition: cond, body, line })
    }

    fn parse_for(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        let var = self.consume_ident()?;
        self.consume(Token::In)?;
        let iter = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::For { variable: var, iterable: iter, body, line })
    }

    fn parse_fn(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        let name = self.consume_ident()?;
        self.consume(Token::LParen)?;
        let mut params = Vec::new();
        if self.cur().token != Token::RParen {
            loop {
                params.push(self.consume_ident()?);
                if self.cur().token == Token::Comma { self.advance(); } else { break; }
            }
        }
        self.consume(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::Function { name, params, body, line })
    }

    fn parse_return(&mut self) -> WhispemResult<Stmt> {
        let line = self.line(); self.advance();
        let value = if matches!(self.cur().token, Token::Newline|Token::RightBrace|Token::Eof) { None }
                    else { Some(self.parse_expr()?) };
        Ok(Stmt::Return { value, line })
    }

    fn parse_block(&mut self) -> WhispemResult<Vec<Stmt>> {
        self.consume(Token::LeftBrace)?;
        let mut stmts = Vec::new();
        loop {
            self.skip_nl();
            if self.cur().token == Token::RightBrace { break; }
            if self.cur().token == Token::Eof { return Err(WhispemError::new(ErrorKind::UnexpectedEof, self.cur().line, self.cur().column)); }
            stmts.push(self.parse_stmt()?);
        }
        self.consume(Token::RightBrace)?;
        Ok(stmts)
    }

    fn parse_ident_stmt(&mut self) -> WhispemResult<Stmt> {
        let line = self.line();
        let name = self.consume_ident()?;
        if self.cur().token == Token::LeftBracket {
            self.advance();
            let idx = self.parse_expr()?;
            self.consume(Token::RightBracket)?;
            self.consume(Token::Equals)?;
            let val = self.parse_expr()?;
            return Ok(Stmt::IndexAssign { object: name, index: idx, value: val, line });
        }
        if self.cur().token == Token::LParen {
            let args = self.parse_call_args()?;
            return Ok(Stmt::Expression { expr: Expr::Call { name, arguments: args, line }, line });
        }
        Err(WhispemError::new(ErrorKind::UnexpectedToken { expected: "'(' or '['".to_string(), found: self.cur().token.to_string() }, self.cur().line, self.cur().column))
    }

    fn parse_call_args(&mut self) -> WhispemResult<Vec<Expr>> {
        self.consume(Token::LParen)?;
        let mut args = Vec::new();
        if self.cur().token != Token::RParen {
            loop {
                args.push(self.parse_expr()?);
                if self.cur().token == Token::Comma { self.advance(); } else { break; }
            }
        }
        self.consume(Token::RParen)?;
        Ok(args)
    }

    fn parse_expr(&mut self)     -> WhispemResult<Expr> { self.parse_or() }

    fn parse_or(&mut self) -> WhispemResult<Expr> {
        let mut e = self.parse_and()?;
        while self.cur().token == Token::Or {
            self.advance();
            let r = self.parse_and()?;
            e = Expr::Logical { left: Box::new(e), op: LogicalOp::Or, right: Box::new(r) };
        }
        Ok(e)
    }

    fn parse_and(&mut self) -> WhispemResult<Expr> {
        let mut e = self.parse_cmp()?;
        while self.cur().token == Token::And {
            self.advance();
            let r = self.parse_cmp()?;
            e = Expr::Logical { left: Box::new(e), op: LogicalOp::And, right: Box::new(r) };
        }
        Ok(e)
    }

    fn parse_cmp(&mut self) -> WhispemResult<Expr> {
        let mut e = self.parse_add()?;
        loop {
            let op = match self.cur().token {
                Token::Less         => BinaryOp::Less,
                Token::LessEqual    => BinaryOp::LessEqual,
                Token::Greater      => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::EqualEqual   => BinaryOp::EqualEqual,
                Token::BangEqual    => BinaryOp::BangEqual,
                _ => break,
            };
            self.advance();
            let r = self.parse_add()?;
            e = Expr::Binary { left: Box::new(e), op, right: Box::new(r) };
        }
        Ok(e)
    }

    fn parse_add(&mut self) -> WhispemResult<Expr> {
        let mut e = self.parse_mul()?;
        loop {
            let op = match self.cur().token { Token::Plus=>BinaryOp::Add, Token::Minus=>BinaryOp::Sub, _=>break };
            self.advance();
            let r = self.parse_mul()?;
            e = Expr::Binary { left: Box::new(e), op, right: Box::new(r) };
        }
        Ok(e)
    }

    fn parse_mul(&mut self) -> WhispemResult<Expr> {
        let mut e = self.parse_unary()?;
        loop {
            let op = match self.cur().token { Token::Star=>BinaryOp::Mul, Token::Slash=>BinaryOp::Div, Token::Percent=>BinaryOp::Mod, _=>break };
            self.advance();
            let r = self.parse_unary()?;
            e = Expr::Binary { left: Box::new(e), op, right: Box::new(r) };
        }
        Ok(e)
    }

    fn parse_unary(&mut self) -> WhispemResult<Expr> {
        match self.cur().token {
            Token::Not | Token::Bang => { self.advance(); Ok(Expr::Unary { op: UnaryOp::Not, operand: Box::new(self.parse_unary()?) }) }
            Token::Minus             => { self.advance(); Ok(Expr::Unary { op: UnaryOp::Neg, operand: Box::new(self.parse_unary()?) }) }
            _                        => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> WhispemResult<Expr> {
        let mut e = self.parse_call()?;
        while self.cur().token == Token::LeftBracket {
            self.advance();
            let idx = self.parse_expr()?;
            self.consume(Token::RightBracket)?;
            e = Expr::Index { object: Box::new(e), index: Box::new(idx) };
        }
        Ok(e)
    }

    fn parse_call(&mut self) -> WhispemResult<Expr> {
        let e = self.parse_term()?;
        if let Expr::Variable(ref name) = e {
            if self.cur().token == Token::LParen {
                let line = self.cur().line;
                let name = name.clone();
                let args = self.parse_call_args()?;
                return Ok(Expr::Call { name, arguments: args, line });
            }
        }
        Ok(e)
    }

    fn parse_term(&mut self) -> WhispemResult<Expr> {
        let s = self.cur().clone();
        match &s.token {
            Token::Number(n)  => { let v=*n; self.advance(); Ok(Expr::Number(v)) }
            Token::Str(st)    => { let v=st.clone(); self.advance(); Ok(Expr::Str(v)) }
            Token::True       => { self.advance(); Ok(Expr::Bool(true)) }
            Token::False      => { self.advance(); Ok(Expr::Bool(false)) }
            Token::Length     => { self.advance(); Ok(Expr::Variable("length".to_string())) }
            Token::Push       => { self.advance(); Ok(Expr::Variable("push".to_string())) }
            Token::Pop        => { self.advance(); Ok(Expr::Variable("pop".to_string())) }
            Token::Reverse    => { self.advance(); Ok(Expr::Variable("reverse".to_string())) }
            Token::Slice      => { self.advance(); Ok(Expr::Variable("slice".to_string())) }
            Token::Range      => { self.advance(); Ok(Expr::Variable("range".to_string())) }
            Token::Input      => { self.advance(); Ok(Expr::Variable("input".to_string())) }
            Token::ReadFile   => { self.advance(); Ok(Expr::Variable("read_file".to_string())) }
            Token::WriteFile  => { self.advance(); Ok(Expr::Variable("write_file".to_string())) }
            Token::Keys       => { self.advance(); Ok(Expr::Variable("keys".to_string())) }
            Token::Values     => { self.advance(); Ok(Expr::Variable("values".to_string())) }
            Token::HasKey     => { self.advance(); Ok(Expr::Variable("has_key".to_string())) }
            Token::LeftBracket => {
                self.advance();
                let mut elems = Vec::new();
                if self.cur().token != Token::RightBracket {
                    loop {
                        elems.push(self.parse_expr()?);
                        if self.cur().token == Token::Comma { self.advance(); } else { break; }
                    }
                }
                self.consume(Token::RightBracket)?;
                Ok(Expr::Array(elems))
            }
            Token::LeftBrace => {
                self.advance(); self.skip_nl();
                let mut pairs = Vec::new();
                if self.cur().token != Token::RightBrace {
                    loop {
                        self.skip_nl();
                        let k = self.parse_expr()?;
                        self.consume(Token::Colon)?;
                        let v = self.parse_expr()?;
                        pairs.push((k,v));
                        self.skip_nl();
                        if self.cur().token == Token::Comma { self.advance(); } else { break; }
                    }
                }
                self.skip_nl();
                self.consume(Token::RightBrace)?;
                Ok(Expr::Dict(pairs))
            }
            Token::Identifier(name) => { let v=name.clone(); self.advance(); Ok(Expr::Variable(v)) }
            Token::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.consume(Token::RParen)?;
                Ok(e)
            }
            _ => Err(WhispemError::new(ErrorKind::UnexpectedToken { expected: "expression".to_string(), found: s.token.to_string() }, s.line, s.column)),
        }
    }
}