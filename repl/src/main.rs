use std::io::{stdin, stdout, Write};

use jasm::{assembler::Assembler, helpers::DynErr};
use strawberryvm::prelude::*;

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

fn reset(machine: &mut Machine, e: DynErr) -> u16 {
    println!("Failed: {e:?}");
    println!("{}", machine.status());
    println!("-- Restarting VM! --");
    *machine = Machine::new();
    0
}

fn main() -> Result<(), DynErr> {
    let mut machine = Machine::new();
    let mut mem_index = 0;

    machine.define_handler(0xF0, sig_halt);
    machine.define_handler(0xF1, log_reg_a);
    machine.define_handler(0xF2, log_regs);
    machine.define_handler(0xF3, mem_dump);

    let assembler = Assembler();

    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.starts_with(';') || input.is_empty() {
            continue;
        }

        if input == "break" || input == "quit" {
            break;
        }

        if input == "restart" {
            mem_index = reset(&mut machine, "Restarting".into());
            continue;
        }

        let dbyte = match assembler.parse_line(String::from(input), 0) {
            Ok(v) => v,
            Err(e) => {
                mem_index = reset(&mut machine, e);
                continue;
            }
        };

        match machine.memory.write_u16(mem_index, dbyte) {
            Ok(()) => mem_index += 2,
            Err(e) => {
                mem_index = reset(&mut machine, Box::new(e));
                continue;
            }
        };

        match machine.step() {
            Ok(()) => (),
            Err(e) => {
                mem_index = reset(&mut machine, e);
                continue;
            }
        }
    }

    Ok(())
}
