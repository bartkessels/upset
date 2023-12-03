use std::any::Any;
use std::sync::Arc;
use crate::command::Command;
use crate::file_download::file_download::FileDownload;
use crate::terminal::TerminalOutput;

pub struct WgetFileDownload {
    /// The wget command
    wget_command: Arc<dyn Command>,

    /// The folder where the files are downloaded to
    destination_folder: String,

    terminal_output: Arc<dyn TerminalOutput>
}

impl FileDownload for WgetFileDownload {
    fn download(&self, remote_sources: &Vec<String>) {
        for remote_source in remote_sources {
            self.download_file(&remote_source);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WgetFileDownload {
    pub fn new(
        wget_command: &Arc<dyn Command>,
        destination_folder: &str,
        terminal_output: &Arc<dyn TerminalOutput>
    ) -> Arc<dyn FileDownload> {
        return Arc::new(Self {
            wget_command: wget_command.clone(),
            destination_folder: destination_folder.to_string(),
            terminal_output: terminal_output.clone()
        });
    }

    pub fn download_file(&self, remote_source: &String) {
        self.terminal_output.loading(&format!("Downloading {}", &remote_source));

        let result = self.wget_command
            .execute(&[
                remote_source.to_string(),
                "-O".to_string(),
                self.destination_folder.to_string()
            ]);

        if result.is_ok_and(|output| output) {
            self.terminal_output.finish_with_success(
                &format!("Successfully downloaded {}", &remote_source)
            );
        } else {
            self.terminal_output.finish_with_warning(
                &format!("Unable to download {}", &remote_source)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use mockall::Sequence;
    use crate::command::{Command, MockCommand};
    use crate::file_download::wget_file_download::WgetFileDownload;
    use crate::terminal::{MockTerminalOutput, TerminalOutput};

    #[test]
    fn download_calls_the_wget_command_for_each_file() {
        // Arrange
        let remote_sources = vec!("remote1".to_string(), "remote2".to_string());
        let destination_folder = "~/Downloads";
        let mut command_mock = MockCommand::new();
        let terminal_output_mock = setup_terminal_output_mock();

        // Setup the expectations
        command_mock.expect_execute()
            .times(remote_sources.len())
            .returning(|_| Ok(true));

        // Act
        let sut = WgetFileDownload::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&remote_sources);
    }

    #[test]
    fn install_calls_the_wget_command_with_the_expected_arguments() {
        // Arrange
        let remote_source = "https://github.com/bartkessels/upset/release.zip";
        let remote_sources = vec!(remote_source.to_string());
        let destination_folder = "~/Downloads";
        let mut command_mock = MockCommand::new();
        let terminal_output_mock = setup_terminal_output_mock();

        // Setup the expectation
        command_mock.expect_execute()
            .withf(|args| args == &[
                remote_source.to_string(),
                "-O".to_string(),
                destination_folder.to_string()
            ])
            .returning(|_| Ok(true));

        // Act
        let sut = WgetFileDownload::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&remote_sources);
    }

    #[test]
    fn download_calls_loading_on_the_the_terminal_output_before_the_command_is_being_executed() {
        // Arrange
        let remote_source = "https://github.com/bartkessels/upset/release.zip";
        let remote_sources = vec!(remote_source.to_string());
        let destination_folder = "~/Downloads";
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
        let sut = WgetFileDownload::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&remote_sources);
    }

    #[test]
    fn download_calls_the_finish_with_success_on_the_the_terminal_output_when_the_command_succeeded() {
        // Arrange
        let remote_source = "https://github.com/bartkessels/upset/release.zip";
        let remote_sources = vec!(remote_source.to_string());
        let destination_folder = "~/Downloads";
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
        let sut = WgetFileDownload::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&remote_sources);
    }

    #[test]
    fn download_calls_finish_with_warning_on_the_the_terminal_output_when_the_command_fails() {
        // Arrange
        let remote_source = "https://github.com/bartkessels/upset/release.zip";
        let remote_sources = vec!(remote_source.to_string());
        let destination_folder = "~/Downloads";
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
        let sut = WgetFileDownload::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&remote_sources);
    }

    #[test]
    fn download_calls_finish_with_warning_on_the_the_terminal_output_when_the_command_returns_an_error() {
        // Arrange
        let remote_source = "https://github.com/bartkessels/upset/release.zip";
        let remote_sources = vec!(remote_source.to_string());
        let destination_folder = "~/Downloads";
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
        let sut = WgetFileDownload::new(
            &(Arc::new(command_mock) as Arc<dyn Command>),
            &destination_folder,
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        _ = sut.download(&remote_sources);
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