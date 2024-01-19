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

// #[derive(Debug, VmInstruction)]
// pub enum Instruction {
//     #[opcode(0x0)]
//     Nop,
//     #[opcode(0x1)]
//     Push(u8),
//     #[opcode(0x2)]
//     PopReg(Register),
//     #[opcode(0x3)]
//     AddStack,
//     #[opcode(0x4)]
//     AddReg(Register, Register),
//     #[opcode(0x5)]
//     PushReg(Register),
//     #[opcode(0x6)]
//     Signal(u8),
//     #[opcode(0x7)]
//     Jmp(u8),
// }

impl Instruction {
    fn encode_r1(r: Register) -> u16 {
        (r as u16) & 0xf << 8
    }

    fn encode_r2(r: Register) -> u16 {
        (r as u16) & 0xf << 12
    }

    fn encode_num(u: u16) -> u16 {
        u << 8
    }

    fn encode_rs(r1: Register, r2: Register) -> u16 {
        Self::encode_r1(r1) | Self::encode_r2(r2)
    }

    pub fn encode_u16(&self) -> u16 {
        match self {
            Self::Nop => OpCode::Nop as u16,
            Self::Push(x) => OpCode::Push as u16 | Self::encode_num(*x as u16),
            Self::PopReg(r) => OpCode::PopReg as u16 | Self::encode_r1(*r),
            Self::PushReg(r) => OpCode::PushReg as u16 | Self::encode_r1(*r),
            Self::AddStack => OpCode::AddStack as u16,
            Self::AddReg(r1, r2) => OpCode::AddReg as u16 | Self::encode_rs(*r1, *r2),
            Self::Signal(x) => OpCode::Signal as u16 | Self::encode_num(*x as u16),
            Self::Jmp(r) => OpCode::Jmp as u16 | Self::encode_num(*r as u16),
        }
    }
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
