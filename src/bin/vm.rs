use strawberryvm::vm::{Machine, Register};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();

    // vm.memory.write(0, 0x0)?;

    vm.memory.write(0, 0x1)?;
    vm.memory.write(1, 10)?;
    vm.memory.write(2, 0x1)?;
    vm.memory.write(3, 8)?;
    vm.memory.write(4, 0x3)?;
    vm.memory.write(6, 0x2)?;
    vm.memory.write(7, 0)?;

    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {}", vm.get_register(Register::A));

    Ok(())
}
