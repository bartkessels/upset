use std::process;
use std::sync::Arc;
use crate::command::Command;

pub struct WingetCommand;

const WINGET_COMMAND: &str = "winget";

impl Command for WingetCommand {
    fn execute(&self, arguments: &[String]) -> Result<bool, String> {
        if !self.command_exists(&WINGET_COMMAND) {
            return Err("Winget command can not be found!".to_string());
        }

        let command_output = process::Command::new(&WINGET_COMMAND)
            .args(arguments)
            .output();

        return Ok(command_output.is_ok_and(|output| output.status.success()));
    }
}

impl WingetCommand {
    pub fn new() -> Arc<dyn Command> {
        return Arc::new(Self);
    }
}