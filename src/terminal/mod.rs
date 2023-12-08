mod terminal_output;
mod terminal_output_factory;
mod spinner_terminal_output;

pub use terminal_output::TerminalOutput;
pub use terminal_output::MockTerminalOutput;
pub use terminal_output_factory::MockTerminalOutputFactory;
pub use terminal_output_factory::TerminalOutputFactory;
pub use terminal_output_factory::TerminalOutputFactoryImpl;
pub use terminal_output_factory::TerminalOutputType;