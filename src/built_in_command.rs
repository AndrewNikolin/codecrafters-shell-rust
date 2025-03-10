use std::fs::File;
use crate::{command, locate_in_path};

pub struct BuiltInCommandWrapper {
    pub command: BuiltInCommand,
    pub stdout: Option<File>,
    pub stderr: Option<File>,
}

impl command::Command for BuiltInCommandWrapper {
    fn execute(&self) {
        let result:Option<String> = self.command.execute();
        
        if let Some(message) = result {
            if let Some(file) = &self.stdout {
                use std::io::Write;
                file.try_clone().unwrap().write_all(message.as_bytes()).unwrap();
            } else {
                println!("{}", message);
            }
        }
    }

    fn stdout(&mut self, file: File) {
        self.stdout = Some(file);
    }

    fn stderr(&mut self, file: File) {
        self.stderr = Some(file);
    }
}

pub enum BuiltInCommand {
    Echo(Vec<String>),
    Cd(String),
    Pwd,
    Type(String),
    Exit(i32),
}

impl BuiltInCommand {
    pub fn try_from_string(command: String, arguments: Vec<String>) -> Result<BuiltInCommandWrapper, ()> {
        let command = BuiltInCommand::try_from(&command).map(|c| match c {
            BuiltInCommand::Echo(_) => BuiltInCommand::Echo(arguments),
            BuiltInCommand::Cd(_) => BuiltInCommand::Cd(arguments.first().unwrap().to_string()),
            BuiltInCommand::Type(_) => BuiltInCommand::Type(arguments.first().unwrap().to_string()),
            BuiltInCommand::Exit(_) => {
                BuiltInCommand::Exit(arguments.first().unwrap().parse().unwrap())
            }
            _ => c,
        });
        
        match command {
            Ok(command) => Ok(BuiltInCommandWrapper {
                command,
                stdout: None,
                stderr: None,
            }),
            Err(_) => Err(()),
        }
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

impl BuiltInCommand {
    fn execute(&self) -> Option<String> {
        match self {
            BuiltInCommand::Echo(messages) => format!("{}", messages.join(" ")).into(),
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
                        return format!("cd: {}: No such file or directory", new_dir).into();
                    }
                }
                None
            }
            BuiltInCommand::Pwd => format!("{}", std::env::current_dir().unwrap().display()).into(),
            BuiltInCommand::Type(command) => {
                if BuiltInCommand::try_from(command).is_ok() {
                    format!("{} is a shell builtin", command).into()
                } else {
                    match locate_in_path(command) {
                        Some(full_path) => format!("{} is {}", command, full_path).into(),
                        None => format!("{}: not found", command).into(),
                    }
                }
            }
            BuiltInCommand::Exit(code) => std::process::exit(*code),
        }
    }
}
