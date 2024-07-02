use strawberryvm::Machine;

fn main() {
    let mut machine = Machine::default(); // Create a new machine
    machine.iterate(5); // Run the machine for 5 iterations

    // while machine.step().is_ok() {} // Run the machine until it fails to run again
}
