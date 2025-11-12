use std::fmt::*;

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

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    // Meta operations
    /// Symbol placeholder (to be stripped)
    Label(String),
    /// Terminate program
    Halt,
    /// No operation
    Nop,
    // Unary operations
    /// Move (rd = rs1)
    Mv { rd: Op, rs1: Op },
    /// Bitwise NOT (rd = !rs1)
    Not { rd: Op, rs1: Op },
    // Binary operations
    /// Addition (rd = rs1 + rs2)
    Add { rd: Op, rs1: Op, rs2: Op },
    /// Subtraction (rd = rs1 - rs2)
    Sub { rd: Op, rs1: Op, rs2: Op },
    /// Bitwise AND (rd = rs1 & rs2)
    And { rd: Op, rs1: Op, rs2: Op },
    /// Bitwise OR (rd = rs1 | rs2)
    Or { rd: Op, rs1: Op, rs2: Op },
    /// Bitwise XOR (rd = rs1 ^ rs2)
    Xor { rd: Op, rs1: Op, rs2: Op },
    // Immediate operations
    /// Immediate addition (rd = rs1 + imm)
    Addi { rd: Op, rs1: Op, imm: Op },
    /// Immediate bitwise AND (rd = rs1 & imm)
    Andi { rd: Op, rs1: Op, imm: Op },
    /// Immediate bitwise OR (rd = rs1 | imm)
    Ori { rd: Op, rs1: Op, imm: Op },
    /// Immediate bitwise XOR (rd = rs1 ^ imm)
    Xori { rd: Op, rs1: Op, imm: Op },
    // Branching operations
    /// Jump to address (pc = imm)
    Jmp { imm: Op },
    /// Jump to address (pc = imm) if flag zero
    Bz { imm: Op },
    /// Jump to address (pc = imm) if not flag zero
    Bnz { imm: Op },
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Label(l) => write!(f, "{}:", l),
            Self::Nop => write!(f, "nop"),
            Self::Halt => write!(f, "halt"),

            Self::Mv { rd, rs1 } => write!(f, "mv {}, {}", rd, rs1),
            Self::Not { rd, rs1 } => write!(f, "not {}, {}", rd, rs1),

            Self::Add { rd, rs1, rs2 } => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Self::Sub { rd, rs1, rs2 } => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            Self::And { rd, rs1, rs2 } => write!(f, "and {}, {}, {}", rd, rs1, rs2),
            Self::Or { rd, rs1, rs2 } => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            Self::Xor { rd, rs1, rs2 } => write!(f, "xor {}, {}, {}", rd, rs1, rs2),

            Self::Addi { rd, rs1, imm } => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            Self::Andi { rd, rs1, imm } => write!(f, "andi {}, {}, {}", rd, rs1, imm),
            Self::Ori { rd, rs1, imm } => write!(f, "ori {}, {}, {}", rd, rs1, imm),
            Self::Xori { rd, rs1, imm } => write!(f, "xori {}, {}, {}", rd, rs1, imm),

            Self::Jmp { imm } => write!(f, "jmp {}", imm),
            Self::Bz { imm } => write!(f, "bz {}", imm),
            Self::Bnz { imm } => write!(f, "bnz {}", imm),
        }
    }
}

/// The program type (being a list of instructions)
pub type Program = Vec<Instr>;
