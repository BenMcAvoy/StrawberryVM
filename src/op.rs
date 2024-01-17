use crate::register::Register;
use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddReg(Register, Register),
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
            Self::AddStack => OpCode::AddStack as u16,
            Self::AddReg(r1, r2) => OpCode::AddReg as u16 | Self::encode_rs(*r1, *r2),
            Self::Signal(x) => OpCode::Signal as u16 | Self::encode_num(*x as u16),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    Nop = 0x0,
    Push = 0x1,
    PopReg = 0x2,
    Signal = 0x0f,
    AddStack = 0x10,
    AddReg = 0x11,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Nop" => Ok(Self::Nop),
            "Push" => Ok(Self::Push),
            "PopReg" => Ok(Self::PopReg),
            "Signal" => Ok(Self::Signal),
            "AddStack" => Ok(Self::AddStack),
            "AddReg" => Ok(Self::AddReg),
            _ => Err(()),
        }
    }
}

impl OpCode {
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            x if x == Self::Nop as u8 => Some(Self::Nop),
            x if x == Self::Push as u8 => Some(Self::Push),
            x if x == Self::PopReg as u8 => Some(Self::PopReg),
            x if x == Self::Signal as u8 => Some(Self::Signal),
            x if x == Self::AddStack as u8 => Some(Self::AddStack),
            x if x == Self::AddReg as u8 => Some(Self::AddReg),
            _ => None,
        }
    }
}
