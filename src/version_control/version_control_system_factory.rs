use std::sync::Arc;
use crate::commands::GitCommand;
use crate::terminal::TerminalOutput;
use crate::version_control::git_version_control_system::GitVersionControlSystem;
use crate::version_control::version_control_system::VersionControlSystem;

pub struct VersionControlSystemFactory {
    terminal_output: Arc<dyn TerminalOutput>
}

impl VersionControlSystemFactory {
    pub fn new(terminal_output: &Arc<dyn TerminalOutput>) -> Arc<Self> {
        return Arc::new(Self {
            terminal_output: terminal_output.clone()
        });
    }

    pub fn get_version_control_system(
        &self,
        name: &str,
        destination_folder: &str
    ) -> Option<Arc<dyn VersionControlSystem>> {
        return match name.to_lowercase().as_str() {
            "git" => Some(GitVersionControlSystem::new(&GitCommand::new(), &destination_folder, &self.terminal_output)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::terminal::{MockTerminalOutput, TerminalOutput};
    use crate::version_control::git_version_control_system::GitVersionControlSystem;
    use crate::version_control::VersionControlSystemFactory;

    #[test]
    fn get_version_control_system_returns_none_for_empty_string() {
        // Arrange
        let name = "";
        let destination_folder = ".";
        let terminal_output = Arc::new(MockTerminalOutput::default());

        // Act
        let sut = VersionControlSystemFactory::new(
            &(terminal_output as Arc<dyn TerminalOutput>)
        );
        let result = sut.get_version_control_system(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_version_control_system_returns_none_for_unsupported_version_control_system() {
        // Arrange
        let name = "unsupported VCS";
        let destination_folder = ".";
        let terminal_output = Arc::new(MockTerminalOutput::default());

        // Act
        let sut = VersionControlSystemFactory::new(
            &(terminal_output as Arc<dyn TerminalOutput>)
        );
        let result = sut.get_version_control_system(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_version_control_system_returns_git_for_git_version_control_system() {
        // Arrange
        let name = "git";
        let destination_folder = ".";
        let terminal_output = Arc::new(MockTerminalOutput::default());

        // Act
        let sut = VersionControlSystemFactory::new(
            &(terminal_output as Arc<dyn TerminalOutput>)
        );
        let result = sut.get_version_control_system(name, destination_folder);

        // Assert
        assert!(result.is_some());
        assert!(result.unwrap().as_any().is::<GitVersionControlSystem>());
    }
}