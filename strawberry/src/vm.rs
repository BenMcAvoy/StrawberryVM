use crate::op::Instruction;
use crate::prelude::Flag;
use crate::register::Register;
use std::collections::HashMap;

use crate::memory;
use crate::panic_report;

pub const MEMORY_KILO_BYTES: usize = 1;
pub const REGISTER_COUNT: usize = 8;

type SignalFunction = fn(&mut Machine);

/// The main structure for the VM. This can be created
/// and essentially is all you need to get going.
///
/// Through this structure, you can load data into
/// the machines memory, define signals and start
/// the machine.
pub struct Machine {
    pub memory: Box<dyn memory::Addressable>,
    registers: [u16; REGISTER_COUNT],

    signal_handlers: HashMap<u8, SignalFunction>,

    pub debug: bool,
    pub machine_halted: bool,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    /// Creates a new instance of a Machine with register and memory counts
    /// based on the constants set in the file.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_COUNT],
            memory: Box::new(memory::Linear::new(MEMORY_KILO_BYTES * 1024)),

            signal_handlers: HashMap::new(),
            machine_halted: false,

            debug: false,
        }
    }

    /// Returns a table of each register as a string
    /// This is only really useful for debugging and
    /// is not really useful for anything else.
    #[must_use]
    pub fn status(&self) -> String {
        let width = 5;

        let (a, b, c, m) = (
            self.get_register(Register::A),
            self.get_register(Register::B),
            self.get_register(Register::C),
            self.get_register(Register::D),
        );

        let (sp, pc, bp) = (
            self.get_register(Register::SP),
            self.get_register(Register::PC),
            self.get_register(Register::BP),
        );

        let flags = self.get_register(Register::FL);

        let line_width = (width + 3) * 8 - 1;
        let lines = vec![
            String::new(),
            format!("   {:^line_width$}", "» Registers «"),
            format!(" ┌{:─<line_width$}┐",""),
            format!(" │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │", "A", "B", "C", "M", "SP", "PC", "BP", "FLAGS"),
            format!(" │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │", a, b, c, m, sp, pc, bp, flags),
            format!(" └{:─<line_width$}┘", ""),
            String::new(),
        ];

        lines.join("\n")
    }

    /// Returns the value of of a register inside of the machine
    /// Typically only useful for debugging or pulling data out
    /// of the machine as the host.
    #[must_use]
    pub const fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    /// Creates a handler for a signal. Signals are simply used to
    /// communicate to the host from inside the machine.
    pub fn define_handler(&mut self, id: u8, handler: SignalFunction) {
        self.signal_handlers.insert(id, handler);
    }

    /// Used to push values to the stack. Will take in a u16, split it into two bytes
    /// and write them to the machines memory.
    ///
    /// # Errors
    /// This can fail if you attempt to write out of the memory constraints. E.g.
    /// if the VM has 16KBs of RAM and you write to 16,385 (1 above 16kb), you will
    /// get an error
    pub(crate) fn push(&mut self, v: u16) -> Result<(), Box<dyn std::error::Error>> {
        let sp = self.registers[Register::SP as usize];
        self.memory.write_u16(sp, v)?;
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    /// Used to pop values off of the stack. Will return 2 bytes from the memory
    /// as a u16.
    ///
    /// # Errors
    /// This can fail if you attempt pop too many values and the stack poitner goes
    /// below zero, which is impossible for an unsigned integer.
    pub(crate) fn pop(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let Some(sp) = self.registers[Register::SP as usize].checked_sub(2) else {
            return Err("Stack pointer went back too far".into());
        };

        self.registers[Register::SP as usize] -= 2;

        self.memory
            .read_u16(sp)
            .map_err(|_| format!("Failed to read memory @ 0x{sp:X}"))
            .map(Ok)?
    }

    fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.registers[Register::FL as usize] |= flag as u16;
        } else {
            self.registers[Register::FL as usize] &= !(flag as u16);
        }
    }

    const fn test_flag(&self, flag: Flag) -> bool {
        (self.registers[Register::FL as usize] & (flag as u16)) != 0
    }

    /// Used to step the machine forward, can be called by a
    /// virtual "clock" to simulate cpu cycles.
    ///
    /// # Errors
    /// This can error if memory read fails (see `read_u16`).
    /// It can also fail if memory popping fails.
    /// It can also fail if a signal is non-existant.
    /// It can also fail if the instruction is invalid.
    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // sleep(Duration::from_millis(100));
        let pc = self.registers[Register::PC as usize];
        self.registers[Register::PC as usize] += 2;
        let instruction = self.memory.read_u16(pc)?;

        // Snapshot state for panic reporting (panic hook must be `Send + Sync`, so it
        // can't safely capture `&Machine`).
        panic_report::set_last_status(format!(
            "PC: 0x{pc:04X}\nINSTR: 0x{instruction:04X}\n{}",
            self.status()
        ));

        let op = Instruction::try_from(instruction)?;

        if self.debug {
            println!("{pc:0>4} │ Got instruction `{op}`");
        }

        match op {
            Instruction::Nop => Ok(()),

            Instruction::Push(v) => self.push(u16::from(v)),

            Instruction::Pop(r) => {
                let v = self.pop()?;
                self.registers[r as usize] = v;
                Ok(())
            }

            Instruction::PushReg(r) => {
                let v = self.registers[r as usize];
                self.push(v)
            }

            Instruction::Add(dest, src) => {
                let (result, overflowed) = self.registers[dest as usize]
                    .overflowing_add(self.registers[src as usize]);

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Overflow, overflowed);
                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Sub(dest, src) => {
                let (result, overflowed) = self.registers[dest as usize]
                    .overflowing_sub(self.registers[src as usize]);

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Overflow, overflowed);
                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Shl(dest, src) => {
                let shift_amount = self.registers[src as usize] & 0x0F;
                let result = self.registers[dest as usize] << shift_amount;

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Shr(dest, src) => {
                let shift_amount = self.registers[src as usize] & 0x0F;
                let result = self.registers[dest as usize] >> shift_amount;

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::And(dest, src) => {
                let result = self.registers[dest as usize] & self.registers[src as usize];

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Or(dest, src) => {
                let result = self.registers[dest as usize] | self.registers[src as usize];

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Xor(dest, src) => {
                let result = self.registers[dest as usize] ^ self.registers[src as usize];

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Not(r) => {
                let result = !self.registers[r as usize];

                self.registers[r as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Mul(dest, src) => {
                let (result, overflowed) = self.registers[dest as usize]
                    .overflowing_mul(self.registers[src as usize]);

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Overflow, overflowed);
                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Div(dest, src) => {
                let divisor = self.registers[src as usize];
                if divisor == 0 {
                    return Err("Division by zero".into());
                }

                let result = self.registers[dest as usize] / divisor;

                self.registers[dest as usize] = result;

                self.set_flag(Flag::Negative, (result & 0x8000) != 0);
                self.set_flag(Flag::Compare, result == 0);

                Ok(())
            }

            Instruction::Mov(dest, src) => {
                let value = self.registers[src as usize];
                self.registers[dest as usize] = value;
                Ok(())
            }

            Instruction::Cmp(a, b) => {
                let val_a = self.registers[a as usize];
                let val_b = self.registers[b as usize];

                self.set_flag(Flag::Compare, val_a == val_b);
                self.set_flag(Flag::Negative, val_a < val_b);
                // Overflow flag is not typically set for comparisons

                Ok(())
            }

            Instruction::Jmp(offset) => {
                let pc = self.registers[Register::PC as usize];
                let new_pc = if offset.is_negative() {
                    pc.wrapping_sub(offset.wrapping_abs() as u16 * 2)
                } else {
                    pc.wrapping_add(offset as u16 * 2)
                };

                self.registers[Register::PC as usize] = new_pc;
                Ok(())
            }

            Instruction::Je(offset) => {
                if self.test_flag(Flag::Compare) {
                    let pc = self.registers[Register::PC as usize];
                    let new_pc = if offset.is_negative() {
                        pc.wrapping_sub(offset.wrapping_abs() as u16 * 2)
                    } else {
                        pc.wrapping_add(offset as u16 * 2)
                    };

                    self.registers[Register::PC as usize] = new_pc;
                }
                Ok(())
            }

            Instruction::Jne(offset) => {
                if !self.test_flag(Flag::Compare) {
                    let pc = self.registers[Register::PC as usize];
                    let new_pc = if offset.is_negative() {
                        pc.wrapping_sub(offset.wrapping_abs() as u16 * 2)
                    } else {
                        pc.wrapping_add(offset as u16 * 2)
                    };

                    self.registers[Register::PC as usize] = new_pc;
                }
                Ok(())
            }

            Instruction::Load(dest, src) => {
                let address = self.registers[src as usize];
                let value = self.memory.read_u16(address)?;
                self.registers[dest as usize] = value;
                Ok(())
            }

            Instruction::Store(src, dest) => {
                let address = self.registers[dest as usize];
                let value = self.registers[src as usize];
                self.memory.write_u16(address, value)?;
                Ok(())
            }

            Instruction::Signal(signal) => {
                self.signal_handlers
                    .get(&signal)
                    .ok_or(format!("Unknown signal 0x{signal:X}"))?(self);

                Ok(())
            }
        }
    }
}
