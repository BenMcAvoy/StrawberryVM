use crate::op::{Instruction, OpCode};
use crate::register::Register;
use std::collections::HashMap;

use crate::memory::Addressable;
use crate::memory::LinearMemory;

pub(crate) const REGISTER_COUNT: usize = 8;
pub(crate) const MEMORY_KILO_BYTES: usize = 8;

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

        OpCode::PushReg => {
            let reg = (ins & 0xf00) >> 8;
            let reg = Register::from(reg as u8);

            Ok(Instruction::PushReg(reg))
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

    pub fn status(&self) -> String {
        let width = 4;

        let (a, b, c, m) = (
            self.get_register(Register::A),
            self.get_register(Register::B),
            self.get_register(Register::C),
            self.get_register(Register::M),
        );

        let (sp, pc, bp) = (
            self.get_register(Register::SP),
            self.get_register(Register::PC),
            self.get_register(Register::BP),
        );

        let flags = self.get_register(Register::FL);

        let line_width = (width + 3) * 8;
        let lines = vec![
            String::from(""),
            format!("   {:^line_width$}", "» Registers «"),
            format!(" ┌{:─<line_width$}┐",""),
            format!(" │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │", "A", "B", "C", "M", "SP", "PC", "BP", "FLAGS"),
            format!(" │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^5} │", a, b, c, m, sp, pc, bp, flags),
            format!(" └{:─<line_width$}┘", ""),
            String::from(""),
        ];

        lines.join("\n")
    }

    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    pub fn define_handler(&mut self, id: u8, handler: SignalFunction) {
        self.signal_handlers.insert(id, handler);
    }

    pub fn push(&mut self, v: u16) -> Result<(), Box<dyn std::error::Error>> {
        let sp = self.registers[Register::SP as usize];
        self.memory.write_u16(sp, v)?;
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

        println!("{:0>4} │ Got instruction {op:?}", pc);

        match op {
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => {
                self.push(v as u16)?;
                Ok(())
            }

            Instruction::PopReg(r) => {
                let popped = self.pop()?;
                self.registers[r as usize] = popped;
                Ok(())
            }

            Instruction::PushReg(r) => {
                self.push(self.registers[r as usize])?;
                Ok(())
            }

            Instruction::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;

                self.push(a + b)?;

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
