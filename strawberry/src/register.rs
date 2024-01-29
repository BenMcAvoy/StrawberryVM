use strawberryvm_derive::{FromU8, Display};

/// Enum for registers, only really used
/// to coordinate the register slice.
#[derive(Debug, Clone, Copy, FromU8, Display)]
pub enum Register {
    A = 0,  // General purpose
    B = 1,  // General purpose
    C = 2,  // General purpose
    M = 3,  // Memory address register
    SP = 4, // Stack pointer
    PC = 5, // Program counter
    BP = 6, // Base pointer
    FL = 7, // Flags register
}
