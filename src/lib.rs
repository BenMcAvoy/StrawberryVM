mod memory;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::vm::{Machine, Register, MEMORY_KILO_BYTES};

    // Tests for failure (these should fail!)
    #[test]
    fn unknown_instruction() {
        let mut machine = Machine::new();
        machine.memory.write(0, 0xF).unwrap();
        assert!(machine.step().is_err())
    }

    #[test]
    fn out_of_bound() {
        let mut machine = Machine::new();

        assert!(machine
            .memory
            .write((MEMORY_KILO_BYTES * 1024 + 1) as u16, 0xF)
            .is_err());
    }

    #[test]
    fn addition() {
        let mut machine = Machine::new();

        machine.memory.write(0, 0x1).unwrap();
        machine.memory.write(1, 10).unwrap();
        machine.memory.write(2, 0x1).unwrap();
        machine.memory.write(3, 8).unwrap();
        machine.memory.write(4, 0x3).unwrap();
        machine.memory.write(6, 0x2).unwrap();
        machine.memory.write(7, 0).unwrap();

        machine.step().unwrap(); // PUSH 10
        machine.step().unwrap(); // PUSH 8
        machine.step().unwrap(); // ADDSTACK
        machine.step().unwrap(); // POPREGISTER A

        assert_eq!(machine.get_register(Register::A), 18);
    }
}
