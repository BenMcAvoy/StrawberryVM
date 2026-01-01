use jasm::runner::run;

use std::env::{self, args};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::panic;

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

    // Note: Panic hook must be `Send + Sync`, so it can't capture `Machine`.
    panic::set_hook(Box::new(|info| {
        eprintln!("Runtime error!");
        eprintln!("{info}");

        if let Some(status) = strawberryvm::panic_report::get_last_status() {
            eprintln!("{status}");
        }
    }));

    run(&load_program())?;

    Ok(())
}
