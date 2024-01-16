use std::{env, fs::File, io::Read, path::Path};

use strawberryvm::vm::{Machine, Register};

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

    vm.memory.load(&program, 0)?;

    // Loop until we get `Nop`
    // TODO: Use hault!
    while vm.step()? {}

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
