use std::{error::Error, str::FromStr};

use strawberryvm::prelude::*;

#[derive(Debug)]
pub enum JamParseError {
    InvalidOpCode(String, usize),
    Empty(usize),
}

impl Error for JamParseError {}

impl std::fmt::Display for JamParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JamParseError::InvalidOpCode(invalid, line) => {
                write!(f, "Error at {invalid} on line {line}")
            }

            JamParseError::Empty(line) => {
                write!(f, "Error, empty line {line}")
            }
        }
    }
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

pub fn validate_line(line: &str, index: usize) -> Result<(), JamParseError> {
    let opcode = match line.split_whitespace().next() {
        Some(v) => v,
        None => return Err(JamParseError::InvalidOpCode(line.to_string(), index)),
    };

    if OpCode::from_str(opcode).is_err() {
        return Err(JamParseError::InvalidOpCode(opcode.to_string(), index + 1));
    }

    Ok(())
}

pub fn validate_jam(lines: &[String]) -> Result<(), JamParseError> {
    for (line_number, line) in lines.iter().enumerate() {
        validate_line(line, line_number)?;
    }

    Ok(())
}
