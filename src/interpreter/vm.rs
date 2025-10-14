use thiserror::Error;

use crate::compiler::ast::*;

struct RegFile([u8; 15]);

impl RegFile {
    #[inline]
    fn r(&self, reg: u8) -> Result<u8, VMError> {
        match reg {
            0 => Ok(0),
            1..=15 => Ok(self.0[reg as usize]),
            _ => Err(VMError::InvalidRegister(reg)),
        }
    }

    #[inline]
    fn w(&mut self, reg: u8, v: u8) -> Result<(), VMError> {
        match reg {
            0 => Ok(()),
            1..=15 => {
                self.0[reg as usize] = v;
                Ok(())
            }
            _ => Err(VMError::InvalidRegister(reg)),
        }
    }
}

/// Does a wrapping add, with bool set if overflowed
fn inbounds_add(a: u8, b: u8) -> (u8, bool) {
    match a.checked_add(b) {
        Some(s) => (s, false),
        None => (a.wrapping_add(b), true),
    }
}

#[derive(Debug, Error)]
pub enum VMError {
    #[error("invalid operand: {0}")]
    InvalidOperand(String),

    #[error("invalid register: {0}")]
    InvalidRegister(u8),
}

pub struct VM {
    pc: u16,
    regs: RegFile,
    // flags: Vec<bool>,
}

impl VM {
    pub fn interpret(&mut self, instr: &Instr) -> Result<Option<u16>, VMError> {
        let _overflow: bool;
        let _res: u8;

        match instr {
            Instr::Label(_) => (),
            Instr::Halt => {
                self.pc = u16::MAX;
            }
            Instr::Addi { rd, rs1, imm } => match (rd, rs1, imm) {
                (Op::Reg(rd), Op::Reg(rs1), Op::Imm8(imm)) => {
                    let a = self.regs.r(*rs1)?;
                    let (res, _overflow) = inbounds_add(a, *imm as u8);
                    self.regs.w(*rd, res)?
                }
                _ => return Err(VMError::InvalidOperand("".to_string())),
            },
            Instr::Mv { rd, rs1 } => match (rd, rs1) {
                (Op::Reg(rd), Op::Reg(rs1)) => {
                    let a = self.regs.r(*rs1)?;
                    self.regs.w(*rd, a)?
                }
                _ => return Err(VMError::InvalidOperand("".to_string())),
            },
            Instr::Nop => (),
            Instr::Add { rd, rs1, rs2 } => match (rd, rs1, rs2) {
                (Op::Reg(rd), Op::Reg(rs1), Op::Reg(rs2)) => {
                    let a = self.regs.r(*rs1)?;
                    let b = self.regs.r(*rs2)?;
                    self.regs.w(*rd, a.wrapping_add(b))?
                }
                _ => return Err(VMError::InvalidOperand("".to_string())),
            },
            Instr::Sub { rd, rs1, rs2 } => match (rd, rs1, rs2) {
                (Op::Reg(rd), Op::Reg(rs1), Op::Reg(rs2)) => {
                    let a = self.regs.r(*rs1)?;
                    let b = self.regs.r(*rs2)?;
                    self.regs.w(*rd, a.wrapping_sub(b))?
                }
                _ => return Err(VMError::InvalidOperand("".to_string())),
            },
            Instr::Jmp { target } => match &target {
                Op::Imm12(a) => {
                    self.pc = *a;
                }
                Op::Label(_) => {}
                _ => return Err(VMError::InvalidOperand("".to_string())),
            },
        }

        // Interpreted correctly, no jump address
        Ok(None)
    }
}

pub fn interpret_program(instrs: &Program) -> Result<VM, VMError> {
    let mut vm = VM {
        pc: 0,
        regs: RegFile([0; 15]),
    };

    for instr in instrs {
        if let Some(jmp) = vm.interpret(&instr)? {
            // Jump to given progam address
            vm.pc = jmp;
            continue;
        }

        // If max instruction address reached, terminate
        if vm.pc == u16::MAX {
            return Ok(vm);
        }

        // Increment program counter
        vm.pc += 1;
    }

    Ok(vm)
}

#[test]
fn test_basic_program() {
    // addi  r1, r0, 2
    // addi  r2, r0, 2
    // add   r3, r1, r2
    // halt
    let program = vec![
        Instr::Addi {
            rd: Op::Reg(1),
            rs1: Op::Reg(0),
            imm: Op::Imm8(2),
        },
        Instr::Addi {
            rd: Op::Reg(2),
            rs1: Op::Reg(0),
            imm: Op::Imm8(2),
        },
        Instr::Add {
            rd: Op::Reg(3),
            rs1: Op::Reg(1),
            rs2: Op::Reg(2),
        },
        Instr::Halt,
    ];

    let vm = interpret_program(&program).unwrap();
    // Result in register 3 should be 4
    assert_eq!(vm.regs.r(3).ok().unwrap(), 4);
}

#[test]
fn test_invalids() {
    // Invalid operand
    let program = vec![Instr::Mv {
        rd: Op::Reg(0),
        rs1: Op::Imm8(0),
    }];
    assert!(interpret_program(&program).err().is_some());

    // Invalid register
    let program = vec![Instr::Addi {
        rd: Op::Reg(0),
        rs1: Op::Reg(16),
        imm: Op::Imm8(32),
    }];
    assert!(interpret_program(&program).err().is_some())
}
