#[allow(unused_imports)]
use std::io::{self, Write};

mod command;
use command::Command;
mod custom_command;
use custom_command::CustomCommand;
mod built_in_command;
use built_in_command::BuiltInCommand;

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
    let input_parts = input.split_whitespace().collect::<Vec<&str>>();
    let command = input_parts.first().unwrap_or(&"").to_string();
    let argument = input_parts
        .iter()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let command: Box<dyn Command> = BuiltInCommand::try_from_string(command.clone(), argument.clone())
        .map(|c| Box::new(c) as Box<dyn Command>)
        .unwrap_or_else(|_| Box::new(CustomCommand { command, argument }));

    command.execute();
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
