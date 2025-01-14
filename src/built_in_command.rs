use crate::{command, locate_in_path};

pub enum BuiltInCommand {
    Echo(Vec<String>),
    Cd(String),
    Pwd,
    Type(String),
    Exit(i32),
}

impl BuiltInCommand {
    pub fn try_from_string(command: String, arguments: Vec<String>) -> Result<BuiltInCommand, ()> {
        BuiltInCommand::try_from(&command).map(|c| match c {
            BuiltInCommand::Echo(_) => BuiltInCommand::Echo(arguments),
            BuiltInCommand::Cd(_) => BuiltInCommand::Cd(arguments.first().unwrap().to_string()),
            BuiltInCommand::Type(_) => BuiltInCommand::Type(arguments.first().unwrap().to_string()),
            BuiltInCommand::Exit(_) => BuiltInCommand::Exit(arguments.first().unwrap().parse().unwrap()),
            _ => c,
        })
    }
}

impl TryFrom<&String> for BuiltInCommand {
    type Error = ();

    fn try_from(command: &String) -> Result<Self, Self::Error> {
        match command.to_lowercase().as_str() {
            "echo" => Ok(BuiltInCommand::Echo(vec![])),
            "cd" => Ok(BuiltInCommand::Cd("".to_string())),
            "pwd" => Ok(BuiltInCommand::Pwd),
            "type" => Ok(BuiltInCommand::Type("".to_string())),
            "exit" => Ok(BuiltInCommand::Exit(0)),
            _ => Err(()),
        }
    }
}

impl command::Command for BuiltInCommand {
    fn execute(&self) {
        match self {
            BuiltInCommand::Echo(messages) => println!("{}", messages.join(" ")),
            BuiltInCommand::Cd(new_dir) => {
                if new_dir.starts_with('~') {
                    let home_dir = std::env::var("HOME").unwrap();
                    let new_dir = new_dir.replacen("~", &home_dir, 1);
                    std::env::set_current_dir(new_dir).unwrap();
                } else {
                    let path = std::path::Path::new(new_dir);
                    if path.exists() {
                        std::env::set_current_dir(path).unwrap();
                    } else {
                        println!("cd: {}: No such file or directory", new_dir);
                    }
                }
            }
            BuiltInCommand::Pwd => println!("{}", std::env::current_dir().unwrap().display()),
            BuiltInCommand::Type(command) => {
                if BuiltInCommand::try_from(command).is_ok() {
                    println!("{} is a shell builtin", command);
                } else {
                    match locate_in_path(command) {
                        Some(full_path) => println!("{} is {}", command, full_path),
                        None => println!("{}: not found", command),
                    }
                }
            }
            BuiltInCommand::Exit(code) => std::process::exit(*code),
        }
    }
}
