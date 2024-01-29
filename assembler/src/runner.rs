use strawberryvm::prelude::*;

use crate::signals::apply_signals;

/// Usage: ./machine <prog.bin>
pub fn run(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    apply_signals(&mut vm);

    vm.memory.load(bytes, 0)?;

    while !vm.machine_halted {
        vm.step()?;
    }

    Ok(())
}
