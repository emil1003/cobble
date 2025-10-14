pub mod assembler;
pub mod compiler;
pub mod interpreter;

use assembler::encoder::encode_program;
use compiler::parser::parse_program;

use compiler::ast::Instr;

pub fn assemble(src: &str) -> Result<Vec<u32>, String> {
    let instrs = parse_program(src).map_err(|e| format!("Parse error: {}", e))?;

    println!("len {}", instrs.len());

    let mut _addr = 0u32;
    for instr in &instrs {
        match instr {
            Instr::Label(_) => {}
            _ => {
                _addr += 1;
            }
        }
    }

    encode_program(&instrs).map_err(|e| format!("Encode error: {}", e))
}
