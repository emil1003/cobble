pub mod vm;

use crate::compiler::ast::*;
use vm::*;

/// Interprets a given program, then returns a tuple of
/// the encountered error (if any) + the final state of
/// the machine.
pub fn interpret_program(
    prg: Program,
    initial_state: Option<InterpreterState>,
) -> (Option<InterpreterError>, InterpreterState) {
    // // Use given initial state, or default
    let mut state = initial_state.unwrap_or_default();

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
    (status, state)
}

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
    let (status, state) = interpret_program(prg, None);
    assert!(status.is_none());
    assert_eq!(state.regs.r(3).unwrap(), 4);
}
