use strawberryvm::prelude::*;

use crate::assembler::Assembler;

mod assembler;

use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::process::exit;

fn usage() {
    println!(
        "
Jasm - Jam assembler

Example usages:
    jasm main.jam -o out.bin
    jasm main.jam
"
    );

    exit(1);
}

fn parse_args(args: &[String]) -> (String, String) {
    let arg_pairs: Vec<(&str, &str)> = args
        .iter()
        .skip(1)
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|c| (c[0].as_str(), c[1].as_str()))
        .collect();

    let prog_path = arg_pairs
        .iter()
        .find(|&&pair| pair.0 == "-o")
        .map_or_else(|| "out.bin", |&(_, value)| value);

    let file_path = arg_pairs.iter().find(|&&pair| pair.0 == "-i").map_or_else(
        || {
            usage();
            unreachable!()
        },
        |&(_, value)| value,
    );

    (file_path.to_string(), prog_path.to_string())
}

fn sig_halt(vm: &mut Machine) {
    vm.machine_halted = true;
}

fn log_reg_a(vm: &mut Machine) {
    println!("A = {}", vm.get_register(Register::A));
}

fn log_regs(vm: &mut Machine) {
    println!("{}", vm.status());
}

/// Jasm - Jam assembler
///
/// Example usages:
///     jasm main.jam -o out.bin
///     jasm main.jam
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        usage();
    }

    let (file_path, prog_path) = parse_args(&args);

    let file_path = Path::new(&file_path);
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Jam file {file_path:?} was not found!");
                exit(1);
            } else {
                eprintln!("Error loading file {e}");
                usage();
                unreachable!() // Let the compiler know
                               // we won't get here
            }
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let lines: Vec<&str> = contents.lines().map(str::trim_start).collect();

    let mut labels = HashMap::new();

    for (index, line) in lines
        .iter()
        .filter(|line| !line.starts_with(';'))
        .enumerate()
    {
        if !line.ends_with(':') {
            continue;
        }

        if let Some(label) = line.strip_suffix(':') {
            let offset = lines
                .iter()
                .take(index + 1)
                .filter(|line| line.ends_with(':'))
                .count();

            let pc = u8::try_from((index * 2) - offset * 2)?;
            labels.insert(label, pc);
        }
    }

    let mut assembler = Assembler::new(&labels, &lines);
    assembler.parse_input()?;

    let run = args.contains(&String::from("--run")) || args.contains(&String::from("-r"));

    if run {
        let mut vm = Machine::new();

        vm.define_handler(0xF0, sig_halt);
        vm.define_handler(0xF1, log_reg_a);
        vm.define_handler(0xF2, log_regs);

        vm.memory.load(&assembler.output, 0)?;

        while !vm.machine_halted {
            vm.step()?;
        }
    }

    if !run || args.contains(&String::from("-o")) {
        let mut file = File::create(prog_path)?;
        file.write_all(&assembler.output)?;
    }

    Ok(())
}
