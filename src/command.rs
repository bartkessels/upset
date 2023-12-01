use std::io::ErrorKind::NotFound;
use std::process;
use mockall::automock;

/// Wrapper object around a command
/// This is useful for decoupling specific
/// applications from the logic of the business logic
#[automock]
pub trait Command {
    /// Execute the command with arguments
    ///
    /// Returns an error when the command does not exist
    /// otherwise returns whether the command has executed
    /// successfully or not
    fn execute(&self, arguments: &[String]) -> Result<bool, String>;

    /// Check if a specific command exists
    fn command_exists(&self, name: &str) -> bool {
        return match process::Command::new(&name).output() {
            Ok(_) => true,
            Err(e) => e.kind() != NotFound
        };
    }
}
