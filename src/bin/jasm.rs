use strawberryvm::prelude::*;

use std::collections::HashMap;
use std::env;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::process::exit;

/// The main struct for the assembler containing
/// important information for creating a resultant
/// binary that the machine can run.
struct Assembler {
    pub output: Vec<u8>,

    labels: HashMap<String, u8>,

    lines: Vec<String>,
}

impl Assembler {
    /// Creates a new assembler from some labels and the lines of an
    /// assembly file as input.
    pub fn new(labels: &HashMap<&str, u8>, lines: &[&str]) -> Self {
        let labels: HashMap<String, u8> =
            labels.iter().map(|(k, &v)| ((*k).to_string(), v)).collect();

        let lines = lines.iter().map(ToString::to_string).collect();
        let output = Vec::new();

        Self {
            output,
            labels,
            lines,
        }
    }
}

impl Assembler {
    /// Used to parse a numeric based on whether it is binary,
    /// decimal, or hexadecimal.
    fn parse_numeric(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
        let first = s.chars().next().unwrap();
        let (num, radix) = match first {
            '$' => (&s[1..], 16),
            '%' => (&s[1..], 2),
            _ => (s, 10),
        };

        Ok(u8::from_str_radix(num, radix)?)
    }

    /// Used to parse a register from a string into an actual
    /// register than can be encoded into binary
    fn parse_register(s: &str) -> Result<Register, Box<dyn std::error::Error>> {
        match s {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            "C" => Ok(Register::C),
            _ => Err(format!("Unknown register {s}").into()),
        }
    }

    /// Used for the jump instruction to detect if the user is trying
    /// to jump to a label they have defined or just a pc count.
    ///
    /// This is automatically done here and returns a simple u8 the
    /// pc register is set to.
    fn parse_label(&self, s: &str) -> Result<u8, Box<dyn std::error::Error>> {
        if let Ok(u8) = Self::parse_numeric(s) {
            return Ok(u8);
        }

        if let Some(s) = s.strip_prefix('^') {
            if let Some(u8) = self.labels.get(s) {
                return Ok(*u8);
            }
        }

        Err(format!("Couldn't parse {s}").into())
    }

    fn assert_length(parts: &[&str], n: usize) -> Result<(), Box<dyn std::error::Error>> {
        if !parts.len() == n {
            return Err(format!("Expected {} got {}", n, parts.len()).into());
        }

        Ok(())
    }

    // TODO: Very good candidate for derive macro.
    /// Used to take a line and figure out what it's opcode and operand are to
    /// create an instruction that can be encoded.
    fn handle_line(&self, parts: &[&str]) -> Result<Instruction, Box<dyn std::error::Error>> {
        let opcode = parts[0]
            .parse()
            .map_err(|_| format!("Unknown opcode: {}", parts[0]))?;

        match opcode {
            OpCode::Push => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::Push(Self::parse_numeric(parts[1])?))
            }

            OpCode::AddStack => {
                Self::assert_length(parts, 1)?;
                Ok(Instruction::AddStack)
            }

            OpCode::AddReg => {
                let (r1, r2) = (
                    Self::parse_register(parts[1])?,
                    Self::parse_register(parts[2])?,
                );
                Ok(Instruction::AddReg(r1, r2))
            }

            OpCode::PopReg => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::PopReg(Self::parse_register(parts[1])?))
            }

            OpCode::PushReg => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::PushReg(Self::parse_register(parts[1])?))
            }

            OpCode::Signal => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::Signal(Self::parse_numeric(parts[1])?))
            }

            OpCode::Nop => {
                Self::assert_length(parts, 1)?;
                Ok(Instruction::Nop)
            }

            OpCode::Jmp => {
                Self::assert_length(parts, 2)?;
                Ok(Instruction::Jmp(self.parse_label(parts[1])?))
            }
        }
    }

    /// Central function that takes the input and turns it into actual assembly using helper
    /// functions
    fn parse_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for line in self
            .lines
            .iter()
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with(';'))
            .filter(|line| !line.ends_with(':'))
        {
            let parts: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();

            if parts.is_empty() {
                continue;
            }

            let instruction = self.handle_line(&parts)?;
            let raw_instruction: u16 = instruction.encode_u16();

            let lower = (raw_instruction & 0xff) as u8;
            let upper = (raw_instruction >> 8) as u8;

            self.output.push(lower);
            self.output.push(upper);
        }

        Ok(())
    }
}

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
