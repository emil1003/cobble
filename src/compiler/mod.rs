pub mod ast;
pub mod parser;
pub mod symbol;

use ast::Program;
use parser::parse_program;
use symbol::{replace_symbols, strip_symbols};

/// Compiles a given program from a string into AST,
/// with symbols stripped and replaced.
pub fn compile_program(src: &str) -> Result<Program, String> {
    // Parse program into AST
    let prg = parse_program(src).map_err(|e| format!("Parse error: {}", e))?;

    // Strip symbols
    let (stripped, symbols) = strip_symbols(&prg).map_err(|e| format!("Symbol error: {}", e))?;

    // Replace symbol references
    let replaced =
        replace_symbols(&stripped, &symbols).map_err(|e| format!("Symbol error: {}", e))?;

    Ok(replaced)
}

#[test]
fn test_compiler() {}
