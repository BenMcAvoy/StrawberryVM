use std::io::Write;

use strawberryvm::prelude::*;

/// Basic program to continuously get input from the user,
/// write it to the machines memory, and step the machine.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = Machine::new();
    let mut mem_index = 0;

    vm.define_handler(0xf, |machine| println!("{}", machine.status()));

    loop {
        let mut buf = String::new();

        let prompt = if mem_index % 2 == 0 {
            "Opcode  >> "
        } else {
            "Operand >> "
        };

        print!("{prompt}");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut buf)?;
        let input = buf.trim();

        if input == "quit" {
            break;
        }

        let hex = match u8::from_str_radix(&input[2..], 16) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {e:?}");
                continue;
            }
        };

        match vm.memory.write(mem_index, hex) {
            Ok(()) => (),
            Err(e) => {
                println!("Failed: {e:?}");
                println!("{}", vm.status());
                println!("-- Restarting VM! --");
                vm = Machine::new();
            }
        };

        mem_index += 1;

        if mem_index % 2 == 0 {
            match vm.step() {
                Ok(()) => {}
                Err(e) => {
                    println!("Failed: {e:?}");
                    println!("{}", vm.status());
                    println!("-- Restarting VM! --");
                    vm = Machine::new();
                }
            }
        }
    }

    Ok(())
}
