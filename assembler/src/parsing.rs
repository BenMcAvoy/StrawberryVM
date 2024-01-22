use std::str::FromStr;

use strawberryvm::prelude::*;

#[derive(Debug)]
pub enum JamParseError {
    InvalidOpCode(String, usize),
}

/// Used to parse a numeric based on whether it is binary,
/// decimal, or hexadecimal.
pub fn parse_numeric(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let first = s.chars().next().unwrap();
    let (num, radix) = match first {
        '$' => (&s[1..], 16),
        '%' => (&s[1..], 2),
        _ => (s, 10),
    };

    Ok(u8::from_str_radix(num, radix)?)
}

// TODO: Very good candidate for derive macro.
/// Used to parse a register from a string into an actual
/// register than can be encoded into binary
pub fn parse_register(s: &str) -> Result<Register, Box<dyn std::error::Error>> {
    let s = s.to_lowercase();

    match s.as_str() {
        "a" => Ok(Register::A),
        "b" => Ok(Register::B),
        "c" => Ok(Register::C),
        _ => Err(format!("Unknown register {s}").into()),
    }
}


pub fn validate_jam(lines: &[&str]) -> Result<(), JamParseError> {
    let substituted = lines.iter().map(|l| match l.starts_with(';') {
        true => "",
        false => l,
    });

    for (line_number, line) in substituted.enumerate() {
        let opcode = match line.split_whitespace().next() {
            Some(v) => v,
            None => continue,
        };

        if OpCode::from_str(opcode).is_err() {
            return Err(JamParseError::InvalidOpCode(opcode.to_string(), line_number + 1))
        }
    }

    Ok(())
}
