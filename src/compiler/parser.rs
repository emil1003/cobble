use crate::compiler::ast::*;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, hex_digit1, multispace0},
    combinator::{map, map_res},
    sequence::{preceded, terminated},
};
use std::str::FromStr;

/// Parse a u16 numerical value, hex or decimal
fn parse_u16(input: &str) -> IResult<&str, u16> {
    alt((
        map_res(preceded(tag("0x"), hex_digit1), |hex| {
            u16::from_str_radix(hex, 16)
        }),
        map_res(digit1, |digits| u16::from_str(digits)),
    ))
    .parse(input)
}

/// Parse a u8 numerical value, hex or decimal
fn parse_u8(input: &str) -> IResult<&str, u8> {
    alt((
        map_res(preceded(tag("0x"), hex_digit1), |hex| {
            u8::from_str_radix(hex, 16)
        }),
        map_res(digit1, |digits| u8::from_str(digits)),
    ))
    .parse(input)
}

/// Parse a register name like "r" → Operand::Reg(2)
fn parse_reg(input: &str) -> IResult<&str, Op> {
    map(preceded(char('r'), parse_u8), |val| Op::Reg(val)).parse(input)
}

/// Parse an 8-bit immediate like "42" → Operand::Imm8(42)
fn parse_imm8(input: &str) -> IResult<&str, Op> {
    map(parse_u8, |v: u8| Op::Imm8(v)).parse(input)
}

/// Parse a 12-bit immediate like "42" → Operand::Imm12(42)
fn parse_imm12(input: &str) -> IResult<&str, Op> {
    map_res(parse_u16, |val| {
        // Ensure is within 12 bits
        if val <= 0xFFF {
            Ok(Op::Imm12(val))
        } else {
            Err(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TooLarge,
            ))
        }
    })
    .parse(input)
}

/// Parse a label reference like “LOOP”
fn parse_label_ref(input: &str) -> IResult<&str, Op> {
    map(alpha1, |s: &str| Op::Label(s.to_string())).parse(input)
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    let (rest, label) = terminated(alpha1, char(':')).parse(input)?;
    Ok((rest, label))
}

/// Parse a full instruction line.
///   e.g. “addi x4, x1, 5”
fn parse_line(input: &str) -> IResult<&str, Vec<Instr>> {
    // Consume leading whitespace
    let (input, _) = multispace0(input)?;

    // Try to parse a label first (label lines end with ':')
    if let Ok((rest, label)) = parse_label(input) {
        return Ok((rest, vec![Instr::Label(label.to_string())]));
    }

    // Otherwise we have an opcode + operands.
    let (input, opcode) = terminated(alpha1, multispace0).parse(input)?;

    match opcode.to_uppercase().as_str() {
        "HALT" => Ok((input, vec![Instr::Halt])),
        "ADDI" => {
            let (input, (rd, rs1, imm)) = (
                parse_reg,
                preceded((char(','), multispace0), parse_reg),
                preceded((char(','), multispace0), parse_imm8),
            )
                .parse(input)?;
            Ok((input, vec![Instr::Addi { rd, rs1, imm }]))
        }
        "MV" => {
            let (input, (rd, rs1)) =
                (parse_reg, preceded((char(','), multispace0), parse_reg)).parse(input)?;
            Ok((input, vec![Instr::Mv { rd, rs1 }]))
        }
        "NOP" => Ok((input, vec![Instr::Nop])),
        "ADD" => {
            let (input, (rd, rs1, rs2)) = (
                parse_reg,
                preceded((char(','), multispace0), parse_reg),
                preceded((char(','), multispace0), parse_reg),
            )
                .parse(input)?;
            Ok((input, vec![Instr::Add { rd, rs1, rs2 }]))
        }
        "SUB" => {
            let (input, (rd, rs1, rs2)) = (
                parse_reg,
                preceded((char(','), multispace0), parse_reg),
                preceded((char(','), multispace0), parse_reg),
            )
                .parse(input)?;
            Ok((input, vec![Instr::Sub { rd, rs1, rs2 }]))
        }
        "JMP" => {
            let (input, target) = alt((parse_label_ref, parse_imm12)).parse(input)?;
            Ok((input, vec![Instr::Jmp { target }]))
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(
            opcode,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

/// Parse an entire file (separated by newlines).
pub fn parse_program(src: &str) -> Result<Program, String> {
    let mut program = Vec::new();

    for (lineno, line) in src.lines().enumerate() {
        // skip blank or comment lines
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }

        match parse_line(line) {
            Ok((_, mut instr)) => program.append(&mut instr),
            Err(s) => {
                return Err(format!(
                    "Parse error on line {}: {}",
                    lineno + 1,
                    s.to_string()
                ));
            }
        }
    }

    Ok(program)
}

#[test]
fn test_parser() {
    // Simple Halt
    let mut input = "halt";
    assert_eq!(parse_line(input).ok().unwrap(), ("", vec![Instr::Halt]));

    // Basic addi r0, r0, 0
    input = "addi r0, r0, 0";
    assert_eq!(
        parse_line(input).ok().unwrap(),
        (
            "",
            vec![Instr::Addi {
                rd: Op::Reg(0),
                rs1: Op::Reg(0),
                imm: Op::Imm8(0),
            }]
        )
    );

    // Malformed mv
    input = "mv r0, 0";
    assert!(parse_line(input).err().is_some());

    // Label
    input = "loop:";
    assert_eq!(
        parse_line(input).ok().unwrap(),
        ("", vec![Instr::Label("loop".to_string())])
    );

    // Jump to label
    input = "jmp loop";
    assert_eq!(
        parse_line(input).ok().unwrap(),
        (
            "",
            vec![Instr::Jmp {
                target: Op::Label("loop".to_string())
            }]
        )
    )
}
