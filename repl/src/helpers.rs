use std::io::{stdin, stdout, Write};

pub fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt}");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    input
}
