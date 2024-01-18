use crate::register::Register;
use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddReg(Register, Register),
    PushReg(Register),
    Signal(u8),
}

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
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    Nop = 0x0,
    Push = 0x1,
    PopReg = 0x2,
    PushReg = 0x3,
    Signal = 0x0f,
    AddStack = 0x10,
    AddReg = 0x11,
}

impl FromStr for OpCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Nop" => Ok(Self::Nop),
            "Push" => Ok(Self::Push),
            "PopReg" => Ok(Self::PopReg),
            "PushReg" => Ok(Self::PushReg),
            "Signal" => Ok(Self::Signal),
            "AddStack" => Ok(Self::AddStack),
            "AddReg" => Ok(Self::AddReg),
            _ => Err(format!("Unknown opcode {s}")),
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Nop as u8 => Ok(Self::Nop),
            x if x == Self::Push as u8 => Ok(Self::Push),
            x if x == Self::PopReg as u8 => Ok(Self::PopReg),
            x if x == Self::PushReg as u8 => Ok(Self::PushReg),
            x if x == Self::Signal as u8 => Ok(Self::Signal),
            x if x == Self::AddStack as u8 => Ok(Self::AddStack),
            x if x == Self::AddReg as u8 => Ok(Self::AddReg),
            _ => Err(format!("Unknown opcode {value:X}")),
        }
    }
}
