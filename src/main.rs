#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        process_input();
    }
}

fn process_input() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "exit 0" => std::process::exit(0),
        _ => {
            let command = input.trim().split_whitespace().next().unwrap();
            match command {
                "echo" => echo(input),
                _ => println!("{}: command not found", command),
            }
        },
    }
}

fn echo(input: String) {
    let message = input.trim().split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
    println!("{}", message);
}
