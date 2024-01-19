use strawberryvm::prelude::*;

use std::env;
use std::path::Path;

use std::fs::read_to_string;
use std::io::stdout;
use std::io::Write;

fn parse_numeric(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let first = s.chars().next().unwrap();
    let (num, radix) = match first {
        '$' => (&s[1..], 16),
        '%' => (&s[1..], 2),
        _ => (s, 10),
    };

    Ok(u8::from_str_radix(num, radix)?)
}

fn parse_register(s: &str) -> Result<Register, Box<dyn std::error::Error>> {
    match s {
        "A" => Ok(Register::A),
        "B" => Ok(Register::B),
        "C" => Ok(Register::C),
        _ => Err(format!("Unknown register {s}").into()),
    }
}

fn assert_length(parts: &Vec<&str>, n: usize) -> Result<(), Box<dyn std::error::Error>> {
    match parts.len() == n {
        true => Ok(()),
        false => Err(format!("Expected {} got {}", n, parts.len()).into()),
    }
}

// TODO: Very good candidate for derive macro.
fn handle_line(parts: Vec<&str>) -> Result<Instruction, Box<dyn std::error::Error>> {
    let opcode = parts[0]
        .parse()
        .map_err(|_| format!("Unknown opcode: {}", parts[0]))?;

    match opcode {
        OpCode::Push => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Push(parse_numeric(parts[1])?))
        }

        OpCode::AddStack => {
            assert_length(&parts, 1)?;
            Ok(Instruction::AddStack)
        }

        OpCode::AddReg => {
            let (r1, r2) = (parse_register(parts[1])?, parse_register(parts[2])?);
            Ok(Instruction::AddReg(r1, r2))
        }

        OpCode::PopReg => {
            assert_length(&parts, 2)?;
            Ok(Instruction::PopReg(parse_register(parts[1])?))
        }

        OpCode::PushReg => {
            assert_length(&parts, 2)?;
            Ok(Instruction::PushReg(parse_register(parts[1])?))
        }

        OpCode::Signal => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Signal(parse_numeric(parts[1])?))
        }

        OpCode::Nop => {
            assert_length(&parts, 1)?;
            Ok(Instruction::Nop)
        }

        OpCode::Jmp => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Jmp(parse_numeric(parts[1])?))
        }
    }
}

/// Usage:
/// ./asm file.asm
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 2);

    let file = read_to_string(Path::new(&args[1]))?;
    let lines: Vec<&str> = file.lines().collect();

    let mut output: Vec<u8> = Vec::new();

    for line in lines.iter().filter(|line| line.ends_with(':')) {
        dbg!(line);
    }

    for line in lines
        .iter()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with(';'))
        .filter(|line| !line.ends_with(':'))
    {
        let parts: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();

        if parts.is_empty() {
            continue;
        }

        let instruction = handle_line(parts)?;
        let raw_instruction: u16 = instruction.encode_u16();

        let lower = (raw_instruction & 0xff) as u8;
        let upper = (raw_instruction >> 8) as u8;

        output.push(lower);
        output.push(upper);
    }

    stdout().lock().write_all(&output)?;

    Ok(())
}
