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
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);

    let mut file = match File::open(Path::new(&args[1])) {
        Err(e) => panic!("Failed to open file: {e}"),
        Ok(file) => file,
    };

    let mut program: Vec<u8> = Vec::new();
    file.read_to_end(&mut program)?;

    let mut vm = Machine::new();

    vm.define_handler(0x90, sig_hault);

    vm.memory.load(&program, 0)?;
    while !vm.machine_halted {
        vm.step()?;
    }

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
