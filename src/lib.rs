mod memory;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::vm::{Machine, MEMORY_KILO_BYTES};

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
}
