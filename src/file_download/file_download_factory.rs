use std::sync::Arc;
use mockall::automock;
use crate::commands::WgetCommand;
use crate::file_download::file_download::FileDownload;
use crate::file_download::wget_file_download::WgetFileDownload;
use crate::terminal::TerminalOutput;

#[automock]
pub trait FileDownloadFactory {
    fn get_file_downloader(
        &self,
        name: &str,
        destination_folder: &str
    ) -> Option<Arc<dyn FileDownload>>;
}

pub struct FileDownloadFactoryImpl {
    terminal_output: Arc<dyn TerminalOutput>
}

impl FileDownloadFactory for FileDownloadFactoryImpl {
    fn get_file_downloader(
        &self, name: &str,
        destination_folder: &str
    ) -> Option<Arc<dyn FileDownload>> {
        return match name.to_lowercase().as_str() {
            "wget" => Some(WgetFileDownload::new(
                &WgetCommand::new(),
                &destination_folder,
                &self.terminal_output
            )),
            _ => None
        };
    }
}

impl FileDownloadFactoryImpl {
    pub fn new(terminal_output: &Arc<dyn TerminalOutput>) -> Arc<dyn FileDownloadFactory> {
        return Arc::new(Self {
            terminal_output: terminal_output.clone()
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::file_download::file_download_factory::{FileDownloadFactory, FileDownloadFactoryImpl};
    use crate::file_download::wget_file_download::WgetFileDownload;
    use crate::terminal::{MockTerminalOutput, TerminalOutput};

    #[test]
    fn get_file_download_returns_none_for_empty_string() {
        // Arrange
        let name = "";
        let destination_folder = ".";
        let terminal_output_mock = MockTerminalOutput::new();

        // Act
        let sut = FileDownloadFactoryImpl::new(
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        let result = sut.get_file_downloader(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_file_download_returns_none_for_unsupported_file_downloader() {
        // Arrange
        let name = "unsupported download tool";
        let destination_folder = ".";
        let terminal_output_mock = MockTerminalOutput::new();

        // Act
        let sut = FileDownloadFactoryImpl::new(
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        let result = sut.get_file_downloader(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_file_download_returns_wget_for_wget_file_downloader() {
        // Arrange
        let name = "wget";
        let destination_folder = ".";
        let terminal_output_mock = MockTerminalOutput::new();

        // Act
        let sut = FileDownloadFactoryImpl::new(
            &(Arc::new(terminal_output_mock) as Arc<dyn TerminalOutput>)
        );
        let result = sut.get_file_downloader(name, destination_folder);

        // Assert
        assert!(result.is_some());
        assert!(result.unwrap().as_any().is::<WgetFileDownload>());
    }
}
