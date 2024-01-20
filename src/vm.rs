use crate::op::Instruction;
use crate::register::Register;
use std::collections::HashMap;

use crate::memory;

pub(crate) const REGISTER_COUNT: usize = 8;
pub(crate) const MEMORY_KILO_BYTES: usize = 8;

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
        }
    }

    /// Returns a table of each register as a string
    /// This is only really useful for debugging and
    /// is not really useful for anything else.
    #[must_use]
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
            String::new(),
            format!("   {:^line_width$}", "» Registers «"),
            format!(" ┌{:─<line_width$}┐",""),
            format!(" │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │", "A", "B", "C", "M", "SP", "PC", "BP", "FLAGS"),
            format!(" │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^5} │", a, b, c, m, sp, pc, bp, flags),
            format!(" └{:─<line_width$}┘", ""),
            String::new(),
        ];

        lines.join("\n")
    }

    /// Returns the value of of a register inside of the machine
    /// Typically only useful for debugging or pulling data out
    /// of the machine as the host.
    #[must_use]
    pub fn get_register(&self, r: Register) -> u16 {
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

    /// Used to step the machine forward, can be called by a
    /// virtual "clock" to simulate cpu cycles.
    ///
    /// # Errors
    /// This can error if memory read fails (see `read_u16`).
    /// It can also fail if memory popping fails.
    /// It can also fail if a signal is non-existant.
    /// It can also fail if the instruction is invalid.
    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pc = self.registers[Register::PC as usize];
        self.registers[Register::PC as usize] += 2;
        let instruction = self.memory.read_u16(pc)?;
        let op = Instruction::try_from(instruction)?;

        println!("{pc:0>4} │ Got instruction {op:?}");

        match op {
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => self.push(u16::from(v)),

            Instruction::PopReg(r) => {
                let popped = self.pop()?;
                self.registers[r as usize] = popped;
                Ok(())
            }

            Instruction::PushReg(r) => self.push(self.registers[r as usize]),

            Instruction::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;

                self.push(a + b)
            }

            Instruction::AddReg(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }

            Instruction::Signal(signal) => {
                self.signal_handlers
                    .get(&signal)
                    .ok_or(format!("Unknown signal 0x{signal:X}"))?(self);

                Ok(())
            }

            Instruction::Jmp(reg) => {
                self.registers[Register::PC as usize] = u16::from(reg);
                Ok(())
            }
        }
    }
}
