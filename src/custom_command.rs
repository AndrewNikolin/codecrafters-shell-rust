use std::io::Write;
use crate::command::Command;
use crate::locate_in_path;

pub struct CustomCommand {
    pub(crate) command: String,
    pub(crate) arguments: Vec<String>,
    stdout: Option<std::fs::File>,
    stderr: Option<std::fs::File>,
}

impl Command for CustomCommand {
    fn execute(&self) {
        match locate_in_path(&self.command) {
            Some(_) => self.execute_internal(),
            None => println!("{}: command not found", self.command),
        }
    }
    
    fn stdout(&mut self, file: std::fs::File) {
        self.stdout = Some(file);
    }
    
    fn stderr(&mut self, file: std::fs::File) {
        self.stderr = Some(file);
    }
}

impl CustomCommand {
    fn execute_internal(&self) {
        let mut command = std::process::Command::new(&self.command);
        command.args(self.arguments.iter());
        
        if let Some(file) = &self.stdout {
            command.stdout(file.try_clone().unwrap());
        }
        
        if let Some(file) = &self.stderr {
            command.stderr(file.try_clone().unwrap());
        }
        
        command.spawn().unwrap().wait().unwrap();

        let stdout = command.output().unwrap().stdout;
        if !stdout.is_empty() && stdout.last().unwrap() != &b'\n' {
            if let Some(file) = &self.stdout {
                use std::io::Write;
                let mut file = file.try_clone().unwrap();
                file.flush().expect("TODO: panic message");
                file.write_all(b"\n").unwrap();
            }
        }

        let stderr = command.output().unwrap().stderr;
        if !stderr.is_empty() && stderr.last().unwrap() != &b'\n' {
            if let Some(file) = &self.stderr {
                use std::io::Write;
                let mut file = file.try_clone().unwrap();
                file.flush().expect("TODO: panic message");
                file.write_all(b"\n").unwrap();
            }
        }
    }
    
    pub(crate) fn new(command: String, arguments: Vec<String>) -> Self {
        CustomCommand {
            command,
            arguments,
            stdout: None,
            stderr: None,
        }
    }
}
