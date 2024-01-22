use arguments::Arguments;
use assembler::Assembler;
use runner::run;

mod assembler;
mod arguments;
mod parsing;
mod passes;
mod runner;

// use crate::assembler::Assembler;

use crate::arguments::usage;
use crate::parsing::JamParseError;
use crate::parsing::validate_jam;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::process::exit;

/// Jasm - Jam assembler
///
/// Usage: jasm <program.jam> [options]
///
/// -i, --input  | Input file (can also just type the name rather than specifying as an argument.)
/// -o, --output | Output file (where to write the file to)
/// -r, --run    | Automatically run after compiling (won't write a file when this flag is used unless output argument is specified.)
///
/// Notes:
///     If simply just the file name is specified or just an input flag is specified, the program will take the file stem and write out a binary file with the same file stem.
///     E.g. if you run `jasm test.jam` it will write to `test.bin`.
///
/// Example usages:
///     jasm main.jam -o out.bin
///     jasm main.jam -r
///     jasm main.jam
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::default();
    args.populate_args();

    if let Some(input) = args.input {
        let mut file = match File::open(input.clone()) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    eprintln!("Jam file {input:?} was not found!");
                    exit(1);
                } else {
                    eprintln!("Error loading file {e}");
                    usage();
                    exit(1);
                }
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let lines: Vec<&str> = contents.lines().collect();

        let mut assembler = Assembler::new(&lines);
        if let Err(e) = validate_jam(&lines) {
            eprintln!("Encountered error when validating!");

            match e {
                JamParseError::InvalidOpCode(opcode, line) => {
                    eprintln!("Invalid opcode `{opcode}` on line {line}");

                    let line = lines.get(line - 1).unwrap();
                    let width = line.len();

                    eprintln!("\n{line}");
                    eprintln!("{:~<width$}", "");
                }
            }

            exit(1);
        };

        assembler.clean_passes();

        // dbg!(assembler.input);
        assembler.parse_input()?;

        if args.run {
            run(&assembler.output)?;
        }

        if !args.run || args.output.is_some() {
            let out_path = match args.output {
                Some(v) => v,
                None => {
                    let stem = Path::new(&input).file_stem().unwrap().to_str().unwrap();
                    format!("{stem}.bin")
                }
            };

            let mut file = File::create(out_path)?;
            file.write_all(&assembler.output)?;
        }
    }

    Ok(())
}
