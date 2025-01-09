#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let message = input + ": command not found";
    println!("{}", message);
}
