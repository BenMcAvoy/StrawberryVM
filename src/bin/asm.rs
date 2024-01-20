use strawberryvm::prelude::*;

use std::collections::HashMap;
use std::env;
use std::path::Path;

use std::fs::read_to_string;
use std::io::stdout;
use std::io::Write;

struct Assembler {
    pub output: Vec<u8>,

    labels: HashMap<String, u8>,

    lines: Vec<String>,
}

impl Assembler {
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

    fn parse_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

/// Usage:
/// ./asm file.asm
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let file = read_to_string(Path::new(&args[1]))?;
    let lines: Vec<&str> = file.lines().map(str::trim_start).collect();

    let mut labels = HashMap::new();

    for (index, line) in lines
        .iter()
        .filter(|line| !line.starts_with(';'))
        .enumerate()
    {
        if !line.ends_with(':') {
            continue;
        }

        if let Some(label) = line.strip_suffix(':') {
            let offset = lines
                .iter()
                .take(index + 1)
                .filter(|line| line.ends_with(':'))
                .count();

            let pc = u8::try_from((index * 2) - offset * 2)?;
            labels.insert(label, pc);
        }
    }

    let mut assembler = Assembler::new(&labels, &lines);
    assembler.parse_input()?;

    stdout().lock().write_all(&assembler.output)?;

    Ok(())
}
