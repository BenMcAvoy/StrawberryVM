use jasm::arguments::usage;
use jasm::arguments::Arguments;
use jasm::assembler::Assembler;
use jasm::helpers::DynErr;
use jasm::parsing::validate_jam;
use jasm::parsing::JamParseError;
use jasm::runner::run;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
fn main() -> Result<(), DynErr> {
    let mut args = Arguments::default();
    args.populate_args();

    let assembler = Assembler();

    if let Some(input) = args.input {
        if let Ok(mut file) = File::open(&input) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");

            let lines: Vec<String> = contents.lines().map(String::from).collect();
            let bytes = assembler.parse_vec(&lines)?;

            if let Err(e) = validate_jam(&lines) {
                eprintln!("Encountered error when validating!");

                match e {
                    JamParseError::InvalidOpCode(opcode, line) => {
                        eprintln!("Invalid opcode `{}` on line {}", opcode, line);

                        if let Some(line_content) = lines.get(line - 1) {
                            let width = line_content.len();
                            eprintln!("\n{}\n{:~<width$}", line_content, "", width = width);
                        }
                    }
                }

                std::process::exit(1);
            }

            if args.run {
                run(&bytes)?;
            }

            if !args.run || args.output.is_some() {
                let out_path = args.output.unwrap_or_else(|| {
                    let stem = Path::new(&input).file_stem().unwrap().to_str().unwrap();
                    format!("{}.bin", stem)
                });

                let mut file = File::create(out_path)?;
                file.write_all(&bytes)?;
            }
        } else {
            eprintln!("Error loading file {:?}", input);
            usage();
            std::process::exit(1);
        }
    }

    Ok(())
}
