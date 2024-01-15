use crate::memory::Addressable;
use crate::memory::LinearMemory;

pub(crate) const REGISTER_COUNT: usize = 8;
pub(crate) const MEMORY_KILO_BYTES: usize = 8;

#[derive(Debug)]
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

#[repr(u8)]
#[derive(Debug)]
pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddReg(Register, Register),
}

impl Op {
    pub fn value(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

fn parse_instruction(ins: u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;

    match op {
        x if x == Op::Nop.value() => Ok(Op::Nop),
        x if x == Op::Push(0).value() => {
            let arg = (ins & 0xff00) >> 8;
            Ok(Op::Push(arg as u8))
        }

        x if x == Op::PopReg(Register::A).value() => {
            let reg = (ins & 0xf00) >> 8;
            let reg = Register::from(reg as u8);

            Ok(Op::PopReg(reg))
        },

        x if x == Op::AddStack.value() => Ok(Op::AddStack),

        _ => Err(format!("Failed to parse instruction 0x{:X}", op)),
    }
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

    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    pub fn push(&mut self, v: u8) -> Result<(), Box<dyn std::error::Error>> {
        let sp = self.registers[Register::SP as usize];
        self.memory.write_u16(sp, v as u16)?;
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let sp = self.registers[Register::SP as usize] - 2;
        self.registers[Register::SP as usize] -= 2;

        self.memory
            .read_u16(sp)
            .map_err(|_| format!("Failed to read memory @ 0x{:X}", sp))
            .map(Ok)?
    }

    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pc = self.registers[Register::PC as usize];
        self.registers[Register::PC as usize] = pc + 2;
        let instruction = self.memory.read_u16(pc)?;
        let op = parse_instruction(instruction)?;

        println!("Got op {op:?}");

        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => self.push(v),
            Op::PopReg(r) => {
                let popped = self.pop()?;
                self.registers[r as usize] = popped;
                Ok(())
            },

            Op::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;

                self.push((a + b) as u8)
            }

            Op::AddReg(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }

            _ => Err(format!("Invalid operator {:?} @ PC {}", op, pc).into()),
        }
    }
}
