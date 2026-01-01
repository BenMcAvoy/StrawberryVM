use crate::register::Register;
use strawberryvm_derive::VmInstruction;

/// All instructions for the VM. They are automatically
/// implemented with an encode function to turn them into
/// binary and also implements From traits.
#[derive(Debug, VmInstruction)]
pub enum Instruction {
    #[opcode(0x00)] Nop,                       // No operation    

    #[opcode(0x10)] Push(u8),                  // Push an 8-bit value onto the stack
    #[opcode(0x11)] Pop(Register),             // Pop top of stack -> register
    #[opcode(0x12)] PushReg(Register),         // Push register value onto stack (does not modify register)
    #[opcode(0x13)] Mov(Register, Register),   // Copy value from second register to first

    #[opcode(0x20)] Add(Register, Register),   // Add two registers and store in the first
    #[opcode(0x21)] Sub(Register, Register),   // Subtract two registers and store in the first
    #[opcode(0x22)] Shl(Register, Register),   // Shift left first register by amount in second
    #[opcode(0x23)] Shr(Register, Register),   // Shift right first register by amount in second
    #[opcode(0x24)] And(Register, Register),   // Bitwise AND two registers and store in the first
    #[opcode(0x25)] Or(Register, Register),    // Bitwise OR two registers and store in the first
    #[opcode(0x26)] Xor(Register, Register),   // Bitwise XOR two registers and store in the first
    #[opcode(0x27)] Not(Register),             // Bitwise NOT on a register
    #[opcode(0x28)] Mul(Register, Register),   // Multiply two registers and store in the first
    #[opcode(0x29)] Div(Register, Register),   // Divide two registers and store in the first

    #[opcode(0x30)] Cmp(Register, Register),   // Compare two registers and set flags
    #[opcode(0x31)] Jmp(i8),                   // Jump by signed offset
    #[opcode(0x32)] Je(i8),                    // Jump if Compare flag is set
    #[opcode(0x33)] Jne(i8),                   // Jump if Compare flag is not set

    #[opcode(0x40)] Load(Register, Register),  // Load from memory address in second register into first
    #[opcode(0x41)] Store(Register, Register), // Store value from first register into memory address in second
    
    #[opcode(0x50)] Signal(u8),                // Host call
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
