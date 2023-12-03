use std::any::Any;
use std::sync::Arc;
use crate::command::Command;
use crate::terminal::TerminalOutput;
use crate::version_control::version_control_system::VersionControlSystem;

pub struct GitVersionControlSystem {
    /// The git command
    git_command: Arc<dyn Command>,

    /// The folder where all repositories are going to be cloned into
    _destination_folder: String,

    /// Terminal output where the clone status is written to
    terminal_output: Arc<dyn TerminalOutput>
}

impl VersionControlSystem for GitVersionControlSystem {
    fn download(&self, repositories: &Vec<String>) {
        for repository in repositories {
            self.clone_repository(&repository);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GitVersionControlSystem {
    pub fn new(
        git_command: &Arc<dyn Command>,
        destination_folder: &str,
        terminal_output: &Arc<dyn TerminalOutput>
    ) -> Arc<dyn VersionControlSystem> {
        return Arc::new(Self {
            git_command: git_command.clone(),
            _destination_folder: destination_folder.to_string(),
            terminal_output: terminal_output.clone()
        });
    }

    fn clone_repository(&self, repository: &String) {
        self.terminal_output.loading(&format!("Cloning {}", &repository));

        let result = self.git_command
            .execute(&[
                "clone".to_string(),
                repository.to_string()
            ]);
        let is_success = result.is_ok_and(|output| output);

        if is_success {
            self.terminal_output.finish_with_success(
                &format!("Successfully cloned {}", &repository)
            );
        } else {
            self.terminal_output.finish_with_warning(
                &format!("Unable to clone {}", &repository)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use mockall::Sequence;
    use crate::command::{Command, MockCommand};
    use crate::terminal::{MockTerminalOutput, TerminalOutput};
    use crate::version_control::git_version_control_system::GitVersionControlSystem;

    #[test]
    fn download_calls_the_git_command_for_each_repository() {
        // Arrange
        let repositories = vec!("repo1".to_string(), "repo2".to_string());
        let destination_folder = "~/Git-projects".to_string();
        let mut command_mock = MockCommand::new();
        let terminal_output_mock = setup_terminal_output_mock();

        // Setup the expectations
        command_mock.expect_execute()
            .times(repositories.len())
            .returning(|_| Ok(true));

        // Act
        let sut = GitVersionControlSystem::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&repositories);
    }

    #[test]
    fn download_calls_the_git_command_with_the_expected_arguments() {
        // Arrange
        let repository = "git@github.com:bartkessels/upset.git";
        let repositories = vec!(repository.to_string());
        let destination_folder = "~/Git-projects".to_string();
        let mut command_mock = MockCommand::new();
        let terminal_output_mock = setup_terminal_output_mock();

        // Setup the expectations
        command_mock.expect_execute()
            .withf(|args| args ==&[
                "clone".to_string(),
                repository.to_string()
            ])
            .returning(|_| Ok(true));

        // Act
        let sut = GitVersionControlSystem::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&repositories);
    }

    #[test]
    fn download_calls_loading_on_the_the_terminal_output_before_the_command_is_being_executed() {
        // Arrange
        let repositories = vec!("repository".to_string());
        let destination_folder = "~/Git-projects".to_string();
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
        let sut = GitVersionControlSystem::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&repositories);
    }

    #[test]
    fn download_calls_the_finish_with_success_on_the_the_terminal_output_when_the_command_succeeded() {
        // Arrange
        let repositories = vec!("repository".to_string());
        let destination_folder = "~/Git-projects".to_string();
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
        let sut = GitVersionControlSystem::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&repositories);
    }

    #[test]
    fn download_calls_the_finish_with_warning_on_the_the_terminal_output_when_the_command_fails() {
        // Arrange
        let repositories = vec!("repository".to_string());
        let destination_folder = "~/Git-projects".to_string();
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
        let sut = GitVersionControlSystem::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&repositories);
    }

    #[test]
    fn download_calls_the_finish_with_warning_on_the_the_terminal_output_when_the_command_returns_an_error() {
        // Arrange
        let repositories = vec!("repository".to_string());
        let destination_folder = "~/Git-projects".to_string();
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
        let sut = GitVersionControlSystem::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&repositories);
    }

    fn setup_terminal_output_mock() -> MockTerminalOutput {
        let mut terminal_output_mock = MockTerminalOutput::new();

        // Setup the expectations
        terminal_output_mock.expect_loading().returning(|_| {});
        terminal_output_mock.expect_finish_with_success().returning(|_| {});
        terminal_output_mock.expect_finish_with_warning().returning(|_| {});

        return terminal_output_mock;
    }
}