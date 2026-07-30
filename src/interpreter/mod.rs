pub mod state;
pub mod vm;

use crate::{compiler::ast::*, interpreter::state::State};
use vm::*;

#[derive(Debug, Default)]
pub struct Stats {
    pub interpreted: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Interprets a given program, then returns a tuple of
/// the encountered error (if any) + the final state of
/// the machine.
pub fn interpret_program(
    prg: Program,
    initial_state: Option<State>,
) -> (Option<InterpreterError>, State, Stats) {
    // // Use given initial state, or default
    let mut state = initial_state.unwrap_or_default();
    let mut stats = Stats::new();

    let status = loop {
        // Get instruction at given PC
        let instr = match prg.get(state.pc as usize) {
            Some(i) => i,
            None => {
                // PC points to out-of-bounds instruction
                break Some(InterpreterError::PCOutOfBounds(state.pc));
            }
        };

        // Interpret instruction
        match interpret(instr, &mut state) {
            Ok(new_pc) => {
                stats.interpreted += 1;
                // Set new PC
                match new_pc {
                    Some(a) => state.pc = a,
                    None => break None,
                }
            }
            Err(err) => {
                break Some(err);
            }
        }
    };

    // Return state
    (status, state, stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interpret_sample_program() {
        // 2 + 2
        let prg = vec![
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
        let (err, state, stats) = interpret_program(prg, None);

        // No error
        assert!(err.is_none());
        // r3 = r1 + r2 = 4
        assert_eq!(state.regs.r(3).unwrap(), 4);
        // 4 total instructions
        assert_eq!(stats.interpreted, 4);
    }

    #[test]
    fn test_program_error() {
        // Instruction out-of-bounds
        let prg = vec![Instr::Jmp { imm: Op::Imm12(42) }, Instr::Halt];
        let (err, _state, stats) = interpret_program(prg, None);

        // Out-of-bounds error
        assert!(err.is_some());
        // 1 instruction (lookup of instruction 42 fails)
        assert_eq!(stats.interpreted, 1);
    }
}
