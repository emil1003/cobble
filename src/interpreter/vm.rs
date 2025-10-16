use thiserror::Error;

use crate::compiler::ast::*;

pub type InterpreterFlags = (bool, bool);

pub struct InterpreterState {
    pub pc: u16,
    pub regs: RegFile,
    pub flags: InterpreterFlags,
}

impl Default for InterpreterState {
    fn default() -> Self {
        Self {
            pc: 0u16,
            regs: RegFile([0; 15]),
            flags: (true, false),
        }
    }
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("invalid instruction: {0}")]
    InvalidInstruction(Instr),

    #[error("invalid operands: {0}")]
    InvalidOperands(Instr),

    #[error("invalid register: {0}")]
    InvalidRegister(u8),

    #[error("attempt to interpret out-of-bounds address {0}")]
    PCOutOfBounds(u16),
}

pub struct RegFile(pub [u8; 15]);

impl RegFile {
    #[inline]
    pub fn r(&self, reg: u8) -> Result<u8, InterpreterError> {
        match reg {
            0 => Ok(0),
            1..=15 => Ok(self.0[reg as usize]),
            _ => Err(InterpreterError::InvalidRegister(reg)),
        }
    }

    #[inline]
    pub fn w(&mut self, reg: u8, v: u8) -> Result<(), InterpreterError> {
        match reg {
            0 => Ok(()),
            1..=15 => {
                self.0[reg as usize] = v;
                Ok(())
            }
            _ => Err(InterpreterError::InvalidRegister(reg)),
        }
    }
}

/// Does a wrapping add, with bool set if overflowed
#[inline]
fn inbounds_add(a: u8, b: u8) -> (u8, bool) {
    match a.checked_add(b) {
        Some(s) => (s, false),
        None => (a.wrapping_add(b), true),
    }
}

/// Does a wrapping sub, with bool set if overflowed
#[inline]
fn inbounds_sub(a: u8, b: u8) -> (u8, bool) {
    match a.checked_sub(b) {
        Some(s) => (s, false),
        None => (a.wrapping_sub(b), true),
    }
}

/// Interprets an instruction, mutating a given VM state in the process.
/// Returns the next PC address (does not set it), or `None` if the interpreter should halt.
pub fn interpret(
    instr: &Instr,
    state: &mut InterpreterState,
) -> Result<Option<u16>, InterpreterError> {
    match instr {
        Instr::Halt => {
            state.flags = (true, false);
            Ok(None)
        }
        Instr::Addi {
            rd: Op::Reg(rd),
            rs1: Op::Reg(rs1),
            imm: Op::Imm8(imm),
        } => {
            let a = state.regs.r(*rs1)?;
            let (res, overflow) = inbounds_add(a, *imm);
            state.regs.w(*rd, res)?;
            state.flags = (a.eq(&0u8), overflow);
            Ok(Some(state.pc + 1))
        }
        Instr::Mv {
            rd: Op::Reg(rd),
            rs1: Op::Reg(rs1),
        } => {
            let a = state.regs.r(*rs1)?;
            state.regs.w(*rd, a)?;
            state.flags = (a.eq(&0u8), false);
            Ok(Some(state.pc + 1))
        }
        Instr::Nop => {
            state.flags = (true, false);
            Ok(Some(state.pc + 1))
        }
        Instr::Add {
            rd: Op::Reg(rd),
            rs1: Op::Reg(rs1),
            rs2: Op::Reg(rs2),
        } => {
            let a = state.regs.r(*rs1)?;
            let b = state.regs.r(*rs2)?;
            let (res, overflow) = inbounds_add(a, b);
            state.regs.w(*rd, res)?;
            state.flags = (res.eq(&0u8), overflow);
            Ok(Some(state.pc + 1))
        }
        Instr::Sub {
            rd: Op::Reg(rd),
            rs1: Op::Reg(rs1),
            rs2: Op::Reg(rs2),
        } => {
            let a = state.regs.r(*rs1)?;
            let b = state.regs.r(*rs2)?;
            let (res, overflow) = inbounds_sub(a, b);
            state.regs.w(*rd, res)?;
            state.flags = (res.eq(&0u8), overflow);
            Ok(Some(state.pc + 1))
        }
        Instr::Jmp { target } => match &target {
            Op::Imm12(imm) => {
                state.flags = (true, false);
                Ok(Some(*imm))
            }
            _ => return Err(InterpreterError::InvalidOperands(instr.clone())),
        },
        _ => Err(InterpreterError::InvalidInstruction(instr.clone())),
    }
}

#[test]
fn test_interpreter() {
    let mut state = InterpreterState::default();
    // Basic instruction
    let instr = Instr::Addi {
        rd: Op::Reg(1),
        rs1: Op::Reg(0),
        imm: Op::Imm8(2),
    };
    assert!(
        interpret(&instr, &mut state)
            .ok()
            .unwrap()
            .is_some()
    );
    assert_eq!(state.regs.r(1).unwrap(), 2);

    // Halting
    let instr = Instr::Halt;
    assert!(
        interpret(&instr, &mut InterpreterState::default())
            .ok()
            .unwrap()
            .is_none()
    );
}

#[test]
fn test_interpreter_errors() {
    // Invalid operand
    let instr = Instr::Mv {
        rd: Op::Reg(0),
        rs1: Op::Imm8(0),
    };
    assert!(
        interpret(&instr, &mut InterpreterState::default())
            .err()
            .is_some()
    );

    // Invalid register
    let instr = Instr::Addi {
        rd: Op::Reg(0),
        rs1: Op::Reg(16),
        imm: Op::Imm8(32),
    };
    assert!(
        interpret(&instr, &mut InterpreterState::default())
            .err()
            .is_some()
    );
}
