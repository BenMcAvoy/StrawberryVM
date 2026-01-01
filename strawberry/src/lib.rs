//! A fantasy virtual machine with limits on resources.
//!
//! ## Instructions
//!
//! | Name          | Arguments                                        | Description                                                                         |
//! |---------------|--------------------------------------------------|-------------------------------------------------------------------------------------|
//! | No Operation  | None                                             | Does nothing.                                                                       |
//! | Push          | u8 (8-bit value to push)                         | Pushes an 8-bit value onto the stack.                                               |
//! | Pop Register  | Register (destination register)                  | Pops a value from the stack into the specified register.                            |
//! | Push Register | Register (source register)                       | Pushes the value of the specified register onto the stack.                          |
//! | Add Stack     | None                                             | Adds the top two values on the stack.                                               |
//! | Add Register  | Two Registers (operands)                         | Adds the values of two registers and stores the result in the destination register. |
//! | Signal        | u8 (signal value)                                | Sends a signal with an 8-bit value.                                                 |
//! | Jump          | u8 (target address)                              | Jumps to the specified address in the program.                                      |
//! | `ShiftLeft`   | Register (target register) and u8 (shift amount) | Left shifts a specific register by a certain amount.                                |
//!
//! ## Reserved symbols
//! | Symbol | Use               |
//! |--------|-------------------|
//! | $      | Hexadecimal value |
//! | %      | Binary value      |
//! | ^      | Label value       |
//!
//! ## *Hopes* for this project
//! - Turing completion
//! - Custom assembly
//! - Possible language implementation (C or Lua)
//! - Video card
//! - Ability to render games
//!
//! # Example usage:
//! ```rust
//! use strawberryvm::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut vm = Machine::new();
//!
//!     vm.define_handler(0xF0, |machine| machine.machine_halted = true);
//!
//!     write_memory!(vm,
//!        // Push 10; Pop A; Push 8; Pop B; Add A B; Signal 0xF0 (halt)
//!        0 => 0x10,
//!        1 => 0x0A,
//!        2 => 0x11,
//!        3 => 0x00,
//!        4 => 0x10,
//!        5 => 0x08,
//!        6 => 0x11,
//!        7 => 0x01,
//!        8 => 0x20,
//!        9 => 0x10,
//!        10 => 0x50,
//!        11 => 0xF0
//!     );
//!
//!     while !vm.machine_halted {
//!         vm.step()?;
//!     }
//!
//!     println!("A = {}", vm.get_register(Register::A));
//!
//!     Ok(())
//! }
//! ```

mod macros;
mod memory;
mod op;
pub mod panic_report;
mod register;
mod vm;

/// Can be included to get everything useful
/// for the machine
pub mod prelude {
    pub use crate::write_memory;

    pub use crate::op::*;
    pub use crate::register::*;
    pub use crate::vm::*;
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::{
        register::Register,
        vm::{Machine, MEMORY_KILO_BYTES},
        write_memory,
    };

    fn sig_halt(vm: &mut Machine) {
        vm.machine_halted = true;
    }

    // Tests for failure (these should fail!)
    #[test]
    fn unknown_instruction() {
        let mut machine = Machine::new();
        // Low byte is opcode; 0xFF is not a valid opcode.
        machine.memory.write(0, 0xFF).unwrap();
        assert!(machine.step().is_err());
    }

    #[test]
    fn out_of_bounds() -> Result<(), Box<dyn std::error::Error>> {
        let mut machine = Machine::new();

        assert!(machine
            .memory
            .write(u16::try_from(MEMORY_KILO_BYTES * 1024 + 1)?, 0xf)
            .is_err());

        Ok(())
    }

    #[test]
    fn addition() -> Result<(), Box<dyn std::error::Error>> {
        let mut machine = Machine::new();
        machine.define_handler(0xf0, sig_halt);

        write_memory!(machine,
         // Push 10; Pop A; Push 8; Pop B; Add A B; Signal 0xF0
         0 => 0x10,
         1 => 0x0a,
         2 => 0x11,
         3 => 0x00,
         4 => 0x10,
         5 => 0x08,
         6 => 0x11,
         7 => 0x01,
         8 => 0x20,
         9 => 0x10,
         10 => 0x50,
         11 => 0xf0
        );

        machine.step().unwrap(); // Push 10
        machine.step().unwrap(); // Pop A
        machine.step().unwrap(); // Push 8
        machine.step().unwrap(); // Pop B
        machine.step().unwrap(); // Add A B
        machine.step().unwrap(); // Signal 0xF0

        assert_eq!(machine.get_register(Register::A), 18);

        Ok(())
    }

    #[test]
    fn subsequent_addition() -> Result<(), Box<dyn std::error::Error>> {
        for _ in 1..=2 {
            let mut machine = Machine::new();
            machine.define_handler(0xf0, sig_halt);

            write_memory!(machine,
             // Push 10; Pop A; Push 8; Pop B; Add A B; Signal 0xF0
             0 => 0x10,
             1 => 0x0a,
             2 => 0x11,
             3 => 0x00,
             4 => 0x10,
             5 => 0x08,
             6 => 0x11,
             7 => 0x01,
             8 => 0x20,
             9 => 0x10,
             10 => 0x50,
             11 => 0xf0
            );

            machine.step().unwrap(); // Push 10
            machine.step().unwrap(); // Pop A
            machine.step().unwrap(); // Push 8
            machine.step().unwrap(); // Pop B
            machine.step().unwrap(); // Add A B
            machine.step().unwrap(); // Signal 0xF0

            assert_eq!(machine.get_register(Register::A), 18);
        }

        Ok(())
    }
}
