use strawberryvm::Machine;

fn main() {
    let mut machine = Machine::default(); // Create a new machine

    let _ = machine.step();
    let _ = machine.step();
    let _ = machine.step();
    let _ = machine.step();

    // while machine.step().is_ok() {} // Run the machine until it fails to run again
}
