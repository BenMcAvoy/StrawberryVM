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
}
