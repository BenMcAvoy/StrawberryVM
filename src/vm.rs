use crate::memory::Addressable;
use crate::memory::LinearMemory;

const REGISTER_COUNT: usize = 8;
const MEMORY_KILO_BYTES: usize = 8;

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

pub struct Machine {
    registers: [u16; REGISTER_COUNT],
    memory: Box<dyn Addressable>,
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
        let instruction = self.memory.read_u16(pc)?;
        self.registers[Register::PC as usize] = pc + 2;
        println!("{} @ {}", instruction, pc);

        Ok(())
    }
}
