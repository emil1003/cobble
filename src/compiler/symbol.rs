use std::collections::HashMap;

use crate::compiler::ast::*;

/// Table holding a mapping between symbols and their addresses
pub type SymbolTable = HashMap<String, u16>;

/// Strips symbol instructions from a given program.
pub fn strip_symbols(prg: &Program) -> (Program, SymbolTable) {
    let mut out: Program = Vec::with_capacity(prg.len());
    let mut symbols: SymbolTable = HashMap::default();
    let mut pc = 0u16;

    prg.iter().for_each(|instr| {
        match instr {
            Instr::Label(label) => {
                // Record symbol
                symbols.insert(label.to_string(), pc);
            }
            other => {
                out.push(other.clone());
                pc += 1;
            }
        }
    });

    (out, symbols)
}

#[inline]
fn lookup_symbol(symbol: &String, table: &SymbolTable) -> Result<u16, String> {
    match table.get(symbol) {
        Some(a) => Ok(*a),
        None => Err(format!("No such symbol in table: {}", symbol)),
    }
}

/// Replaces symbols in operands with addresses from table.
pub fn replace_symbols(prg: &Program, symbols: &SymbolTable) -> Result<Program, String> {
    let mut out: Program = Vec::with_capacity(prg.len());

    for instr in prg {
        match instr {
            Instr::Label(s) => {
                // Input program not fully stripped
                return Err(format!("Encountered unstripped symbol \"{}\" ", s));
            }
            Instr::Jmp {
                target: Op::Label(symbol),
            } => {
                let addr = lookup_symbol(symbol, symbols)?;
                out.push(Instr::Jmp {
                    target: Op::Imm12(addr),
                })
            }
            _ => out.push(instr.clone()),
        }
    }

    Ok(out)
}

#[test]
fn test_strip_symbols() {
    let prg = vec![Instr::Label("start".to_string())];

    let (stripped, symbols) = strip_symbols(&prg);
    assert_eq!(stripped.len(), 0);
    assert_eq!(symbols.get("start").unwrap(), &0u16);
}

#[test]
fn test_replace_symbols() {
    let prg = vec![Instr::Jmp {
        target: Op::Label("start".to_string()),
    }];
    let mut symbols: SymbolTable = HashMap::default();
    symbols.insert("start".to_string(), 0);

    let replaced = replace_symbols(&prg, &symbols).ok().unwrap();

    assert_eq!(replaced[0], Instr::Jmp { target: Op::Imm12(0) });
}
