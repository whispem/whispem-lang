#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    PushConst   = 0x00,
    PushTrue    = 0x01,
    PushFalse   = 0x02,
    PushNone    = 0x03,
    Load        = 0x10,
    Store       = 0x11,
    Add         = 0x20,
    Sub         = 0x21,
    Mul         = 0x22,
    Div         = 0x23,
    Mod         = 0x24,
    Neg         = 0x25,
    Eq          = 0x30,
    Neq         = 0x31,
    Lt          = 0x32,
    Lte         = 0x33,
    Gt          = 0x34,
    Gte         = 0x35,
    Not         = 0x36,
    Jump              = 0x40,
    JumpIfFalse       = 0x41,
    JumpIfTrue        = 0x42,
    PeekJumpIfFalse   = 0x43, // peek without pop — used by `and`
    PeekJumpIfTrue    = 0x44, // peek without pop — used by `or`
    Call        = 0x50,
    Return      = 0x51,
    ReturnNone  = 0x52,
    MakeArray   = 0x60,
    MakeDict    = 0x61,
    GetIndex    = 0x62,
    SetIndex    = 0x63,
    Print       = 0x70,
    Pop         = 0x71,
    Halt        = 0xFF,
}

impl OpCode {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::PushConst),
            0x01 => Some(Self::PushTrue),
            0x02 => Some(Self::PushFalse),
            0x03 => Some(Self::PushNone),
            0x10 => Some(Self::Load),
            0x11 => Some(Self::Store),
            0x20 => Some(Self::Add),
            0x21 => Some(Self::Sub),
            0x22 => Some(Self::Mul),
            0x23 => Some(Self::Div),
            0x24 => Some(Self::Mod),
            0x25 => Some(Self::Neg),
            0x30 => Some(Self::Eq),
            0x31 => Some(Self::Neq),
            0x32 => Some(Self::Lt),
            0x33 => Some(Self::Lte),
            0x34 => Some(Self::Gt),
            0x35 => Some(Self::Gte),
            0x36 => Some(Self::Not),
            0x40 => Some(Self::Jump),
            0x41 => Some(Self::JumpIfFalse),
            0x42 => Some(Self::JumpIfTrue),
            0x43 => Some(Self::PeekJumpIfFalse),
            0x44 => Some(Self::PeekJumpIfTrue),
            0x50 => Some(Self::Call),
            0x51 => Some(Self::Return),
            0x52 => Some(Self::ReturnNone),
            0x60 => Some(Self::MakeArray),
            0x61 => Some(Self::MakeDict),
            0x62 => Some(Self::GetIndex),
            0x63 => Some(Self::SetIndex),
            0x70 => Some(Self::Print),
            0x71 => Some(Self::Pop),
            0xFF => Some(Self::Halt),
            _    => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::PushConst         => "PUSH_CONST",
            Self::PushTrue          => "PUSH_TRUE",
            Self::PushFalse         => "PUSH_FALSE",
            Self::PushNone          => "PUSH_NONE",
            Self::Load              => "LOAD",
            Self::Store             => "STORE",
            Self::Add               => "ADD",
            Self::Sub               => "SUB",
            Self::Mul               => "MUL",
            Self::Div               => "DIV",
            Self::Mod               => "MOD",
            Self::Neg               => "NEG",
            Self::Eq                => "EQ",
            Self::Neq               => "NEQ",
            Self::Lt                => "LT",
            Self::Lte               => "LTE",
            Self::Gt                => "GT",
            Self::Gte               => "GTE",
            Self::Not               => "NOT",
            Self::Jump              => "JUMP",
            Self::JumpIfFalse       => "JUMP_IF_FALSE",
            Self::JumpIfTrue        => "JUMP_IF_TRUE",
            Self::PeekJumpIfFalse   => "PEEK_JUMP_IF_FALSE",
            Self::PeekJumpIfTrue    => "PEEK_JUMP_IF_TRUE",
            Self::Call              => "CALL",
            Self::Return            => "RETURN",
            Self::ReturnNone        => "RETURN_NONE",
            Self::MakeArray         => "MAKE_ARRAY",
            Self::MakeDict          => "MAKE_DICT",
            Self::GetIndex          => "GET_INDEX",
            Self::SetIndex          => "SET_INDEX",
            Self::Print             => "PRINT",
            Self::Pop               => "POP",
            Self::Halt              => "HALT",
        }
    }

    pub fn operand_size(self) -> usize {
        match self {
            Self::PushConst
            | Self::Load
            | Self::Store
            | Self::MakeArray
            | Self::MakeDict  => 1,
            Self::Jump
            | Self::JumpIfFalse
            | Self::JumpIfTrue
            | Self::PeekJumpIfFalse
            | Self::PeekJumpIfTrue
            | Self::Call      => 2,
            _                 => 0,
        }
    }
}