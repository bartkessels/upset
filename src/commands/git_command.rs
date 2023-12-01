use std::process;
use crate::command::Command;

pub struct GitCommand;

const GIT_COMMAND: &str = "git";

impl Command for GitCommand {
    fn execute(&self, arguments: &[String]) -> Result<bool, String> {
        if !self.command_exists(&GIT_COMMAND) {
            return Err("Git command can not be found!".to_string());
        }

        let command_output = process::Command::new(&GIT_COMMAND)
            .args(arguments)
            .output();

        return Ok(command_output.is_ok_and(|output| output.status.success()));
    }
}