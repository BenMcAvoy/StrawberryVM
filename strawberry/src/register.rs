use strawberryvm_derive::{Display, FromStr, FromU8};

pub enum Flag {
    Compare = 1 << 0,
    Negative = 1 << 1,
    Overflow = 1 << 2,
}

/// Enum for registers, only really used
/// to co-ordinate the register slice.
#[derive(Debug, Clone, Copy, FromU8, Display, FromStr)]
pub enum Register {
    A,  // General purpose
    B,  // General purpose
    C,  // General purpose
    D,  // General purpose
    SP, // Stack pointer
    PC, // Program counter
    BP, // Base pointer
    FL, // Flags register
}
