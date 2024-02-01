use crate::register::Register;
use strawberryvm_derive::VmInstruction;

/// All instructions for the VM. They are automatically
/// implemented with an encode function to turn them into
/// binary and also implements From traits.
#[derive(Debug, VmInstruction)]
pub enum Instruction {
    // Miscellaneous
    #[opcode(0x00)]
    Nop,

    // Memory management
    #[opcode(0x10)]
    Push(u8),
    #[opcode(0x11)]
    PopReg(Register),
    #[opcode(0x12)]
    PushReg(Register),
    #[opcode(0x13)]
    LoadAImm(u8),
    #[opcode(0x14)]
    LoadBImm(u8),
    #[opcode(0x13)]
    LoadCImm(u8),
    #[opcode(0x14)]
    LoadSPImm(u8),

    #[opcode(0x20)]
    AddStack,
    #[opcode(0x21)]
    AddReg(Register, Register),
    #[opcode(0x22)]
    SubStack,
    #[opcode(0x23)]
    SubReg(Register, Register),
    #[opcode(0x24)]
    IncReg(Register),

    #[opcode(0x30)]
    IfZero(Register),
    #[opcode(0x31)]
    IfNotZero(Register),
    #[opcode(0x32)]
    BranchImm(i8),

    // Host communication
    #[opcode(0x40)]
    Signal(u8),
}

#[derive(Debug)]
pub enum InstructionParseError {
    NoContent,
    Fail(String),
}

impl std::error::Error for InstructionParseError {}

impl std::fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoContent => {
                write!(f, "No content.")
            }

            Self::Fail(message) => {
                write!(f, "Error {message}")
            }
        }
    }
}
