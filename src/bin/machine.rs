use strawberryvm::vm::{Machine, Register};

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn sig_halt(vm: &mut Machine) -> Result<(), String> {
    vm.machine_halted = true;
    Ok(())
}

fn log_reg_a(vm: &mut Machine) -> Result<(), String> {
    println!("A = {}", vm.get_register(Register::A));
    Ok(())
}

fn log_regs(vm: &mut Machine) -> Result<(), String> {
    println!(" A = {}", vm.get_register(Register::A));
    println!(" B = {}", vm.get_register(Register::B));
    println!(" C = {}", vm.get_register(Register::C));
    println!(" M = {}", vm.get_register(Register::M));
    println!("SP = {}", vm.get_register(Register::SP));
    println!("PC = {}", vm.get_register(Register::PC));
    println!("BP = {}", vm.get_register(Register::BP));
    println!("FL = {}", vm.get_register(Register::FL));
    Ok(())
}

fn load_program() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);

    let mut file = match File::open(Path::new(&args[1])) {
        Err(e) => panic!("Failed to open file: {e}"),
        Ok(file) => file,
    };

    let mut program: Vec<u8> = Vec::new();
    file.read_to_end(&mut program).unwrap();

    program
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    vm.define_handler(0xF0, sig_halt);
    vm.define_handler(0xF1, log_reg_a);
    vm.define_handler(0xF2, log_regs);

    vm.memory.load(&load_program(), 0)?;

    while !vm.machine_halted {
        vm.step()?;
    }

    Ok(())
}
