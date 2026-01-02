use std::collections::HashMap;

use crate::compiler::ast::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("Duplicate symbol declaration: {0}")]
    DuplicateSymbol(String),

    #[error("No such symbol in table: {0}")]
    NoSuchSymbol(String),

    #[error("Unstripped symbol encountered: {0}")]
    UnstrippedSymbol(String),
}

/// Table holding a mapping between symbols and their addresses
pub type SymbolTable = HashMap<String, u16>;

/// Strips symbol instructions from a given program.
pub fn strip_symbols(prg: &Program) -> Result<(Program, SymbolTable), SymbolError> {
    // Stripped program length is <= input program
    let mut out: Program = Vec::with_capacity(prg.len());

    let mut symbols: SymbolTable = HashMap::new();
    let mut pc = 0u16;

    for instr in prg {
        match instr {
            Instr::Label(label) => {
                // If symbol already seen, error
                if symbols.contains_key(label) {
                    return Err(SymbolError::DuplicateSymbol(label.to_string()));
                }

                // Record symbol
                symbols.insert(label.clone(), pc);
            }
            _ => {
                out.push(instr.clone());
                pc += 1;
            }
        }
    }

    Ok((out, symbols))
}

/// Getter for an address in symbol table.
#[inline]
fn lookup_address(symbol: &str, table: &SymbolTable) -> Result<u16, SymbolError> {
    match table.get(symbol) {
        Some(a) => Ok(*a),
        None => Err(SymbolError::NoSuchSymbol(symbol.to_string())),
    }
}

/// Replaces symbols in operands with addresses from table.
pub fn replace_symbols(prg: &Program, symbols: &SymbolTable) -> Result<Program, SymbolError> {
    // Resulting program length will be == input program length
    let mut out: Program = Vec::with_capacity(prg.len());

    for instr in prg {
        match instr {
            Instr::Label(s) => {
                // Input program not fully stripped
                return Err(SymbolError::UnstrippedSymbol(s.to_string()));
            }
            Instr::Jmp {
                imm: Op::Label(symbol),
            } => {
                let addr = lookup_address(symbol, symbols)?;
                out.push(Instr::Jmp {
                    imm: Op::Imm12(addr),
                })
            }
            Instr::Bnz {
                imm: Op::Label(symbol),
            } => {
                let addr = lookup_address(symbol, symbols)?;
                out.push(Instr::Bnz {
                    imm: Op::Imm12(addr),
                })
            }
            Instr::Bz {
                imm: Op::Label(symbol),
            } => {
                let addr = lookup_address(symbol, symbols)?;
                out.push(Instr::Bz {
                    imm: Op::Imm12(addr),
                })
            }
            _ => out.push(instr.clone()),
        }
    }

    Ok(out)
}

#[test]
fn test_strip_symbols() {
    let prg = vec![Instr::Label("start".to_string()), Instr::Halt];

    let (stripped, symbols) = strip_symbols(&prg).unwrap();
    assert_eq!(stripped.len(), 1);
    assert_eq!(symbols.get("start").unwrap(), &0u16);

    // Program with duplicate labels
    let prg = vec![
        Instr::Label("start".to_string()),
        Instr::Label("start".to_string()),
    ];
    let res = strip_symbols(&prg);
    assert!(res.is_err());
}

#[test]
fn test_replace_symbols() {
    let prg = vec![
        Instr::Jmp {
            imm: Op::Label("start".to_string()),
        },
        Instr::Halt,
    ];
    let mut symbols: SymbolTable = HashMap::default();
    symbols.insert("start".to_string(), 0);

    let replaced = replace_symbols(&prg, &symbols).ok().unwrap();

    assert_eq!(replaced[0], Instr::Jmp { imm: Op::Imm12(0) });

    // Unstripped program
    let prg = vec![Instr::Label("start".to_string()), Instr::Halt];
    assert!(replace_symbols(&prg, &symbols).err().is_some());
}
