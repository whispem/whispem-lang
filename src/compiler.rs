use crate::ast::{BinaryOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::chunk::Chunk;
use crate::error::{ErrorKind, Span, WhispemError, WhispemResult};
use crate::opcode::OpCode;
use crate::value::Value;
use std::collections::HashMap;

pub struct Compiler {
    current:    Chunk,
    functions:  HashMap<String, Chunk>,
    loop_stack: Vec<LoopContext>,
}

struct LoopContext {
    break_jumps:    Vec<usize>,
    continue_jumps: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current:    Chunk::new("<main>"),
            functions:  HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    pub fn compile(
        mut self,
        program: Vec<Stmt>,
    ) -> WhispemResult<(Chunk, HashMap<String, Chunk>)> {
        // First pass: functions, so forward calls work.
        for stmt in &program {
            if let Stmt::Function { name, params, body, line } = stmt {
                self.compile_function(name, params, body, *line)?;
            }
        }
        for stmt in program {
            if !matches!(stmt, Stmt::Function { .. }) {
                self.compile_stmt(stmt)?;
            }
        }
        self.current.emit_op(OpCode::Halt, 0);
        Ok((self.current, self.functions))
    }

    fn compile_function(
        &mut self,
        name:   &str,
        params: &[String],
        body:   &[Stmt],
        line:   usize,
    ) -> WhispemResult<()> {
        let parent = std::mem::replace(&mut self.current, Chunk::new(name));
        self.current.param_count = params.len();

        // Caller pushes args left-to-right, so we STORE in reverse.
        for param in params.iter().rev() {
            let idx = self.name_const(param, line)?;
            self.current.emit_op_u8(OpCode::Store, idx, line);
        }
        for stmt in body {
            self.compile_stmt(stmt.clone())?;
        }
        self.current.emit_op(OpCode::ReturnNone, line);

        let fn_chunk = std::mem::replace(&mut self.current, parent);
        self.functions.insert(name.to_string(), fn_chunk);
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: Stmt) -> WhispemResult<()> {
        match stmt {
            Stmt::Let { name, value, line } => {
                self.compile_expr(value, line)?;
                let idx = self.name_const(&name, line)?;
                self.current.emit_op_u8(OpCode::Store, idx, line);
            }

            Stmt::Print { value, line } => {
                self.compile_expr(value, line)?;
                self.current.emit_op(OpCode::Print, line);
            }

            Stmt::If { condition, then_branch, else_branch, line } => {
                self.compile_expr(condition, line)?;
                let jump_else = self.current.emit_jump(OpCode::JumpIfFalse, line);

                for s in then_branch {
                    self.compile_stmt(s)?;
                }

                if let Some(else_stmts) = else_branch {
                    let jump_end   = self.current.emit_jump(OpCode::Jump, line);
                    let else_start = self.current.current_offset();
                    self.current.patch_jump(jump_else, else_start);
                    for s in else_stmts {
                        self.compile_stmt(s)?;
                    }
                    let end = self.current.current_offset();
                    self.current.patch_jump(jump_end, end);
                } else {
                    let end = self.current.current_offset();
                    self.current.patch_jump(jump_else, end);
                }
            }

            Stmt::While { condition, body, line } => {
                let loop_start = self.current.current_offset();
                self.loop_stack.push(LoopContext {
                    break_jumps:    vec![],
                    continue_jumps: vec![],
                });

                self.compile_expr(condition, line)?;
                let exit = self.current.emit_jump(OpCode::JumpIfFalse, line);

                for s in body {
                    self.compile_stmt(s)?;
                }

                self.current.emit_op_u16(OpCode::Jump, loop_start as u16, line);
                let after = self.current.current_offset();
                self.current.patch_jump(exit, after);

                let ctx = self.loop_stack.pop().unwrap();
                for p in ctx.break_jumps    { self.current.patch_jump(p, after); }
                for p in ctx.continue_jumps { self.current.patch_jump(p, loop_start); }
            }

            Stmt::For { variable, iterable, body, line } => {
                let depth     = self.loop_stack.len();
                let iter_name = format!("__iter_{}", depth);
                let idx_name  = format!("__idx_{}", depth);

                self.compile_expr(iterable, line)?;
                let iter_c = self.name_const(&iter_name, line)?;
                self.current.emit_op_u8(OpCode::Store, iter_c, line);

                let zero = self.current.add_constant(Value::Number(0.0));
                self.current.emit_op_u8(OpCode::PushConst, zero, line);
                let idx_c = self.name_const(&idx_name, line)?;
                self.current.emit_op_u8(OpCode::Store, idx_c, line);

                let loop_start = self.current.current_offset();
                self.loop_stack.push(LoopContext {
                    break_jumps:    vec![],
                    continue_jumps: vec![],
                });

                self.current.emit_op_u8(OpCode::Load, idx_c, line);
                self.current.emit_op_u8(OpCode::Load, iter_c, line);
                let len_c = self.name_const("length", line)?;
                self.current.emit_byte(OpCode::Call as u8, line);
                self.current.emit_byte(len_c, line);
                self.current.emit_byte(1, line);
                self.current.emit_op(OpCode::Lt, line);

                let exit = self.current.emit_jump(OpCode::JumpIfFalse, line);

                self.current.emit_op_u8(OpCode::Load, iter_c, line);
                self.current.emit_op_u8(OpCode::Load, idx_c, line);
                self.current.emit_op(OpCode::GetIndex, line);
                let var_c = self.name_const(&variable, line)?;
                self.current.emit_op_u8(OpCode::Store, var_c, line);

                for s in body {
                    self.compile_stmt(s)?;
                }

                let continue_target = self.current.current_offset();

                self.current.emit_op_u8(OpCode::Load, idx_c, line);
                let one = self.current.add_constant(Value::Number(1.0));
                self.current.emit_op_u8(OpCode::PushConst, one, line);
                self.current.emit_op(OpCode::Add, line);
                self.current.emit_op_u8(OpCode::Store, idx_c, line);
                self.current.emit_op_u16(OpCode::Jump, loop_start as u16, line);

                let after = self.current.current_offset();
                self.current.patch_jump(exit, after);

                let ctx = self.loop_stack.pop().unwrap();
                for p in ctx.break_jumps    { self.current.patch_jump(p, after); }
                for p in ctx.continue_jumps { self.current.patch_jump(p, continue_target); }
            }

            Stmt::Function { .. } => {}

            Stmt::Return { value, line } => {
                if let Some(expr) = value {
                    self.compile_expr(expr, line)?;
                    self.current.emit_op(OpCode::Return, line);
                } else {
                    self.current.emit_op(OpCode::ReturnNone, line);
                }
            }

            Stmt::Break { line } => {
                if self.loop_stack.is_empty() {
                    return Err(WhispemError::new(
                        ErrorKind::BreakOutsideLoop,
                        Span::new(line, 0),
                    ));
                }
                let p = self.current.emit_jump(OpCode::Jump, line);
                self.loop_stack.last_mut().unwrap().break_jumps.push(p);
            }

            Stmt::Continue { line } => {
                if self.loop_stack.is_empty() {
                    return Err(WhispemError::new(
                        ErrorKind::ContinueOutsideLoop,
                        Span::new(line, 0),
                    ));
                }
                let p = self.current.emit_jump(OpCode::Jump, line);
                self.loop_stack.last_mut().unwrap().continue_jumps.push(p);
            }

            Stmt::IndexAssign { object, index, value, line } => {
                let obj_c = self.name_const(&object, line)?;
                self.current.emit_op_u8(OpCode::Load,  obj_c, line);
                self.compile_expr(index, line)?;
                self.compile_expr(value, line)?;
                self.current.emit_op(OpCode::SetIndex, line);
                self.current.emit_op_u8(OpCode::Store, obj_c, line);
            }

            Stmt::Expression { expr, line } => {
                self.compile_expr(expr, line)?;
                self.current.emit_op(OpCode::Pop, line);
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: Expr, ctx_line: usize) -> WhispemResult<()> {
        match expr {
            Expr::Number(n) => {
                let idx = self.current.add_constant(Value::Number(n));
                self.current.emit_op_u8(OpCode::PushConst, idx, ctx_line);
            }
            Expr::Str(s) => {
                let idx = self.current.add_constant(Value::Str(s));
                self.current.emit_op_u8(OpCode::PushConst, idx, ctx_line);
            }
            Expr::Bool(true)  => self.current.emit_op(OpCode::PushTrue,  ctx_line),
            Expr::Bool(false) => self.current.emit_op(OpCode::PushFalse, ctx_line),

            Expr::Variable(name) => {
                let idx = self.current.add_constant(Value::Str(name));
                self.current.emit_op_u8(OpCode::Load, idx, ctx_line);
            }

            Expr::Array(elems) => {
                let n = elems.len() as u8;
                for e in elems {
                    self.compile_expr(e, ctx_line)?;
                }
                self.current.emit_op_u8(OpCode::MakeArray, n, ctx_line);
            }

            Expr::Dict(pairs) => {
                let n = pairs.len() as u8;
                for (k, v) in pairs {
                    self.compile_expr(k, ctx_line)?;
                    self.compile_expr(v, ctx_line)?;
                }
                self.current.emit_op_u8(OpCode::MakeDict, n, ctx_line);
            }

            Expr::Index { object, index } => {
                self.compile_expr(*object, ctx_line)?;
                self.compile_expr(*index,  ctx_line)?;
                self.current.emit_op(OpCode::GetIndex, ctx_line);
            }

            Expr::Binary { left, op, right } => {
                self.compile_expr(*left,  ctx_line)?;
                self.compile_expr(*right, ctx_line)?;
                let opcode = match op {
                    BinaryOp::Add          => OpCode::Add,
                    BinaryOp::Sub          => OpCode::Sub,
                    BinaryOp::Mul          => OpCode::Mul,
                    BinaryOp::Div          => OpCode::Div,
                    BinaryOp::Mod          => OpCode::Mod,
                    BinaryOp::Less         => OpCode::Lt,
                    BinaryOp::LessEqual    => OpCode::Lte,
                    BinaryOp::Greater      => OpCode::Gt,
                    BinaryOp::GreaterEqual => OpCode::Gte,
                    BinaryOp::EqualEqual   => OpCode::Eq,
                    BinaryOp::BangEqual    => OpCode::Neq,
                };
                self.current.emit_op(opcode, ctx_line);
            }

            Expr::Logical { left, op, right } => {
                // PeekJump* tests the top without popping it.
                // If we short-circuit, the left value stays as the result.
                // If we don't, we pop it and evaluate the right side.
                self.compile_expr(*left, ctx_line)?;
                match op {
                    LogicalOp::And => {
                        let j = self.current.emit_jump(OpCode::PeekJumpIfFalse, ctx_line);
                        self.current.emit_op(OpCode::Pop, ctx_line);
                        self.compile_expr(*right, ctx_line)?;
                        let done = self.current.current_offset();
                        self.current.patch_jump(j, done);
                    }
                    LogicalOp::Or => {
                        let j = self.current.emit_jump(OpCode::PeekJumpIfTrue, ctx_line);
                        self.current.emit_op(OpCode::Pop, ctx_line);
                        self.compile_expr(*right, ctx_line)?;
                        let done = self.current.current_offset();
                        self.current.patch_jump(j, done);
                    }
                }
            }

            Expr::Unary { op, operand } => {
                self.compile_expr(*operand, ctx_line)?;
                match op {
                    UnaryOp::Not => self.current.emit_op(OpCode::Not, ctx_line),
                    UnaryOp::Neg => self.current.emit_op(OpCode::Neg, ctx_line),
                }
            }

            Expr::Call { name, arguments, line } => {
                let argc = arguments.len() as u8;
                for arg in arguments {
                    self.compile_expr(arg, line)?;
                }
                let name_c = self.name_const(&name, line)?;
                self.current.emit_byte(OpCode::Call as u8, line);
                self.current.emit_byte(name_c, line);
                self.current.emit_byte(argc, line);
            }
        }
        Ok(())
    }

    fn name_const(&mut self, name: &str, line: usize) -> WhispemResult<u8> {
        if self.current.constants.len() >= 256 {
            return Err(WhispemError::new(
                ErrorKind::TooManyConstants,
                Span::new(line, 0),
            ));
        }
        Ok(self.current.add_constant(Value::Str(name.to_string())))
    }
}