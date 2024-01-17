#[derive(Debug, Clone, Copy)]
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

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            3 => Register::M,
            4 => Register::SP,
            5 => Register::PC,
            6 => Register::BP,
            7 => Register::FL,
            _ => unreachable!(),
        }
    }
}
