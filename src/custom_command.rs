use crate::command::Command;
use crate::locate_in_path;

pub struct CustomCommand {
    pub(crate) command: String,
    pub(crate) arguments: Vec<String>,
}

impl Command for CustomCommand {
    fn execute(&self) {
        match locate_in_path(&self.command) {
            Some(_) => self.execute_internal(),
            None => println!("{}: command not found", self.command),
        }
    }
}

impl CustomCommand {
    fn execute_internal(&self) {
        let mut command = std::process::Command::new(&self.command);
        command.args(self.arguments.iter());
        command.spawn().unwrap().wait().unwrap();
    }
}
