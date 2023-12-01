use std::any::Any;
use crate::command::Command;
use crate::package_manager::{InstallationStatus, PackageManager};
use crate::terminal_output::TerminalOutput;

pub struct WingetPackageManager {
    /// The winget command
    winget_command: Box<dyn Command>,

    /// The source where the WinGet packages are retrieved from
    /// Possible options are
    ///     - msstore
    ///     - winget
    source: String
}

impl PackageManager for WingetPackageManager {
    fn install(&self, applications: &Vec<String>) -> Result<InstallationStatus, &str> {
        for application in applications {
            self.install_application(&application);
        }

        return Ok(InstallationStatus::Installed);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WingetPackageManager {
    pub fn new(winget_command: Box<dyn Command>, source: &str) -> Box<WingetPackageManager> {
        return Box::new(Self {
            winget_command,
            source: source.to_string()
        });
    }

    fn install_application(&self, application: &String) {
        let terminal_output = TerminalOutput::new();
        terminal_output.loading(&format!("Installing {}", &application));

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
            terminal_output.finish_with_success(&format!("Successfully installed {}", &application));
        } else {
            terminal_output.finish_with_failure(&format!("Unable to install {}", &application));
        }
    }
}

// calls the winget command with the expected parameters
#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use crate::command::MockCommand;
    use crate::package_manager::PackageManager;
    use crate::package_manager::winget_package_manager::WingetPackageManager;

    #[test]
    fn install_calls_the_winget_command_for_each_application() {
        // Arrange
        let applications = vec!("upset".to_string(), "ItDepends".to_string());
        let source = "msstore";
        let mut command_mock = MockCommand::new();

        // Act
        let sut = WingetPackageManager::new(Box::new(command_mock), &source);
        let result = sut.install(&applications);

        // Assert
        command_mock.expect_execute()
            .times(applications.len())
            .return_once(move |_| Ok(true));
    }
}