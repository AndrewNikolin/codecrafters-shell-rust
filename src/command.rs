use std::fs::File;

pub trait Command {
    fn execute(&self);
    fn stdout(&mut self, file: File);
    fn stderr(&mut self, file: File);
}