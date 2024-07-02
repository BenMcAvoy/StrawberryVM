use crate::memory::{Addressable, Memory};
use crate::utils::Result;
use crate::op::Op;

#[allow(dead_code)]
enum Register {
    A,     // General purpose
    B,     // General purpose
    C,     // General purpose
    M,     // Memory
    SP,    // Stack Pointer
    PC,    // Program Counter
    BP,    // Base Pointer
    Flags, // Flags
}

pub struct Machine {
    registers: [u16; 8],
    memory: Box<dyn Addressable>,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(Memory::default()),
        }
    }
}

impl Machine {
    pub fn iterate(&mut self, n: usize) {
        for _ in 0..n {
            if let Err(e) = self.step() {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    pub fn step(&mut self) -> Result {
        // self.memory.write_word(0x0, 0x12);
        self.memory.write_word(0x2, 0x12);
        self.memory.write_word(0x4, 0x12);
        // self.memory.write_word(0x6, 0x12);

        let pc = self.registers[Register::PC as usize];
        self.registers[Register::PC as usize] += 2;
        let ins = self.memory.read_word(pc).ok_or("Failed to read instruction")?;

        // Temporary format:
        // instruction = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 ]
        //                 OPERATOR        | ARG(S)
        //                                 | 8 BIT LITERAL
        //                                 | REG 1 | REG 2

        let op = ins & 0xFF; // Get first 8 bits.
        let op_str = match op {
            x if x == Op::Nop as u16 => "NOP",
            _ => "???"
        };

        println!("@ PC 0x{pc:04X}: 0x{ins:04X} == {op_str}");

        Ok(())
    }
}
