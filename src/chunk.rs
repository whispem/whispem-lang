use crate::error::{ErrorKind, WhispemError, WhispemResult};
use crate::opcode::OpCode;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code:         Vec<u8>,
    pub constants:    Vec<Value>,
    pub lines:        Vec<usize>,
    pub name:         String,
    pub param_count:  usize,
    // Number of upvalues this function closes over.
    pub upvalue_count: usize,
}

pub const MAGIC:          &[u8; 4] = b"WHBC";
pub const FORMAT_VERSION: u8       = 4;

impl Chunk {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            code:          Vec::new(),
            constants:     Vec::new(),
            lines:         Vec::new(),
            name:          name.into(),
            param_count:   0,
            upvalue_count: 0,
        }
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
        self.emit_byte(operand,  line);
    }

    pub fn emit_op_u16(&mut self, op: OpCode, operand: u16, line: usize) {
        self.emit_byte(op as u8,              line);
        self.emit_byte((operand >> 8) as u8,  line);
        self.emit_byte((operand & 0xFF) as u8, line);
    }

    pub fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.emit_byte(op as u8, line);
        self.emit_byte(0xFF,     line);
        self.emit_byte(0xFF,     line);
        self.code.len() - 2
    }

    pub fn patch_jump(&mut self, patch_at: usize, target: usize) {
        let t = target as u16;
        self.code[patch_at]     = (t >> 8) as u8;
        self.code[patch_at + 1] = (t & 0xFF) as u8;
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        // Deduplicate strings and numbers — matches wsc.wsp behaviour.
        let duplicate = match &value {
            Value::Str(s) => self.constants.iter().position(|c| {
                matches!(c, Value::Str(e) if e == s)
            }),
            Value::Number(n) => self.constants.iter().position(|c| {
                matches!(c, Value::Number(e) if e.to_bits() == n.to_bits())
            }),
            _ => None,
        };
        if let Some(idx) = duplicate {
            return idx as u8;
        }
        assert!(self.constants.len() < 256, "constants pool overflow in '{}'", self.name);
        let idx = self.constants.len() as u8;
        self.constants.push(value);
        idx
    }

    pub fn current_offset(&self) -> usize { self.code.len() }
}


pub fn serialise(
    main_chunk: &Chunk,
    functions:  &std::collections::HashMap<String, Chunk>,
) -> WhispemResult<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.push(FORMAT_VERSION);

    let mut all: Vec<&Chunk> = vec![main_chunk];
    let mut fn_names: Vec<&String> = functions.keys().collect();
    fn_names.sort();
    for name in &fn_names { all.push(&functions[*name]); }

    let fn_count = all.len() as u16;
    out.push((fn_count >> 8) as u8);
    out.push((fn_count & 0xFF) as u8);

    for chunk in all { write_chunk(chunk, &mut out)?; }
    Ok(out)
}

fn write_chunk(chunk: &Chunk, out: &mut Vec<u8>) -> WhispemResult<()> {
    let name_bytes = chunk.name.as_bytes();
    if name_bytes.len() > 0xFFFF {
        return Err(WhispemError::runtime(ErrorKind::SerializationError(
            format!("chunk name too long: '{}'", chunk.name),
        )));
    }
    let nl = name_bytes.len() as u16;
    out.push((nl >> 8) as u8);
    out.push((nl & 0xFF) as u8);
    out.extend_from_slice(name_bytes);
    out.push(chunk.param_count   as u8);
    out.push(chunk.upvalue_count as u8);

    if chunk.constants.len() > 256 {
        return Err(WhispemError::runtime(ErrorKind::SerializationError(
            format!("too many constants in '{}'", chunk.name),
        )));
    }
    out.push(chunk.constants.len() as u8);
    for c in &chunk.constants { write_const(c, out)?; }

    let code_len = chunk.code.len() as u32;
    out.extend_from_slice(&code_len.to_be_bytes());
    out.extend_from_slice(&chunk.code);

    let lines_len = chunk.lines.len() as u32;
    out.extend_from_slice(&lines_len.to_be_bytes());
    for &l in &chunk.lines { out.extend_from_slice(&(l as u32).to_be_bytes()); }

    Ok(())
}

