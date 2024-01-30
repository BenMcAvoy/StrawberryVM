use std::str::FromStr;

use crate::helpers::assert_length;
use crate::helpers::split_u16;
use crate::helpers::DynErr;

use crate::parsing::validate_line;
use crate::parsing::JamParseError;
use crate::passes::*;

use crate::parsing::parse_numeric;

use strawberryvm::prelude::*;

pub struct Assembler();

impl Assembler {
    pub fn parse_vec(&self, input: &[String]) -> Result<Vec<u8>, DynErr> {
        let mut out = Vec::new();
        for (index, line) in input.iter().enumerate() {
            let dbyte = match self.parse_line(String::from(line), index) {
                Ok(v) => v,
                Err(e) => {
                    if let Some(JamParseError::Empty(_)) = e.downcast_ref::<JamParseError>() {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            };

            let (lower, upper) = split_u16(dbyte);

            out.push(lower);
            out.push(upper);
        }

        Ok(out)
    }

    pub fn parse_line(&self, text: String, line_number: usize) -> Result<u16, DynErr> {
        let text_slice = text.as_str();
        let cleaned = pre::remove_comments_pass(text_slice);

        if let Some(text) = cleaned {
            validate_line(&text, line_number)?;

            let parts: Vec<&str> = text_slice.split(' ').filter(|x| !x.is_empty()).collect();
            let instruction = self.handle_line(&parts)?;

            return Ok(instruction.encode_u16());
        }

        Err(JamParseError::Empty(line_number).into())
    }

    fn handle_line(&self, parts: &[&str]) -> Result<Instruction, DynErr> {
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
                let (r1, r2) = (Register::from_str(parts[1])?, Register::from_str(parts[2])?);
                Ok(Instruction::AddReg(r1, r2))
            }

            OpCode::PopReg => {
                assert_length(parts, 2)?;
                Ok(Instruction::PopReg(Register::from_str(parts[1])?))
            }

            OpCode::PushReg => {
                assert_length(parts, 2)?;
                Ok(Instruction::PushReg(Register::from_str(parts[1])?))
            }

            OpCode::Signal => {
                assert_length(parts, 2)?;
                Ok(Instruction::Signal(parse_numeric(parts[1])?))
            }

            OpCode::Nop => {
                assert_length(parts, 1)?;
                Ok(Instruction::Nop)
            }
        }
    }
}
