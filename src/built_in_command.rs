use crate::{command, locate_in_path};

pub enum BuiltInCommand {
    Echo(String),
    Cd(String),
    Pwd,
    Type(String),
    Exit(i32),
}

impl BuiltInCommand {
    pub fn try_from_string(command: String, argument: String) -> Result<BuiltInCommand, ()> {
        BuiltInCommand::try_from(&command).map(|c| match c {
            BuiltInCommand::Echo(_) => BuiltInCommand::Echo(argument),
            BuiltInCommand::Cd(_) => BuiltInCommand::Cd(argument),
            BuiltInCommand::Type(_) => BuiltInCommand::Type(argument),
            BuiltInCommand::Exit(_) => BuiltInCommand::Exit(argument.parse().unwrap()),
            _ => c,
        })
    }
}

impl TryFrom<&String> for BuiltInCommand {
    type Error = ();

    fn try_from(command: &String) -> Result<Self, Self::Error> {
        match command.to_lowercase().as_str() {
            "echo" => Ok(BuiltInCommand::Echo("".to_string())),
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
            BuiltInCommand::Echo(message) => println!("{}", message),
            BuiltInCommand::Cd(new_dir) => {
                let path = std::path::Path::new(new_dir);
                if path.exists() {
                    std::env::set_current_dir(path).unwrap();
                } else {
                    println!("cd: {}: No such file or directory", new_dir);
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
