use strawberryvm::prelude::*;

fn sig_halt(vm: &mut Machine) {
    vm.machine_halted = true;
}

/// Basic program to test basic functions for addition
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    vm.define_handler(0xF0, sig_halt);

    write_memory!(vm,
        0 => 0x1,
        1 => 0xA,
        2 => 0x1,
        3 => 0x8,
        4 => 0x4,
        5 => 0x0,
        6 => 0x2,
        7 => 0x0,
        8 => 0x6,
        9 => 0xF0
    );

    while !vm.machine_halted {
        vm.step()?;
    }

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
