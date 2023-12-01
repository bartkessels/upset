use std::time::Duration;
use indicatif::ProgressBar;

pub struct TerminalOutput {
    spinner: ProgressBar
}

impl TerminalOutput {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            spinner: ProgressBar::new_spinner()
        });
    }

    pub fn loading(&self, message: &str) {
        self.spinner.enable_steady_tick(Duration::from_millis(120));
        self.spinner.set_message(message.to_owned());
    }

    pub fn finish_with_success(&self, message: &str) {
        self.spinner.finish_with_message(format!("✓ {}", &message));
    }

    pub fn finish_with_failure(&self, message: &str) {
        self.spinner.finish_with_message(format!("⚠ {}", &message));
    }
}