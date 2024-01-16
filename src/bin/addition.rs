use strawberryvm::vm::{Machine, Register};

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn sig_hault(vm: &mut Machine) -> Result<(), String> {
    vm.machine_halted = true;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    vm.define_handler(0x90, sig_hault);

    vm.memory.load(&program, 0)?;
    while !vm.machine_halted {
        vm.step()?;
    }

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
