use strawberryvm::prelude::{Machine, Register};

fn sig_halt(vm: &mut Machine) {
    vm.machine_halted = true;
}

fn log_reg_a(vm: &mut Machine) {
    println!("A = {}", vm.get_register(Register::A));
}

fn log_regs(vm: &mut Machine) {
    println!("{}", vm.status());
}

fn mem_dump(vm: &mut Machine) {
    println!("{}", vm.memory.dump());
}

pub fn apply_signals(vm: &mut Machine) {
    vm.define_handler(0xF0, sig_halt);
    vm.define_handler(0xF1, log_reg_a);
    vm.define_handler(0xF2, log_regs);
    vm.define_handler(0xF3, mem_dump);
}