fn write_const(v: &Value, out: &mut Vec<u8>) -> WhispemResult<()> {
    match v {
        Value::Number(n) => {
            out.push(0);
            out.extend_from_slice(&n.to_bits().to_be_bytes());
        }
        Value::Bool(b) => {
            out.push(1);
            out.push(if *b { 1 } else { 0 });
        }
        Value::Str(s) => {
            out.push(2);
            let bytes = s.as_bytes();
            if bytes.len() > 0xFFFF {
                return Err(WhispemError::runtime(ErrorKind::SerializationError(
                    "string constant too long".to_string(),
                )));
            }
            let slen = bytes.len() as u16;
            out.push((slen >> 8) as u8);
            out.push((slen & 0xFF) as u8);
            out.extend_from_slice(bytes);
        }
        Value::None => { out.push(3); }
        Value::Array(_) | Value::Dict(_) | Value::Closure { .. } => {
            return Err(WhispemError::runtime(ErrorKind::SerializationError(
                "arrays, dicts, and closures cannot appear in the constants pool".to_string(),
            )));
        }
    }
    Ok(())
}


pub fn deserialise(
    data: &[u8],
) -> WhispemResult<(Chunk, std::collections::HashMap<String, Chunk>)> {
    let mut cursor = 0usize;

    if data.len() < 5 { return Err(bad_bc("file too short")); }
    if &data[0..4] != MAGIC { return Err(bad_bc("bad magic bytes (not a .whbc file)")); }
    cursor += 4;

    let ver = data[cursor];
    cursor += 1;
    if ver != FORMAT_VERSION {
        return Err(bad_bc(format!("version mismatch: expected {}, got {}", FORMAT_VERSION, ver)));
    }

    let fn_count = read_u16(data, cursor)?;
    cursor += 2;
    if fn_count == 0 { return Err(bad_bc("bytecode contains no chunks")); }

    let mut chunks: Vec<Chunk> = Vec::with_capacity(fn_count as usize);
    for _ in 0..fn_count {
        let (chunk, new_cursor) = read_chunk(data, cursor)?;
        chunks.push(chunk);
        cursor = new_cursor;
    }

    let main_chunk = chunks.remove(0);
    let mut functions = std::collections::HashMap::new();
    for chunk in chunks { functions.insert(chunk.name.clone(), chunk); }
    Ok((main_chunk, functions))
}

fn read_chunk(data: &[u8], mut cursor: usize) -> WhispemResult<(Chunk, usize)> {
    let name_len = read_u16(data, cursor)? as usize;
    cursor += 2;
    need(data, cursor, name_len)?;
    let name = std::str::from_utf8(&data[cursor..cursor + name_len])
        .map_err(|_| bad_bc("chunk name is not valid UTF-8"))?
        .to_string();
    cursor += name_len;

    need(data, cursor, 2)?;
    let param_count   = data[cursor] as usize; cursor += 1;
    let upvalue_count = data[cursor] as usize; cursor += 1;

    need(data, cursor, 1)?;
    let const_count = data[cursor] as usize;
    cursor += 1;
    let mut constants = Vec::with_capacity(const_count);
    for _ in 0..const_count {
        let (v, new_cursor) = read_const(data, cursor)?;
        constants.push(v);
        cursor = new_cursor;
    }

    let code_len = read_u32(data, cursor)? as usize;
    cursor += 4;
    need(data, cursor, code_len)?;
    let code = data[cursor..cursor + code_len].to_vec();
    cursor += code_len;

    let lines_len = read_u32(data, cursor)? as usize;
    cursor += 4;
    need(data, cursor, lines_len * 4)?;
    let mut lines = Vec::with_capacity(lines_len);
    for i in 0..lines_len {
        let l = read_u32(data, cursor + i * 4)? as usize;
        lines.push(l);
    }
    cursor += lines_len * 4;

    Ok((Chunk { code, constants, lines, name, param_count, upvalue_count }, cursor))
}

fn read_const(data: &[u8], mut cursor: usize) -> WhispemResult<(Value, usize)> {
    need(data, cursor, 1)?;
    let tag = data[cursor];
    cursor += 1;
    match tag {
        0 => {
            need(data, cursor, 8)?;
            let bits = u64::from_be_bytes(data[cursor..cursor + 8].try_into().unwrap());
            cursor += 8;
            Ok((Value::Number(f64::from_bits(bits)), cursor))
        }
        1 => {
            need(data, cursor, 1)?;
            let b = data[cursor] != 0;
            cursor += 1;
            Ok((Value::Bool(b), cursor))
        }
        2 => {
            let slen = read_u16(data, cursor)? as usize;
            cursor += 2;
            need(data, cursor, slen)?;
            let s = std::str::from_utf8(&data[cursor..cursor + slen])
                .map_err(|_| bad_bc("string constant is not valid UTF-8"))?
                .to_string();
            cursor += slen;
            Ok((Value::Str(s), cursor))
        }
        3 => Ok((Value::None, cursor)),
        _ => Err(bad_bc(format!("unknown constant tag {}", tag))),
    }
}


