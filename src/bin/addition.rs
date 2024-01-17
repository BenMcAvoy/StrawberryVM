use strawberryvm::prelude::*;

fn sig_halt(vm: &mut Machine) -> Result<(), String> {
    vm.machine_halted = true;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    vm.define_handler(0x90, sig_halt);

    write_memory!(vm,
        0 => 0x1,
        1 => 0xA,
        2 => 0x1,
        3 => 0x8,
        4 => 0x3,
        5 => 0x0,
        6 => 0x2,
        7 => 0x0,
        8 => 0x5,
        9 => 0x90
    );

    while !vm.machine_halted {
        vm.step()?;
    }

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
