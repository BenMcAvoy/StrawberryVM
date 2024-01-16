use std::env;
use std::path::Path;

use std::fs::read_to_string;
use std::io::{stdout, Write};

/// Usage:
/// ./asm file.asm
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 2);

    let file = match read_to_string(Path::new(&args[1])) {
        Err(e) => panic!("Failed to open file: {e}"),
        Ok(file) => file,
    };

    let output: Vec<u8> = file
        .lines()
        .flat_map(|line| line.split_whitespace())
        .filter_map(|token| u8::from_str_radix(token, 16).ok())
        .collect();

    stdout().lock().write_all(&output)?;

    Ok(())
}
