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
            let arguments = input.trim().split_whitespace().skip(1).collect::<Vec<&str>>();
            match command {
                "echo" => echo(input),
                "type" => type_command(input),
                _ => handle_arbitrary_command(command, arguments),
            }
        }
    }
}

fn handle_arbitrary_command(command: &str, arguments: Vec<&str>) {
    match locate_in_path(command) {
        Some(full_path) => execute(command.to_string(), arguments),
        None => println!("{}: command not found", command),
    }
}

fn execute(command_path: String, arguments: Vec<&str>) {
    let mut command = std::process::Command::new(command_path);
    command.args(arguments);
    command.spawn().unwrap().wait().unwrap();
}

fn type_command(input: String) {
    let _command = input.trim().split_whitespace().skip(1).next().unwrap();

    match _command {
        "type" => print_builtin("type"),
        "exit" => print_builtin("exit"),
        "echo" => print_builtin("echo"),
        _ => find_in_path(_command),
    }
}

fn find_in_path(_command: &str) {
    match locate_in_path(_command) {
        Some(full_path) => println!("{} is {}", _command, full_path),
        None => println!("{}: not found", _command),
    }
}

fn locate_in_path(_command: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap();
    let paths = path.split(":").collect::<Vec<&str>>();

    for p in paths {
        let full_path = format!("{}/{}", p, _command);
        if std::path::Path::new(&full_path).exists() {

            return Some(full_path);
        }
    }
    None
}

fn print_builtin(x: &str) {
    println!("{} is a shell builtin", x)
}

fn echo(input: String) {
    let message = input
        .trim()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");

    println!("{}", message);
}
