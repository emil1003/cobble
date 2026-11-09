use thiserror::Error;

use crate::{
    compiler::ast::*,
    interpreter::state::{Registers, State},
};

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(Instr),

    #[error("Invalid operands: {0}")]
    InvalidOperands(Instr),

    #[error("Invalid register: {0}")]
    InvalidRegister(u8),

    #[error("Attempt to interpret out-of-bounds address {0}")]
    PCOutOfBounds(u16),
}

/// Performs a wrapping addition and returns a tuple containing the result and an overflow flag.
#[inline]
fn add_with_overflow(a: u8, b: u8) -> (u8, bool) {
    match a.checked_add(b) {
        Some(s) => (s, false),
        None => (a.wrapping_add(b), true),
    }
}

/// Performs a wrapping subtraction and returns a tuple containing the result and an overflow flag.
#[inline]
fn sub_with_overflow(a: u8, b: u8) -> (u8, bool) {
    match a.checked_sub(b) {
        Some(s) => (s, false),
        None => (a.wrapping_sub(b), true),
    }
}

/// Extension trait for error-mapped register interaction
pub trait RegisterAccess {
    fn read_err(&self, reg: u8) -> Result<u8, InterpreterError>;
    fn write_err(&mut self, reg: u8, val: u8) -> Result<(), InterpreterError>;
}

impl RegisterAccess for Registers {
    #[inline(always)]
    fn read_err(&self, reg: u8) -> Result<u8, InterpreterError> {
        self.r(reg).ok_or(InterpreterError::InvalidRegister(reg))
    }

    #[inline(always)]
    fn write_err(&mut self, reg: u8, val: u8) -> Result<(), InterpreterError> {
        self.w(reg, val)
            .map_err(|_| InterpreterError::InvalidRegister(reg))
    }
}

/// Interprets an instruction, mutating a given VM state in the process.
/// Returns the next PC address (does not set it), or `None` if the interpreter should halt.
pub fn interpret(instr: &Instr, state: &mut State) -> Result<Option<u16>, InterpreterError> {
    use {Instr::*, Op::*};

    match instr {
        Halt => {
            state.flags = (true, false).into();
            Ok(None)
        }
        Addi {
            rd: Reg(rd),
            rs1: Reg(rs1),
            imm: Imm8(imm),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let (res, overflow) = add_with_overflow(a, *imm);
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), overflow).into();
            Ok(Some(state.pc + 1))
        }
        Mv {
            rd: Reg(rd),
            rs1: Reg(rs1),
        } => {
            let a = state.regs.read_err(*rs1)?;
            state.regs.write_err(*rd, a)?;
            state.flags = (a.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Nop => {
            state.flags = (true, false).into();
            Ok(Some(state.pc + 1))
        }
        Add {
            rd: Reg(rd),
            rs1: Reg(rs1),
            rs2: Reg(rs2),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let b = state.regs.read_err(*rs2)?;
            let (res, overflow) = add_with_overflow(a, b);
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), overflow).into();
            Ok(Some(state.pc + 1))
        }
        Sub {
            rd: Reg(rd),
            rs1: Reg(rs1),
            rs2: Reg(rs2),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let b = state.regs.read_err(*rs2)?;
            let (res, overflow) = sub_with_overflow(a, b);
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), overflow).into();
            Ok(Some(state.pc + 1))
        }
        Not {
            rd: Reg(rd),
            rs1: Reg(rs1),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let res = !a;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        And {
            rd: Reg(rd),
            rs1: Reg(rs1),
            rs2: Reg(rs2),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let b = state.regs.read_err(*rs2)?;
            let res = a & b;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Or {
            rd: Reg(rd),
            rs1: Reg(rs1),
            rs2: Reg(rs2),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let b = state.regs.read_err(*rs2)?;
            let res = a | b;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Xor {
            rd: Reg(rd),
            rs1: Reg(rs1),
            rs2: Reg(rs2),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let b = state.regs.read_err(*rs2)?;
            let res = a ^ b;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Andi {
            rd: Reg(rd),
            rs1: Reg(rs1),
            imm: Imm8(imm),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let res = a & imm;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Ori {
            rd: Reg(rd),
            rs1: Reg(rs1),
            imm: Imm8(imm),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let res = a | imm;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Xori {
            rd: Reg(rd),
            rs1: Reg(rs1),
            imm: Imm8(imm),
        } => {
            let a = state.regs.read_err(*rs1)?;
            let res = a ^ imm;
            state.regs.write_err(*rd, res)?;
            state.flags = (res.eq(&0u8), false).into();
            Ok(Some(state.pc + 1))
        }
        Jmp { imm: target } => match target {
            Imm12(imm) => {
                // Flags are the same as res = 0
                state.flags = (true, false).into();
                Ok(Some(*imm))
            }
            _ => Err(InterpreterError::InvalidOperands(instr.clone())),
        },
        Bz { imm: Imm12(imm) } => {
            if state.flags.zero {
                Ok(Some(*imm))
            } else {
                Ok(Some(state.pc + 1))
            }
        }
        Bnz { imm: Imm12(imm) } => {
            if state.flags.zero {
                Ok(Some(state.pc + 1))
            } else {
                Ok(Some(*imm))
            }
        }
        _ => Err(InterpreterError::InvalidInstruction(instr.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut state = State::new();
        // Basic instruction
        let instr = Instr::Addi {
            rd: Op::Reg(1),
            rs1: Op::Reg(0),
            imm: Op::Imm8(2),
        };
        assert!(interpret(&instr, &mut state).ok().unwrap().is_some());
        assert_eq!(state.regs.r(1).unwrap(), 2);

        // Halting
        let instr = Instr::Halt;
        assert!(interpret(&instr, &mut State::new()).ok().unwrap().is_none());

        // Branching
        let instr = Instr::Jmp {
            imm: Op::Imm12(0xf),
        };
        assert_eq!(interpret(&instr, &mut state).ok().unwrap(), Some(0xf));
    }

    #[test]
    fn test_interpreter_errors() {
        // Invalid operand
        let instr = Instr::Mv {
            rd: Op::Reg(0),
            rs1: Op::Imm8(0),
        };
        assert!(interpret(&instr, &mut State::new()).err().is_some());

        // Invalid register
        let instr = Instr::Addi {
            rd: Op::Reg(0),
            rs1: Op::Reg(16),
            imm: Op::Imm8(32),
        };
        assert!(interpret(&instr, &mut State::new()).err().is_some());
    }
}
