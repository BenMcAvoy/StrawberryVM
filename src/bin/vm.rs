use strawberryvm::vm::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()
}
