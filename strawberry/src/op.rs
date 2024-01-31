use crate::register::Register;
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

    #[opcode(0x40)]
    IfZero(Register),
    #[opcode(0x41)]
    BranchImm(i8),

    // Host communication
    #[opcode(0x6)]
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
