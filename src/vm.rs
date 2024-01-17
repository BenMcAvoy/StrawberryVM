use std::collections::HashMap;
use std::str::FromStr;

use crate::memory::Addressable;
use crate::memory::LinearMemory;

pub(crate) const REGISTER_COUNT: usize = 8;
pub(crate) const MEMORY_KILO_BYTES: usize = 8;

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

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    AddReg(Register, Register),
    Signal(u8),
}

impl Instruction {
    fn encode_r1(r: Register) -> u16 {
        (r as u16) & 0xf << 8
    }

    fn encode_r2(r: Register) -> u16 {
        (r as u16) & 0xf << 12
    }

    fn encode_num(u: u16) -> u16 {
        u << 8
    }

    fn encode_rs(r1: Register, r2: Register) -> u16 {
        Self::encode_r1(r1) | Self::encode_r2(r2)
    }

    pub fn encode_u16(&self) -> u16 {
        match self {
            Self::Nop => OpCode::Nop as u16,
            Self::Push(x) => OpCode::Push as u16 | Self::encode_num(*x as u16),
            Self::PopReg(r) => OpCode::PopReg as u16 | Self::encode_r1(*r),
            Self::AddStack => OpCode::AddStack as u16,
            Self::AddReg(r1, r2) => OpCode::AddReg as u16 | Self::encode_rs(*r1, *r2),
            Self::Signal(x) => OpCode::Signal as u16 | Self::encode_num(*x as u16),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    Nop = 0x0,
    Push = 0x1,
    PopReg = 0x2,
    Signal = 0x0f,
    AddStack = 0x10,
    AddReg = 0x11,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Nop" => Ok(Self::Nop),
            "Push" => Ok(Self::Push),
            "PopReg" => Ok(Self::PopReg),
            "Signal" => Ok(Self::Signal),
            "AddStack" => Ok(Self::AddStack),
            "AddReg" => Ok(Self::AddReg),
            _ => Err(()),
        }
    }
}

impl OpCode {
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            x if x == Self::Nop as u8 => Some(Self::Nop),
            x if x == Self::Push as u8 => Some(Self::Push),
            x if x == Self::PopReg as u8 => Some(Self::PopReg),
            x if x == Self::Signal as u8 => Some(Self::Signal),
            x if x == Self::AddStack as u8 => Some(Self::AddStack),
            x if x == Self::AddReg as u8 => Some(Self::AddReg),
            _ => None,
        }
    }
}

fn parse_instruction_arg(ins: u16) -> u8 {
    ((ins & 0xff00) >> 8) as u8
}

fn parse_instruction(ins: u16) -> Result<Instruction, String> {
    let op = (ins & 0xff) as u8;
    let opcode = OpCode::from_u8(op).ok_or(format!("Unknown op {:X}", op))?;

    match opcode {
        OpCode::Nop => Ok(Instruction::Nop),
        OpCode::Push => {
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Push(arg))
        }

        OpCode::PopReg => {
            let reg = (ins & 0xf00) >> 8;
            let reg = Register::from(reg as u8);

            Ok(Instruction::PopReg(reg))
        }

        OpCode::AddStack => Ok(Instruction::AddStack),

        OpCode::AddReg => {
            let r1 = Register::from(((ins & 0xf00) >> 8) as u8);
            let r2 = Register::from(((ins & 0xf00) >> 12) as u8);

            Ok(Instruction::AddReg(r1, r2))
        }

        OpCode::Signal => {
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Signal(arg))
        }
    }
}

type SignalFunction = fn(&mut Machine) -> Result<(), String>;

pub struct Machine {
    pub memory: Box<dyn Addressable>,
    registers: [u16; REGISTER_COUNT],

    signal_handlers: HashMap<u8, SignalFunction>,

    pub machine_halted: bool,
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

            signal_handlers: HashMap::new(),
            machine_halted: false,
        }
    }

    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    pub fn define_handler(&mut self, id: u8, handler: SignalFunction) {
        self.signal_handlers.insert(id, handler);
    }

    pub fn push(&mut self, v: u8) -> Result<(), Box<dyn std::error::Error>> {
        let sp = self.registers[Register::SP as usize];
        self.memory.write_u16(sp, v as u16)?;
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let sp = match self.registers[Register::SP as usize].checked_sub(2) {
            Some(result) => result,
            None => return Err("Stack pointer went back too far".into()),
        };

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

        // println!("{} | Got instruction {op:?}", pc);

        match op {
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => {
                self.push(v)?;
                Ok(())
            }

            Instruction::PopReg(r) => {
                let popped = self.pop()?;
                self.registers[r as usize] = popped;
                Ok(())
            }

            Instruction::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;

                self.push((a + b) as u8)?;

                Ok(())
            }

            Instruction::AddReg(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }

            Instruction::Signal(signal) => {
                self.signal_handlers
                    .get(&signal)
                    .ok_or(format!("Unknown signal 0x{:X}", signal))?(self)?;

                Ok(())
            }
        }
    }
}
