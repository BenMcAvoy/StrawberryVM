use jasm::arguments::usage;
use jasm::arguments::Arguments;
use jasm::assembler::Assembler;
use jasm::helpers::DynErr;
use jasm::parsing::validate_jam;
use jasm::parsing::JamParseError;
use jasm::runner::run;

use jasm::passes::pre::remove_comments_pass;
use strawberryvm::prelude::Instruction;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

/// Jasm - Jam assembler
///
/// Usage: jasm <program.jam> [options]
///
/// -i, --input   | Input file (can also just type the name rather than specifying as an argument.)
/// -o, --output  | Output file (where to write the file to)
/// -r, --run     | Automatically run after compiling (won't write a file when this flag is used unless output argument is specified.)
/// -R, --reverse | Disassemble a binary back into Jam.
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

    if args.reverse {
        let file = File::open(Path::new(&args.input.clone().unwrap()))
            .map_err(|x| format!("Failed to open: {x}"))?;

        let mut reader = BufReader::new(file);
        let mut program: Vec<u8> = Vec::new();

        reader
            .read_to_end(&mut program)
            .map_err(|x| format!("read: {}", x))?;

        let mut index = 0;

        while index < program.len() {
            if index + 2 <= program.len() {
                let value = u16::from_le_bytes([program[index], program[index + 1]]);
                let instruction = Instruction::try_from(value)?;
                println!("{}", instruction);
                index += 2;
            } else {
                return Err("Incomplete data for instruction decoding.".into());
            }
        }

        exit(0)
    }

    if let Some(input) = args.input {
        if let Ok(mut file) = File::open(&input) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");

            let lines: Vec<String> = contents.lines().map(String::from).collect();
            let bytes = assembler.parse_vec(&lines);

            let bytes = match bytes {
                Ok(v) => Ok(v),
                Err(e) => match e.downcast::<JamParseError>() {
                    Ok(v) => Err(v),
                    Err(e) => {
                        eprintln!("{e}");
                        exit(1);
                    }
                },
            };

            let bytes = match bytes {
                Ok(v) => v,
                Err(jam_error) => match *jam_error {
                    JamParseError::InvalidOpCode(violation, line) => {
                        println!("Unknown opcode violation `{violation}` at line {line}");

                        // We know this line exists, unwrapping is fine.
                        let violating_line = lines.get(line - 1).unwrap();

                        println!("\n{violating_line}");
                        let line_width = violating_line.len();
                        println!("{:~<line_width$}", "");
                        exit(1);
                    }

                    _ => {
                        eprintln!("{jam_error}");
                        exit(1);
                    }
                },
            };

            let lines: Vec<String> = lines
                .iter()
                .filter_map(|line| remove_comments_pass(line))
                .collect();

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

                    JamParseError::Empty(_) => (),
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
