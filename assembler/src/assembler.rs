use std::str::FromStr;

use crate::helpers::split_u16;
use crate::helpers::DynErr;

use crate::parsing::validate_line;
use crate::parsing::JamParseError;
use crate::passes::*;

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

            let parts: &str = &text_slice
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            let instruction = Instruction::from_str(parts)?;

            return Ok(instruction.encode_u16());
        }

        Err(JamParseError::Empty(line_number).into())
    }
}
