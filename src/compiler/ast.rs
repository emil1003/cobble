use std::fmt::{self, Display};

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

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node<{}>", self)
    }
}

pub type Program = Vec<Instr>;
