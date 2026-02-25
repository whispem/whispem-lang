use crate::opcode::OpCode;
use crate::value::Value;

/// A compiled unit: bytecode + constants pool + line info.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code:      Vec<u8>,
    pub constants: Vec<Value>,
    pub lines:     Vec<usize>,
    pub name:      String,
}

impl Chunk {
    pub fn new(name: impl Into<String>) -> Self {
        Self { code: Vec::new(), constants: Vec::new(), lines: Vec::new(), name: name.into() }
    }

    pub fn emit_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn emit_op(&mut self, op: OpCode, line: usize) {
        self.emit_byte(op as u8, line);
    }

    pub fn emit_op_u8(&mut self, op: OpCode, operand: u8, line: usize) {
        self.emit_byte(op as u8, line);
        self.emit_byte(operand, line);
    }

    pub fn emit_op_u16(&mut self, op: OpCode, operand: u16, line: usize) {
        self.emit_byte(op as u8, line);
        self.emit_byte((operand >> 8) as u8, line);
        self.emit_byte((operand & 0xFF) as u8, line);
    }

    pub fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.emit_byte(op as u8, line);
        self.emit_byte(0xFF, line);
        self.emit_byte(0xFF, line);
        self.code.len() - 2
    }

    pub fn patch_jump(&mut self, patch_at: usize, target: usize) {
        let t = target as u16;
        self.code[patch_at]     = (t >> 8) as u8;
        self.code[patch_at + 1] = (t & 0xFF) as u8;
    }

    /// Add a constant, deduplicating strings. Panics if pool exceeds 255.
    pub fn add_constant(&mut self, value: Value) -> u8 {
        if let Value::Str(ref s) = value {
            for (i, existing) in self.constants.iter().enumerate() {
                if let Value::Str(ref e) = existing {
                    if e == s { return i as u8; }
                }
            }
        }
        assert!(self.constants.len() < 256, "constants pool overflow in '{}'", self.name);
        let idx = self.constants.len() as u8;
        self.constants.push(value);
        idx
    }

    pub fn current_offset(&self) -> usize { self.code.len() }

    pub fn disassemble(&self) {
        println!("== {} ==", self.name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04}  ", offset);
        let line = self.lines[offset];
        if offset > 0 && self.lines[offset - 1] == line { print!("   |  "); }
        else { print!("{:4}  ", line); }

        let byte = self.code[offset];
        let op = match OpCode::from_byte(byte) {
            Some(op) => op,
            None => { println!("UNKNOWN({:#04x})", byte); return offset + 1; }
        };

        match op.operand_size() {
            0 => { println!("{}", op.name()); offset + 1 }
            1 => {
                let idx = self.code[offset + 1] as usize;
                println!("{:<16} {:3}    {}", op.name(), idx, self.constant_annotation(idx));
                offset + 2
            }
            2 => {
                match op {
                    OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue => {
                        let hi = self.code[offset + 1] as u16;
                        let lo = self.code[offset + 2] as u16;
                        println!("{:<16}        -> {:04}", op.name(), (hi << 8) | lo);
                    }
                    OpCode::Call => {
                        let name_idx = self.code[offset + 1] as usize;
                        let argc     = self.code[offset + 2];
                        println!("{:<16} {:3}    {} ({} args)", op.name(), name_idx, self.constant_annotation(name_idx), argc);
                    }
                    _ => {
                        println!("{:<16} {:#04x} {:#04x}", op.name(), self.code[offset+1], self.code[offset+2]);
                    }
                }
                offset + 3
            }
            _ => { println!("{} (?)", op.name()); offset + 1 }
        }
    }

    fn constant_annotation(&self, idx: usize) -> String {
        if idx >= self.constants.len() { return "(out of range)".to_string(); }
        match &self.constants[idx] {
            Value::Str(s)    => format!("'{}'", s),
            Value::Number(n) => if n.fract()==0.0 { format!("'{}'",*n as i64) } else { format!("'{}'",n) },
            Value::Bool(b)   => format!("'{}'", b),
            Value::None      => "'none'".to_string(),
            Value::Array(_)  => "[array]".to_string(),
            Value::Dict(_)   => "{dict}".to_string(),
        }
    }
}
