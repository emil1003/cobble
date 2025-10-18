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
            Self::Label(l) => write!(f, "{}", l),
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

    /// Bitwise NOT (unary)
    Not { rd: Op, rs1: Op },
    /// Bitwise AND
    And { rd: Op, rs1: Op, rs2: Op },
    /// Bitwise OR
    Or { rd: Op, rs1: Op, rs2: Op },
    /// Bitwise XOR
    Xor { rd: Op, rs1: Op, rs2: Op },

    /// AND with an immediate value
    Andi { rd: Op, rs1: Op, imm: Op },
    /// OR with an immediate value
    Ori { rd: Op, rs1: Op, imm: Op },
    /// XOR with an immediate value
    Xori { rd: Op, rs1: Op, imm: Op },

    /// Jump to target address (imm12)
    Jmp { target: Op },
    /// Jump to target address (imm12) if flag zero
    Bz { target: Op },
    /// Jump to target address (imm12) if not flag zero
    Bnz { target: Op },
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
            
            Self::Not { rd, rs1 } => write!(f, "not {}, {}", rd, rs1),
            Self::And { rd, rs1, rs2 } => write!(f, "and {}, {}, {}", rd, rs1, rs2),
            Self::Or { rd, rs1, rs2 } => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            Self::Xor { rd, rs1, rs2 } => write!(f, "xor {}, {}, {}", rd, rs1, rs2),

            Self::Andi { rd, rs1, imm } => write!(f, "andi {}, {}, {}", rd, rs1, imm),
            Self::Ori { rd, rs1, imm } => write!(f, "ori {}, {}, {}", rd, rs1, imm),
            Self::Xori { rd, rs1, imm } => write!(f, "xori {}, {}, {}", rd, rs1, imm),

            Self::Jmp { target } => write!(f, "jmp {}", target),
            Self::Bz { target } => write!(f, "bz {}", target),
            Self::Bnz { target } => write!(f, "bnz {}", target),
        }
    }
}

pub type Program = Vec<Instr>;
