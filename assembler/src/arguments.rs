use std::{env::args, process::exit};

pub fn usage() {
    println!(
        "
Jasm - Jam assembler

Usage: jasm <program.jam> [options]

-i, --input  | Input file (can also just type the name rather than specifying as an argument.)
-o, --output | Output file (where to write the file to)
-r, --run    | Automatically run after compiling (won't write a file when this flag is used unless output argument is specified.)

Notes:
    If simply just the file name is specified or just an input flag is specified, the program will take the file stem and write out a binary file with the same file stem.
    E.g. if you run `jasm test.jam` it will write to `test.bin`.

Example usages:
    jasm main.jam -o out.bin
    jasm main.jam -r
    jasm main.jam
"
    );
}

#[derive(Default)]
pub struct Arguments {
    pub input: Option<String>,
    pub output: Option<String>,

    pub run: bool,
}

impl Arguments {
    pub fn populate_args(&mut self) {
        let parts: Vec<String> = args().skip(1).collect();
        if parts.is_empty() {
            usage();
            exit(1);
        }

        let mut result: Vec<(String, Option<String>)> = Vec::new();
        let mut flag: Option<String> = None;

        for part in &parts {
            if part.starts_with("--") || part.starts_with('-') {
                flag = Some(part.to_string());
                result.push((flag.clone().unwrap(), None));
            } else if let Some(ref flg) = flag {
                result.pop();
                result.push((flg.to_string(), Some(part.to_string())));
            }
        }

        if !parts[0].starts_with('-') {
            self.input = Some(parts[0].clone());
        }

        let result: Vec<_> = result
            .iter()
            .map(|(k, v)| (k.as_str(), v))
            .collect();

        for result in result.clone() {
            match result {
                ("-i", v) | ("--input", v) => {
                    self.input = v.clone();
                }

                ("-o", v) | ("--output", v) => {
                    self.output = v.clone();
                }

                ("-r", None) | ("--run", None) => {
                    self.run = true;
                }

                _ => {
                    usage();
                    exit(1);
                }
            }
        }
    }
}
