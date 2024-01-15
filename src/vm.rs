use crate::memory::Addressable;
use crate::memory::LinearMemory;

pub(crate) const REGISTER_COUNT: usize = 8;
pub(crate) const MEMORY_KILO_BYTES: usize = 8;

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

#[repr(u8)]
pub enum Op {
    Nop,
}

pub struct Machine {
    pub memory: Box<dyn Addressable>,
    registers: [u16; REGISTER_COUNT],
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_COUNT],
            memory: Box::new(LinearMemory::new(MEMORY_KILO_BYTES * 1024)),
        }
    }

    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pc = self.registers[Register::PC as usize];
        self.registers[Register::PC as usize] = pc + 2;
        let instruction = self.memory.read_u16(pc)?;
        let op = (instruction & 0xff) as u8;

        match op {
            op if op == Op::Nop as u8 => Ok(()),
            _ => Err(format!("Invalid operator at 0x{:X} @ PC {}", op, pc).into()),
        }
    }
}
