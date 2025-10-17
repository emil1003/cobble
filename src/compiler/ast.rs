use std::fmt;

/// Operand types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    /// Register address
    Reg(u8),

    /// 8-bit immediate value
    Imm8(u8),

    /// 12-bit immediate value
    Imm12(u16),

    /// Address label
    Label(String),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reg(r) => write!(f, "r{}", r),
            Self::Imm8(v) => write!(f, "{}", v),
            Self::Imm12(v) => write!(f, "{}", v),
            Self::Label(l) => write!(f, "<{}>", l),
        }
    }
}

/// Instruction types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    Label(String),

    Halt,
    Addi { rd: Op, rs1: Op, imm: Op },
    Mv { rd: Op, rs1: Op },
    Nop,

    Add { rd: Op, rs1: Op, rs2: Op },
    Sub { rd: Op, rs1: Op, rs2: Op },

    Jmp { target: Op },
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label(l) => write!(f, "{}:", l),
            Self::Halt => write!(f, "halt"),
            Self::Addi { rd, rs1, imm } => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            Self::Mv { rd, rs1 } => write!(f, "mv {}, {}", rd, rs1),
            Self::Nop => write!(f, "nop"),
            Self::Add { rd, rs1, rs2 } => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Self::Sub { rd, rs1, rs2 } => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            Self::Jmp { target } => write!(f, "jmp {}", target),
        }
    }
}

pub type Program = Vec<Instr>;
