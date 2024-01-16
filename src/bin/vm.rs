use strawberryvm::vm::{Machine, Register};
use strawberryvm::write_memory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    write_memory!(vm,
        0 => 0x1,
        1 => 0xA,
        2 => 0x1,
        3 => 0x8,
        4 => 0x3,
        6 => 0x2,
        7 => 0x0
    );

    // Loop until we get `Nop`
    // TODO: Use hault!
    while vm.step()? {}

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
