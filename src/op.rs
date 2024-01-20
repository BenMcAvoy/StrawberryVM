use crate::register::Register;
use std::str::FromStr;

use macros::VmInstruction;

#[derive(Debug, VmInstruction)]
pub enum Instruction {
    #[opcode(0x0)]
    Nop,
    #[opcode(0x1)]
    Push(u8),
    #[opcode(0x2)]
    PopReg(Register),
    #[opcode(0x3)]
    PushReg(Register),
    #[opcode(0x4)]
    AddStack,
    #[opcode(0x5)]
    AddReg(Register, Register),
    #[opcode(0x6)]
    Signal(u8),
    #[opcode(0x7)]
    Jmp(u8),
}

fn parse_instruction_arg(ins: u16) -> u8 {
    ((ins & 0xff00) >> 8) as u8
}

impl TryFrom<u16> for Instruction {
    type Error = String;

    fn try_from(ins: u16) -> Result<Self, Self::Error> {
        let op = (ins & 0xff) as u8;
        let opcode = OpCode::try_from(op)?;

        match opcode {
            OpCode::Nop => Ok(Instruction::Nop),
            OpCode::Push => {
                let arg = parse_instruction_arg(ins);
                Ok(Instruction::Push(arg))
            }

            OpCode::PopReg => {
                let reg = (ins & 0xf00) >> 8;
                let reg = Register::from(reg as u8);

                Ok(Instruction::PopReg(reg))
            }

            OpCode::PushReg => {
                let reg = (ins & 0xf00) >> 8;
                let reg = Register::from(reg as u8);

                Ok(Instruction::PushReg(reg))
            }

            OpCode::AddStack => Ok(Instruction::AddStack),

            OpCode::AddReg => {
                let r1 = Register::from(((ins & 0xf00) >> 8) as u8);
                let r2 = Register::from(((ins & 0xf00) >> 12) as u8);

                Ok(Instruction::AddReg(r1, r2))
            }

            OpCode::Signal => {
                let arg = parse_instruction_arg(ins);
                Ok(Instruction::Signal(arg))
            }

            OpCode::Jmp => {
                let reg = parse_instruction_arg(ins);
                Ok(Instruction::Jmp(reg))
            }
        }
    }
}
