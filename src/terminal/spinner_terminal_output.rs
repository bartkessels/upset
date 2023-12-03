use std::any::Any;
use std::sync::Arc;
use std::time::Duration;
use indicatif::ProgressBar;
use crate::terminal::terminal_output::TerminalOutput;

pub struct SpinnerTerminalOutput {
    spinner: ProgressBar
}

impl TerminalOutput for SpinnerTerminalOutput {
    fn loading(&self, message: &String) {
        self.spinner.enable_steady_tick(Duration::from_millis(120));
        self.spinner.set_message(message.to_owned());
    }

    fn finish_with_success(&self, message: &String) {
        self.spinner.finish_with_message(format!("✓ {}", &message));
    }

    fn finish_with_warning(&self, message: &String) {
        self.spinner.finish_with_message(format!("⚠ {}", &message));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SpinnerTerminalOutput {
    pub fn new() -> Arc<dyn TerminalOutput> {
        return Arc::new(Self {
            spinner: ProgressBar::new_spinner()
        });
    }
}
