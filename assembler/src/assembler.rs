use crate::passes::remove_comments_pass;
use strawberryvm::prelude::*;

use crate::parsing::parse_numeric;
use crate::parsing::parse_register;

pub struct Assembler {
    pub(crate) input: Vec<String>,
    pub(crate) output: Vec<u8>,
}

fn assert_length(parts: &[&str], n: usize) -> Result<(), Box<dyn std::error::Error>> {
    if !parts.len() == n {
        return Err(format!("Expected {} got {}", n, parts.len()).into());
    }

    Ok(())
}

impl Assembler {
    pub fn new(lines: &[&str]) -> Self {
        let input = lines.iter().map(ToString::to_string).collect();
        let output = Vec::new();

        Self { input, output }
    }

    pub fn clean_passes(&mut self) {
        remove_comments_pass(&mut self.input);
    }

    fn handle_line(&self, parts: &[&str]) -> Result<Instruction, Box<dyn std::error::Error>> {
        let opcode = parts[0]
            .parse()
            .map_err(|_| format!("Unknown opcode: {}", parts[0]))?;

        match opcode {
            OpCode::Push => {
                assert_length(parts, 2)?;
                Ok(Instruction::Push(parse_numeric(parts[1])?))
            }

            OpCode::AddStack => {
                assert_length(parts, 1)?;
                Ok(Instruction::AddStack)
            }

            OpCode::AddReg => {
                let (r1, r2) = (parse_register(parts[1])?, parse_register(parts[2])?);
                Ok(Instruction::AddReg(r1, r2))
            }

            OpCode::PopReg => {
                assert_length(parts, 2)?;
                Ok(Instruction::PopReg(parse_register(parts[1])?))
            }

            OpCode::PushReg => {
                assert_length(parts, 2)?;
                Ok(Instruction::PushReg(parse_register(parts[1])?))
            }

            OpCode::Signal => {
                assert_length(parts, 2)?;
                Ok(Instruction::Signal(parse_numeric(parts[1])?))
            }

            OpCode::Nop => {
                assert_length(parts, 1)?;
                Ok(Instruction::Nop)
            }

            OpCode::Jmp => {
                assert_length(parts, 2)?;
                // Ok(Instruction::Jmp(self.parse_label(parts[1])?))
                Ok(Instruction::Jmp(parse_numeric(parts[1])?))
            }

            OpCode::ShiftLeft => {
                let (r1, r2) = (parse_register(parts[1])?, parse_numeric(parts[2])?);
                Ok(Instruction::ShiftLeft(r1, r2))
            }
        }
    }

    pub fn parse_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for line in self.input.iter() {
            let parts: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();

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
