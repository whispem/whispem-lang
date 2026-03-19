use crate::ast::{BinaryOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::chunk::Chunk;
use crate::error::{ErrorKind, Span, WhispemError, WhispemResult};
use crate::opcode::OpCode;
use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UpvalueDesc {
    pub is_local: bool,
    pub name:     String,
}

struct FnScope {
    locals:   Vec<String>,
    upvalues: Vec<(String, UpvalueDesc)>,
}

impl FnScope {
    fn new(params: &[String]) -> Self {
        Self { locals: params.to_vec(), upvalues: Vec::new() }
    }
    fn has_local(&self, name: &str) -> bool { self.locals.iter().any(|n| n == name) }
    fn add_local(&mut self, name: &str) {
        if !self.has_local(name) { self.locals.push(name.to_string()); }
    }
    fn upvalue_slot(&self, name: &str) -> Option<u8> {
        self.upvalues.iter().position(|(n, _)| n == name).map(|i| i as u8)
    }
    fn add_upvalue(&mut self, name: &str, desc: UpvalueDesc) -> u8 {
        if let Some(slot) = self.upvalue_slot(name) { return slot; }
        let slot = self.upvalues.len() as u8;
        self.upvalues.push((name.to_string(), desc));
        slot
    }
    fn upvalue_descs(&self) -> Vec<UpvalueDesc> {
        self.upvalues.iter().map(|(_, d)| d.clone()).collect()
    }
}

pub struct Compiler {
    current:      Chunk,
    functions:    HashMap<String, Chunk>,
    loop_stack:   Vec<LoopContext>,
    global_names: Vec<String>,
    scope_stack:  Vec<FnScope>,
}

struct LoopContext {
    break_jumps:    Vec<usize>,
    continue_jumps: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current:      Chunk::new("<main>"),
            functions:    HashMap::new(),
            loop_stack:   Vec::new(),
            global_names: Vec::new(),
            scope_stack:  Vec::new(),
        }
    }

    pub fn compile(
        mut self,
        program: Vec<Stmt>,
    ) -> WhispemResult<(Chunk, HashMap<String, Chunk>)> {
        for stmt in &program {
            if let Stmt::Let { name, .. } = stmt {
                if !self.global_names.contains(name) {
                    self.global_names.push(name.clone());
                }
            }
        }
        for stmt in &program {
            if let Stmt::Function { name, params, body, line } = stmt {
                self.compile_named_fn(name, params, body, *line)?;
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

    fn compile_named_fn(
        &mut self, name: &str, params: &[String], body: &[Stmt], line: usize,
    ) -> WhispemResult<()> {
        let (chunk, uv_descs) = self.compile_fn_body(name, params, body, line)?;
        if !uv_descs.is_empty() {
            self.emit_make_closure(&chunk.name, &uv_descs, line)?;
            let idx = self.name_const(name, line)?;
            self.current.emit_op_u8(OpCode::Store, idx, line);
        }
        Ok(())
    }

    fn compile_fn_body(
        &mut self, name: &str, params: &[String], body: &[Stmt], line: usize,
    ) -> WhispemResult<(Chunk, Vec<UpvalueDesc>)> {
        let parent = std::mem::replace(&mut self.current, Chunk::new(name));
        self.scope_stack.push(FnScope::new(params));
        self.current.param_count = params.len();

        for param in params.iter().rev() {
            let idx = self.name_const(param, line)?;
            self.current.emit_op_u8(OpCode::Store, idx, line);
        }
        for stmt in body { self.compile_stmt(stmt.clone())?; }
        self.current.emit_op(OpCode::ReturnNone, line);

        let fn_chunk = std::mem::replace(&mut self.current, parent);
        let scope    = self.scope_stack.pop().unwrap();
        let uv_descs = scope.upvalue_descs();
        self.functions.insert(fn_chunk.name.clone(), fn_chunk.clone());
        Ok((fn_chunk, uv_descs))
    }

    fn emit_make_closure(
        &mut self, chunk_name: &str, uv_descs: &[UpvalueDesc], line: usize,
    ) -> WhispemResult<()> {
        let name_c   = self.name_const(chunk_name, line)?;
        let uv_count = uv_descs.len() as u8;
        self.current.emit_byte(OpCode::MakeClosure as u8, line);
        self.current.emit_byte(name_c,   line);
        self.current.emit_byte(uv_count, line);
        for uv in uv_descs {
            self.current.emit_byte(if uv.is_local { 1 } else { 0 }, line);
            let name_bytes = uv.name.as_bytes();
            assert!(name_bytes.len() < 256, "upvalue name too long");
            self.current.emit_byte(name_bytes.len() as u8, line);
            for b in name_bytes { self.current.emit_byte(*b, line); }
        }
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: Stmt) -> WhispemResult<()> {
        match stmt {
            Stmt::Let { name, value, line } => {
                self.compile_expr(value, line)?;
                let depth = self.scope_stack.len();
                // If the name is an upvalue in the current scope, write through the cell.
                if depth > 0 {
                    if let Some(slot) = self.scope_stack[depth - 1].upvalue_slot(&name) {
                        self.current.emit_op_u8(OpCode::StoreUpvalue, slot, line);
                        return Ok(());
                    }
                }
                let idx = self.name_const(&name, line)?;
                self.current.emit_op_u8(OpCode::Store, idx, line);
                if self.scope_stack.is_empty() {
                    if !self.global_names.contains(&name) {
                        self.global_names.push(name.clone());
                    }
                } else {
                    self.scope_stack.last_mut().unwrap().add_local(&name);
                }
            }
            Stmt::Print { value, line } => {
                self.compile_expr(value, line)?;
                self.current.emit_op(OpCode::Print, line);
            }
            Stmt::If { condition, then_branch, else_branch, line } => {
                self.compile_expr(condition, line)?;
                let jelse = self.current.emit_jump(OpCode::JumpIfFalse, line);
                for s in then_branch { self.compile_stmt(s)?; }
                if let Some(else_stmts) = else_branch {
                    let jend       = self.current.emit_jump(OpCode::Jump, line);
                    let else_start = self.current.current_offset();
                    self.current.patch_jump(jelse, else_start);
                    for s in else_stmts { self.compile_stmt(s)?; }
                    let end = self.current.current_offset();
                    self.current.patch_jump(jend, end);
                } else {
                    let end = self.current.current_offset();
                    self.current.patch_jump(jelse, end);
                }
            }
            Stmt::While { condition, body, line } => {
                let loop_start = self.current.current_offset();
                self.loop_stack.push(LoopContext { break_jumps: vec![], continue_jumps: vec![] });
                self.compile_expr(condition, line)?;
                let exit = self.current.emit_jump(OpCode::JumpIfFalse, line);
                for s in body { self.compile_stmt(s)?; }
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
                self.loop_stack.push(LoopContext { break_jumps: vec![], continue_jumps: vec![] });

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

                for s in body { self.compile_stmt(s)?; }

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
                    return Err(WhispemError::new(ErrorKind::BreakOutsideLoop, Span::new(line, 0)));
                }
                let p = self.current.emit_jump(OpCode::Jump, line);
                self.loop_stack.last_mut().unwrap().break_jumps.push(p);
            }
            Stmt::Continue { line } => {
                if self.loop_stack.is_empty() {
                    return Err(WhispemError::new(ErrorKind::ContinueOutsideLoop, Span::new(line, 0)));
                }
                let p = self.current.emit_jump(OpCode::Jump, line);
                self.loop_stack.last_mut().unwrap().continue_jumps.push(p);
            }
            Stmt::IndexAssign { object, index, value, line } => {
                let obj_c = self.name_const(&object, line)?;
                self.current.emit_op_u8(OpCode::Load, obj_c, line);
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

    fn compile_expr(&mut self, expr: Expr, line: usize) -> WhispemResult<()> {
        match expr {
            Expr::Number(n) => {
                let idx = self.current.add_constant(Value::Number(n));
                self.current.emit_op_u8(OpCode::PushConst, idx, line);
            }
            Expr::Str(s) => {
                let idx = self.current.add_constant(Value::Str(s));
                self.current.emit_op_u8(OpCode::PushConst, idx, line);
            }
            Expr::Bool(true)  => self.current.emit_op(OpCode::PushTrue,  line),
            Expr::Bool(false) => self.current.emit_op(OpCode::PushFalse, line),
            Expr::FStr(_)     => unreachable!("FStr must be desugared by the parser"),
            Expr::Variable(name) => { self.emit_load(&name, line)?; }
            Expr::Array(elems) => {
                let n = elems.len() as u8;
                for e in elems { self.compile_expr(e, line)?; }
                self.current.emit_op_u8(OpCode::MakeArray, n, line);
            }
            Expr::Dict(pairs) => {
                let n = pairs.len() as u8;
                for (k, v) in pairs {
                    self.compile_expr(k, line)?;
                    self.compile_expr(v, line)?;
                }
                self.current.emit_op_u8(OpCode::MakeDict, n, line);
            }
            Expr::Index { object, index } => {
                self.compile_expr(*object, line)?;
                self.compile_expr(*index,  line)?;
                self.current.emit_op(OpCode::GetIndex, line);
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(*left,  line)?;
                self.compile_expr(*right, line)?;
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
                self.current.emit_op(opcode, line);
            }
            Expr::Logical { left, op, right } => {
                self.compile_expr(*left, line)?;
                match op {
                    LogicalOp::And => {
                        let j = self.current.emit_jump(OpCode::PeekJumpIfFalse, line);
                        self.current.emit_op(OpCode::Pop, line);
                        self.compile_expr(*right, line)?;
                        let done = self.current.current_offset();
                        self.current.patch_jump(j, done);
                    }
                    LogicalOp::Or => {
                        let j = self.current.emit_jump(OpCode::PeekJumpIfTrue, line);
                        self.current.emit_op(OpCode::Pop, line);
                        self.compile_expr(*right, line)?;
                        let done = self.current.current_offset();
                        self.current.patch_jump(j, done);
                    }
                }
            }
            Expr::Unary { op, operand } => {
                self.compile_expr(*operand, line)?;
                match op {
                    UnaryOp::Not => self.current.emit_op(OpCode::Not, line),
                    UnaryOp::Neg => self.current.emit_op(OpCode::Neg, line),
                }
            }
            Expr::Call { name, arguments, line: call_line } => {
                let argc   = arguments.len() as u8;
                for arg in arguments { self.compile_expr(arg, call_line)?; }
                let name_c = self.name_const(&name, call_line)?;
                self.current.emit_byte(OpCode::Call as u8, call_line);
                self.current.emit_byte(name_c, call_line);
                self.current.emit_byte(argc,   call_line);
            }
            Expr::CallExpr { callee, arguments, line: call_line } => {
                let argc = arguments.len() as u8;
                self.compile_expr(*callee, call_line)?;
                for arg in arguments { self.compile_expr(arg, call_line)?; }
                let sentinel = self.name_const("__callee__", call_line)?;
                self.current.emit_byte(OpCode::Call as u8, call_line);
                self.current.emit_byte(sentinel, call_line);
                self.current.emit_byte(argc, call_line);
            }
            Expr::Lambda { params, body, line: lline } => {
                let lambda_name = format!("__lambda_{}_{}", lline, self.functions.len());
                let (chunk, uv_descs) =
                    self.compile_fn_body(&lambda_name, &params, &body, lline)?;
                self.emit_make_closure(&chunk.name, &uv_descs, lline)?;
            }
        }
        Ok(())
    }

    fn emit_load(&mut self, name: &str, line: usize) -> WhispemResult<()> {
        let depth = self.scope_stack.len();

        if depth == 0 {
            let idx = self.name_const(name, line)?;
            self.current.emit_op_u8(OpCode::Load, idx, line);
            return Ok(());
        }

        if self.scope_stack[depth - 1].has_local(name) {
            let idx = self.name_const(name, line)?;
            self.current.emit_op_u8(OpCode::Load, idx, line);
            return Ok(());
        }

        if let Some(slot) = self.scope_stack[depth - 1].upvalue_slot(name) {
            self.current.emit_op_u8(OpCode::LoadUpvalue, slot, line);
            return Ok(());
        }

        if let Some(uv_slot) = self.resolve_upvalue(depth - 1, name, line)? {
            self.current.emit_op_u8(OpCode::LoadUpvalue, uv_slot, line);
            return Ok(());
        }

        if self.global_names.contains(&name.to_string()) {
            let idx = self.name_const(name, line)?;
            self.current.emit_op_u8(OpCode::LoadGlobal, idx, line);
            return Ok(());
        }

        let idx = self.name_const(name, line)?;
        self.current.emit_op_u8(OpCode::Load, idx, line);
        Ok(())
    }

    fn resolve_upvalue(
        &mut self, scope_idx: usize, name: &str, line: usize,
    ) -> WhispemResult<Option<u8>> {
        if scope_idx == 0 { return Ok(None); }
        let parent = scope_idx - 1;

        if self.scope_stack[parent].has_local(name) {
            let slot = self.scope_stack[scope_idx].add_upvalue(
                name,
                UpvalueDesc { is_local: true, name: name.to_string() },
            );
            return Ok(Some(slot));
        }

        if let Some(parent_slot) = self.resolve_upvalue(parent, name, line)? {
            let slot = self.scope_stack[scope_idx].add_upvalue(
                name,
                UpvalueDesc { is_local: false, name: parent_slot.to_string() },
            );
            return Ok(Some(slot));
        }

        Ok(None)
    }

    fn name_const(&mut self, name: &str, line: usize) -> WhispemResult<u8> {
        if self.current.constants.len() >= 256 {
            return Err(WhispemError::new(
                ErrorKind::TooManyConstants, Span::new(line, 0),
            ));
        }
        Ok(self.current.add_constant(Value::Str(name.to_string())))
    }
}