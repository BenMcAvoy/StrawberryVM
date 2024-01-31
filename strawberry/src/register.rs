use strawberryvm_derive::{Display, FromStr, FromU8};

pub enum Flag {
    Compare = 0x1,
}

/// Enum for registers, only really used
/// to co-ordinate the register slice.
#[derive(Debug, Clone, Copy, FromU8, Display, FromStr)]
pub enum Register {
    A,  // General purpose
    B,  // General purpose
    C,  // General purpose
    M,  // Memory address register
    SP, // Stack pointer
    PC, // Program counter
    BP, // Base pointer
    FL, // Flags register
}
