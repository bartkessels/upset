use std::sync::Arc;
use crate::terminal::spinner_terminal_output::SpinnerTerminalOutput;
use crate::terminal::terminal_output::TerminalOutput;

pub struct TerminalOutputFactory;

pub enum TerminalOutputType {
    Spinner
}

impl TerminalOutputFactory {
    pub fn new() -> Arc<Self> {
        return Arc::new(Self);
    }

    pub fn get_terminal_output(&self, terminal_output_type: TerminalOutputType) -> Arc<dyn TerminalOutput> {
        return match terminal_output_type {
            TerminalOutputType::Spinner => SpinnerTerminalOutput::new()
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::terminal::spinner_terminal_output::SpinnerTerminalOutput;
    use crate::terminal::terminal_output_factory::{TerminalOutputFactory, TerminalOutputType};

    #[test]
    fn get_terminal_output_should_return_spinner_terminal_output_for_spinner_terminal_output_type() {
        // Arrange
        let terminal_output_type = TerminalOutputType::Spinner;

        // Act
        let sut = TerminalOutputFactory::new();
        let result = sut.get_terminal_output(terminal_output_type);

        // Assert
        assert!(result.as_any().is::<SpinnerTerminalOutput>());
    }
}