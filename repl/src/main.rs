use jasm::signals::apply_signals;
use jasm::assembler::Assembler;
use jasm::helpers::DynErr;

use strawberryvm::prelude::*;

use crate::helpers::get_input;

mod helpers;

fn process_input(
    assembler: &Assembler,
    machine: &mut Machine,
    mem_index: u16,
    input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let dbyte = assembler.parse_line(String::from(input), 0)?;
    machine.memory.write_u16(mem_index, dbyte)?;
    machine.step()?;
    Ok(())
}

fn main() -> Result<(), DynErr> {
    let mut machine = Machine::new();
    let mut mem_index = 0;

    apply_signals(&mut machine);

    let assembler = Assembler();

    loop {
        let input = get_input(">>> ");
        let input = input.trim();

        if input.starts_with(';') || input.is_empty() {
            continue;
        }

        if input == "break" || input == "quit" || input == "exit" {
            break;
        }

        if input == "restart" {
            println!("{}", machine.status());
            println!("-- Restarting VM! --");
            mem_index = 0;
            machine = Machine::new();
            continue;
        }

        if let Err(e) = process_input(&assembler, &mut machine, mem_index, input) {
            println!("Failed: {e:?}");
            println!("{}", machine.status());
            println!("-- Restarting VM! --");
            machine = Machine::new();
            mem_index = 0;
        };

        mem_index += 2;
    }

    Ok(())
}
