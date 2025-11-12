use std::panic;

use thiserror::Error;

use crate::compiler::ast::*;

macro_rules! make_instr {
    ($op:expr, $( $field:ident => $val:expr ),* ) => {{
        let mut b = InstrBuilder::new().opcode($op);
        $(
            b = b.$field($val);
        )*
        b.finalize()
    }};
}

/// A 24‑bit instruction builder.
///
/// Internally it stores the word as a `u32` so we can safely shift and
/// mask without overflowing.  The public API is a *chainable* set of
/// methods that set individual fields.
#[derive(Debug, Default, Clone, Copy)]
pub struct InstrBuilder {
    word: u32, // only bits 0..23 are used
}

impl InstrBuilder {
    /// Start with a fresh word
    #[inline]
    pub fn new() -> Self {
        Self { word: 0 }
    }

    /// Set a 6‑bit opcode (bits 0‑5)
    #[inline]
    pub fn opcode(mut self, op: u8) -> Self {
        self.word = (self.word & !0x3F) | ((op as u32) & 0x3F);
        self
    }

    /// Set a 2‑bit fun2 (bits 6‑7)
    #[inline]
    pub fn fun2(mut self, fun: u8) -> Self {
        self.word = (self.word & !0xC0) | (((fun as u32) & 0x03) << 6);
        self
    }

    /// Set a 4‑bit fun4 (bits 20‑23)
    #[inline]
    pub fn fun4(mut self, fun: u8) -> Self {
        self.word = (self.word & !0xF0000) | (((fun as u32) & 0x0F) << 20);
        self
    }

    /// Set a 4‑bit register field (rd, rs1, rs2)
    #[inline]
    fn set_reg(mut self, value: u8, shift: u8) -> Self {
        self.word = (self.word & !(0x0F << shift)) | (((value as u32) & 0x0F) << shift);
        self
    }

    /// Rd – bits 8‑11
    #[inline]
    pub fn rd(self, reg: u8) -> Self {
        self.set_reg(reg, 8)
    }

    /// Rs1 – bits 12‑15
    #[inline]
    pub fn rs1(self, reg: u8) -> Self {
        self.set_reg(reg, 12)
    }

    /// Rs2 – bits 16‑19 (or imm8, see below)
    #[inline]
    pub fn rs2(self, reg: u8) -> Self {
        self.set_reg(reg, 16)
    }

    /// imm8 – bits 16‑23
    #[inline]
    pub fn imm8(mut self, imm: u8) -> Self {
        self.word = (self.word & !0xFF0000) | ((imm as u32) << 16);
        self
    }

    /// imm12 – bits 12‑23
    #[inline]
    pub fn imm12(mut self, imm: u16) -> Self {
        self.word = (self.word & !0xFFF000) | ((imm as u32) << 12);
        self
    }

    /// Return the final 24‑bit word.
    #[inline]
    pub fn finalize(self) -> u32 {
        self.word & 0xFFFFFF
    }
}

pub type MachineCode = u32;

#[derive(Debug, Error)]
pub enum AsmError {
    #[error("unknown instruction: {0}")]
    UnknownInstruction(Instr),

    #[error("label not found: {0}")]
    UndefinedLabel(String),

    #[error("invalid register: {0}")]
    InvalidRegister(String),

    #[error("invalid operand: {0:?}")]
    InvalidOperand(String),

    #[error("overflow in immediate: {0}")]
    ImmOverflow(u16),
}

pub fn encode_program(instrs: &[Instr]) -> Result<Vec<MachineCode>, AsmError> {
    let mut out = Vec::new();
    for instr in instrs {
        let word = encode(instr)?;
        out.push(word);
    }
    Ok(out)
}

fn encode(instr: &Instr) -> Result<MachineCode, AsmError> {
    match instr {
        Instr::Label(_) => panic!("Cannot encode labels"),
        Instr::Halt => Ok(0),
        Instr::Addi { rd, rs1, imm } => match (rd, rs1, imm) {
            (Op::Reg(rd), Op::Reg(rs1), Op::Imm8(imm)) => {
                Ok(make_instr!(0b000001, rd => *rd, rs1 => *rs1, imm8 => *imm))
            }
            _ => Err(AsmError::InvalidOperand("".to_string())),
        },
        Instr::Mv { rd, rs1 } => match (rd, rs1) {
            (Op::Reg(rd), Op::Reg(rs1)) => Ok(make_instr!(0b000001, rd => *rd, rs1 => *rs1)),
            _ => Err(AsmError::InvalidOperand("".to_string())),
        },
        Instr::Nop => Ok(make_instr!(0b000001, rd => 0, rs1 => 0)),
        Instr::Add { rd, rs1, rs2 } => match (rd, rs1, rs2) {
            (Op::Reg(rd), Op::Reg(rs1), Op::Reg(rs2)) => {
                Ok(make_instr!(0b000010, rd => *rd, rs1 => *rs1, rs2 => *rs2))
            }
            _ => Err(AsmError::InvalidOperand("".to_string())),
        },
        Instr::Sub { rd, rs1, rs2 } => match (rd, rs1, rs2) {
            (Op::Reg(rd), Op::Reg(rs1), Op::Reg(rs2)) => {
                Ok(make_instr!(0b000011, rd => *rd, rs1 => *rs1, rs2 => *rs2))
            }
            _ => Err(AsmError::InvalidOperand("".to_string())),
        },
        Instr::Jmp { imm: target } => {
            match &target {
                Op::Imm12(_) => {
                    // Direct address
                    Ok(0x0)
                }
                _ => Err(AsmError::InvalidOperand("".to_string())),
            }
        }
        _ => Err(AsmError::UnknownInstruction(instr.clone())),
    }
}

#[test]
fn test_encode() {
    // Halt instruction (all 0's)
    assert_eq!(encode(&Instr::Halt).unwrap(), 0);

    // Basic addi r0, r0, 0
    let code = encode(&Instr::Addi {
        rd: Op::Reg(0),
        rs1: Op::Reg(0),
        imm: Op::Imm8(0),
    })
    .unwrap();
    assert_eq!(code, 0b00000000_0000_0000_00_000001);

    // Nop instruction (equal to addi r0, r0, 0)
    assert_eq!(encode(&Instr::Nop).unwrap(), code)
}