impl Chunk {
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
        if offset > 0 && self.lines[offset - 1] == line {
            print!("   |  ");
        } else {
            print!("{:4}  ", line);
        }

        let byte = self.code[offset];
        let op = match OpCode::from_byte(byte) {
            Some(op) => op,
            None => {
                println!("UNKNOWN({:#04x})", byte);
                return offset + 1;
            }
        };

        match op {
            OpCode::MakeClosure => {
                let name_idx = self.code[offset + 1] as usize;
                let uv_count = self.code[offset + 2] as usize;
                println!(
                    "{:<20} {:3}    {} ({} upvalues)",
                    op.name(), name_idx, self.constant_annotation(name_idx), uv_count
                );
                // Variable-length descriptors: (is_local: 1) + (name_len: 1) + (name_len bytes)
                let mut pos = offset + 3;
                for _ in 0..uv_count {
                    pos += 1; // is_local
                    if pos < self.code.len() {
                        let nl = self.code[pos] as usize;
                        pos += 1 + nl;
                    }
                }
                pos
            }
            _ => match op.operand_size() {
                0 => { println!("{}", op.name()); offset + 1 }
                1 => {
                    let idx = self.code[offset + 1] as usize;
                    println!("{:<20} {:3}    {}", op.name(), idx, self.constant_annotation(idx));
                    offset + 2
                }
                2 => {
                    match op {
                        OpCode::Jump
                        | OpCode::JumpIfFalse
                        | OpCode::JumpIfTrue
                        | OpCode::PeekJumpIfFalse
                        | OpCode::PeekJumpIfTrue => {
                            let hi = self.code[offset + 1] as u16;
                            let lo = self.code[offset + 2] as u16;
                            println!("{:<20}        -> {:04}", op.name(), (hi << 8) | lo);
                        }
                        OpCode::Call => {
                            let name_idx = self.code[offset + 1] as usize;
                            let argc     = self.code[offset + 2];
                            println!(
                                "{:<20} {:3}    {} ({} args)",
                                op.name(), name_idx, self.constant_annotation(name_idx), argc
                            );
                        }
                        _ => {
                            println!("{:<20} {:#04x} {:#04x}", op.name(),
                                     self.code[offset + 1], self.code[offset + 2]);
                        }
                    }
                    offset + 3
                }
                _ => { println!("{} (?)", op.name()); offset + 1 }
            }
        }
    }

    fn constant_annotation(&self, idx: usize) -> String {
        if idx >= self.constants.len() { return "(out of range)".to_string(); }
        match &self.constants[idx] {
            Value::Str(s)    => format!("'{}'", s),
            Value::Number(n) => {
                if n.fract() == 0.0 { format!("'{}'", *n as i64) }
                else { format!("'{}'", n) }
            }
            Value::Bool(b)   => format!("'{}'", b),
            Value::None      => "'none'".to_string(),
            Value::Array(_)  => "[array]".to_string(),
            Value::Dict(_)   => "{dict}".to_string(),
            Value::Closure {..} => "<closure>".to_string(),
        }
    }
}


fn bad_bc(msg: impl Into<String>) -> WhispemError {
    WhispemError::runtime(ErrorKind::InvalidBytecode(msg.into()))
}

fn need(data: &[u8], cursor: usize, n: usize) -> WhispemResult<()> {
    if cursor + n > data.len() {
        Err(bad_bc(format!(
            "unexpected end of bytecode at offset {} (need {} bytes, have {})",
            cursor, n, data.len().saturating_sub(cursor)
        )))
    } else {
        Ok(())
    }
}

fn read_u16(data: &[u8], cursor: usize) -> WhispemResult<u16> {
    need(data, cursor, 2)?;
    Ok(((data[cursor] as u16) << 8) | (data[cursor + 1] as u16))
}

fn read_u32(data: &[u8], cursor: usize) -> WhispemResult<u32> {
    need(data, cursor, 4)?;
    Ok(u32::from_be_bytes(data[cursor..cursor + 4].try_into().unwrap()))
}