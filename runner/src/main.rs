use strawberryvm::prelude::Machine;

use jasm::runner::run;
use jasm::signals::apply_signals;

use std::env::{self, args};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

fn load_program() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);

    let mut file = match File::open(Path::new(&args[1])) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {e}");
            exit(1)
        }
    };

    let mut program: Vec<u8> = Vec::new();
    file.read_to_end(&mut program).unwrap();

    program
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if args().len() != 2 {
        println!("Usage: `svm prog.bin`");
        exit(1);
    }

    let mut machine = Machine::new();

    apply_signals(&mut machine);

    run(&load_program())?;

    Ok(())
}
