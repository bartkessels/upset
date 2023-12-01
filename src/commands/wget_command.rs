use std::process;
use crate::command::Command;

pub struct WgetCommand;

const WGET_COMMAND: &str = "wget";

impl Command for WgetCommand {
    fn execute(&self, arguments: &[String]) -> Result<bool, String> {
        if !self.command_exists(&WGET_COMMAND) {
            return Err("Wget command can not be found!".to_string());
        }

        let command_output = process::Command::new(&WGET_COMMAND)
            .args(arguments)
            .output();

        return Ok(command_output.is_ok_and(|output| output.status.success()));
    }
}