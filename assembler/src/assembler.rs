use std::collections::HashMap;
use strawberryvm::prelude::*;

// TODO: Take input text. Filter it appopriately to remove all kinds of comments to
// to get the valid assembly.
// TODO: Split up into valid files for easier processing.
// TODO: Iteratively process cleaned up input text and process into binary. (cleaned up version
// rather than the current mess)
// TODO: Future - Iteratively process that data and attempt to create some sort of tree or other data structure
// to allow functions/subroutines to be possible.

/// The main struct for the assembler containing
/// important information for creating a resultant
/// binary that the machine can run.
pub struct Assembler {
    pub output: Vec<u8>,

    labels: HashMap<String, u8>,

    lines: Vec<String>,
}

impl Assembler {
    /// Creates a new assembler from some labels and the lines of an
    /// assembly file as input.
    pub fn new(labels: &HashMap<&str, u8>, lines: &[&str]) -> Self {
        let labels: HashMap<String, u8> =
            labels.iter().map(|(k, &v)| ((*k).to_string(), v)).collect();

        let lines = lines.iter().map(ToString::to_string).collect();
        let output = Vec::new();

        Self {
            output,
            labels,
            lines,
        }
    }
}

impl Assembler {
    /// Used to parse a numeric based on whether it is binary,
    /// decimal, or hexadecimal.
    fn parse_numeric(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
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
    fn parse_register(s: &str) -> Result<Register, Box<dyn std::error::Error>> {
        let s = s.to_lowercase();

        match s.as_str() {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            _ => Err(format!("Unknown register {s}").into()),
        }
    }

    /// Used for the jump instruction to detect if the user is trying
    /// to jump to a label they have defined or just a pc count.
    ///
    /// This is automatically done here and returns a simple u8 the
    /// pc register is set to.
    fn parse_label(&self, s: &str) -> Result<u8, Box<dyn std::error::Error>> {
        if let Ok(u8) = Self::parse_numeric(s) {
            return Ok(u8);
        }

        if let Some(s) = s.strip_prefix('^') {
            if let Some(u8) = self.labels.get(s) {
                return Ok(*u8);
            }
        }

        Err(format!("Couldn't parse {s}").into())
    }

    fn assert_length(parts: &[&str], n: usize) -> Result<(), Box<dyn std::error::Error>> {
        if !parts.len() == n {
            return Err(format!("Expected {} got {}", n, parts.len()).into());
        }

        Ok(())
    }

    // TODO: Very good candidate for derive macro.
    /// Used to take a line and figure out what it's opcode and operand are to
    /// create an instruction that can be encoded.
    fn handle_line(&self, parts: &[&str]) -> Result<Instruction, Box<dyn std::error::Error>> {
        let opcode = parts[0]
            .parse()
            .map_err(|_| format!("Unknown opcode: {}", parts[0]))?;

        match opcode {
            OpCode::Push => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::Push(Self::parse_numeric(parts[1])?))
            }

            OpCode::AddStack => {
                Self::assert_length(parts, 1)?;
                Ok(Instruction::AddStack)
            }

            OpCode::AddReg => {
                let (r1, r2) = (
                    Self::parse_register(parts[1])?,
                    Self::parse_register(parts[2])?,
                );
                Ok(Instruction::AddReg(r1, r2))
            }

            OpCode::PopReg => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::PopReg(Self::parse_register(parts[1])?))
            }

            OpCode::PushReg => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::PushReg(Self::parse_register(parts[1])?))
            }

            OpCode::Signal => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::Signal(Self::parse_numeric(parts[1])?))
            }

            OpCode::Nop => {
                Self::assert_length(parts, 1)?;
                Ok(Instruction::Nop)
            }

            OpCode::Jmp => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::Jmp(self.parse_label(parts[1])?))
            }
        }
    }

    /// Central function that takes the input and turns it into actual assembly using helper
    /// functions
    pub fn parse_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for line in self
            .lines
            .iter()
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with(';'))
            .filter(|line| !line.ends_with(':'))
        {
            let parts: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();

            if parts.is_empty() {
                continue;
            }

            let instruction = self.handle_line(&parts)?;
            let raw_instruction: u16 = instruction.encode_u16();

            let lower = (raw_instruction & 0xff) as u8;
            let upper = (raw_instruction >> 8) as u8;

            self.output.push(lower);
            self.output.push(upper);
        }

        Ok(())
    }
}
