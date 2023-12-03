use std::any::Any;
use mockall::automock;

#[automock]
pub trait TerminalOutput {
    /// Send a message to the terminal with a loading indicator
    fn loading(&self, message: &String);

    /// Send a message to the terminal with a success indicator
    fn finish_with_success(&self, message: &String);

    /// Send a message to the terminal with a warning indicator
    fn finish_with_warning(&self, message: &String);

    fn as_any(&self) -> &dyn Any;
}