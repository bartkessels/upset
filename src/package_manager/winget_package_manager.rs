use std::any::Any;
use std::sync::Arc;
use crate::command::Command;
use crate::package_manager::package_manager::PackageManager;
use crate::terminal::TerminalOutput;

pub struct WingetPackageManager {
    /// The winget command
    winget_command: Arc<dyn Command>,

    /// The source where the WinGet packages are retrieved from
    /// Possible options are
    ///     - msstore
    ///     - winget
    source: String,

    /// Terminal output where the installation status is written to
    terminal_output: Arc<dyn TerminalOutput>
}

impl PackageManager for WingetPackageManager {
    fn install(&self, applications: &Vec<String>) {
        for application in applications {
            self.install_application(&application);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WingetPackageManager {
    pub fn new(winget_command: &Arc<dyn Command>, source: &str, terminal_output: &Arc<dyn TerminalOutput>) -> Arc<dyn PackageManager> {
        return Arc::new(Self {
            winget_command: winget_command.clone(),
            source: source.to_string(),
            terminal_output: terminal_output.clone()
        });
    }

    fn install_application(&self, application: &String) {
        self.terminal_output.loading(&format!("Installing {}", &application));

        let result = self.winget_command
            .execute(&[
                "install".to_string(),
                application.to_string(),
                "-s".to_string(),
                self.source.to_string(),
                "--disable-interactivity".to_string()
            ]);
        let is_success = result.is_ok_and(|result| result);

        if is_success {
            self.terminal_output.finish_with_success(&format!("Successfully installed {}", &application));
        } else {
            self.terminal_output.finish_with_warning(&format!("Unable to install {}", &application));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use mockall::Sequence;
    use crate::command::{Command, MockCommand};
    use crate::package_manager::winget_package_manager::WingetPackageManager;
    use crate::terminal::{MockTerminalOutput, TerminalOutput};

    #[test]
    fn install_calls_the_winget_command_for_each_application() {
        // Arrange
        let applications = vec!("upset".to_string(), "ItDepends".to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();
        let terminal_output_mock = setup_terminal_output_mock();

        // Setup the expectation
        command_mock.expect_execute()
            .times(applications.len())
            .returning(|_| Ok(true));

        // Act
        let sut = WingetPackageManager::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &source,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.install(&applications);
    }

    #[test]
    fn install_calls_the_winget_command_with_the_expected_arguments() {
        // Arrange
        let application_name = "upset";
        let applications = vec!(application_name.to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();
        let terminal_output_mock = setup_terminal_output_mock();

        // Setup the expectation
        command_mock.expect_execute()
            .withf(|args| args == &[
                "install".to_string(),
                application_name.to_string(),
                "-s".to_string(),
                source.to_string(),
                "--disable-interactivity".to_string()
            ])
            .returning(|_| Ok(true));

        // Act
        let sut = WingetPackageManager::new(&(Arc::new(command_mock) as Arc<dyn Command>), &source, &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>));
        _ = sut.install(&applications);
    }

    #[test]
    fn install_calls_loading_on_the_the_terminal_output_before_the_command_is_being_executed() {
        // Arrange
        let applications = vec!("upset".to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();
        let mut terminal_output_mock = MockTerminalOutput::new();

        // Setup the mocks
        let mut sequence = Sequence::new();

        // Setup the expectation
        terminal_output_mock.expect_loading()
            .once()
            .in_sequence(&mut sequence)
            .returning(|_| {});
        terminal_output_mock.expect_finish_with_success().returning(|_| {});

        command_mock.expect_execute()
            .once()
            .in_sequence(&mut sequence)
            .returning(|_| Ok(true));

        // Act
        let sut = WingetPackageManager::new(&(Arc::new(command_mock) as Arc<dyn Command>), &source, &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>));
        _ = sut.install(&applications);
    }

    #[test]
    fn install_calls_the_finish_with_success_on_the_the_terminal_output_when_the_command_succeeded() {
        // Arrange
        let applications = vec!("upset".to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();
        let mut terminal_output_mock = MockTerminalOutput::new();

        // Setup the mocks
        command_mock.expect_execute().returning(|_| Ok(true));

        // Setup the expectation
        terminal_output_mock.expect_loading().returning(|_| {});
        terminal_output_mock.expect_finish_with_success()
            .once()
            .returning(|_| {});

        // Act
        let sut = WingetPackageManager::new(&(Arc::new(command_mock) as Arc<dyn Command>), &source, &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>));
        _ = sut.install(&applications);
    }

    #[test]
    fn install_calls_finish_with_warning_on_the_the_terminal_output_when_the_command_fails() {
        // Arrange
        let applications = vec!("upset".to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();
        let mut terminal_output_mock = MockTerminalOutput::new();

        // Setup the mocks
        command_mock.expect_execute().returning(|_| Ok(false));

        // Setup the expectation
        terminal_output_mock.expect_loading().returning(|_| {});
        terminal_output_mock.expect_finish_with_warning()
            .once()
            .returning(|_| {});

        // Act
        let sut = WingetPackageManager::new(&(Arc::new(command_mock) as Arc<dyn Command>), &source, &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>));
        _ = sut.install(&applications);
    }

    #[test]
    fn install_calls_finish_with_warning_on_the_the_terminal_output_when_the_command_returns_an_error() {
        // Arrange
        let applications = vec!("upset".to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();
        let mut terminal_output_mock = MockTerminalOutput::new();

        // Setup the mocks
        command_mock.expect_execute().returning(|_| Err(String::default()));

        // Setup the expectation
        terminal_output_mock.expect_loading().returning(|_| {});
        terminal_output_mock.expect_finish_with_warning()
            .once()
            .returning(|_| {});

        // Act
        let sut = WingetPackageManager::new(&(Arc::new(command_mock) as Arc<dyn Command>), &source, &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>));
        _ = sut.install(&applications);
    }

    fn setup_terminal_output_mock() -> MockTerminalOutput {
        let mut terminal_output_mock = MockTerminalOutput::new();

        // Setup the expectation
        terminal_output_mock.expect_loading().returning(|_| {});
        terminal_output_mock.expect_finish_with_success().returning(|_| {});
        terminal_output_mock.expect_finish_with_warning().returning(|_| {});

        return terminal_output_mock;
    }
}