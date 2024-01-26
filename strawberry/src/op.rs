use crate::register::Register;
use std::str::FromStr;

use strawberryvm_derive::VmInstruction;

/// All instructions for the VM. They are automatically
/// implemented with an encode function to turn them into
/// binary and also implements From traits.
#[derive(Debug, VmInstruction)]
pub enum Instruction {
    // Miscellaneous
    #[opcode(0x0)]
    Nop,

    // Memory management
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

    // Host communication
    #[opcode(0x6)]
    Signal(u8),

    // Flow management
    #[opcode(0x7)]
    Jmp(u8),
    #[opcode(0x8)]
    JmpNE(u8),
    #[opcode(0x9)]
    JmpEQ(u8),

    // Bitshift operators
    #[opcode(0xa)]
    ShiftLeft(u8),
    #[opcode(0xb)]
    ShiftRight(u8),
    #[opcode(0xc)]
    And,
    #[opcode(0xd)]
    Or,

    // Loading operators
    #[opcode(0xe)]
    LoadA(u8),
    #[opcode(0xf)]
    LoadB(u8),
    #[opcode(0x10)]
    LoadReg(Register, Register),

    // Arithmetic
    #[opcode(0x11)]
    Cmp(Register, Register),
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

            OpCode::JmpNE => {
                let reg = parse_instruction_arg(ins);
                Ok(Instruction::JmpNE(reg))
            }

            OpCode::JmpEQ => {
                let reg = parse_instruction_arg(ins);
                Ok(Instruction::JmpEQ(reg))
            }

            OpCode::ShiftRight => Ok(Instruction::ShiftRight(parse_instruction_arg(ins))),
            OpCode::ShiftLeft => Ok(Instruction::ShiftLeft(parse_instruction_arg(ins))),

            OpCode::And => Ok(Instruction::And),
            OpCode::Or => Ok(Instruction::Or),

            OpCode::LoadA => {
                let arg = parse_instruction_arg(ins);
                Ok(Instruction::LoadA(arg))
            }

            OpCode::LoadB => {
                let arg = parse_instruction_arg(ins);
                Ok(Instruction::LoadB(arg))
            }

            OpCode::LoadReg => {
                let higher = ((ins & 0xFF00) >> 8) as u8;
                let (r1, r2) = ((higher & 0xF0) >> 4, higher & 0x0F);

                Ok(Instruction::LoadReg(Register::from(r1), Register::from(r2)))
            }

            OpCode::Cmp => {
                let higher = ((ins & 0xFF00) >> 8) as u8;
                let (r1, r2) = ((higher & 0xF0) >> 4, higher & 0x0F);

                Ok(Instruction::Cmp(Register::from(r1), Register::from(r2)))
            }
        }
    }
}
