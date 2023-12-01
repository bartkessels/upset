use crate::commands::WgetCommand;
use crate::file_download::file_download::FileDownload;
use crate::file_download::wget_file_download::WgetFileDownload;

pub struct FileDownloadFactory;

impl FileDownloadFactory {
    pub fn get_file_downloader(name: &str, destination_folder: &str) -> Option<Box<dyn FileDownload>> {
        return match name.to_lowercase().as_str() {
            "wget" => Some(WgetFileDownload::new(Box::new(WgetCommand), &destination_folder)),
            _ => None
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::file_download::file_download_factory::FileDownloadFactory;
    use crate::file_download::wget_file_download::WgetFileDownload;

    #[test]
    fn get_file_download_returns_none_for_empty_string() {
        // Arrange
        let name = "";
        let destination_folder = ".";

        // Act
        let result = FileDownloadFactory::get_file_downloader(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_file_download_returns_none_for_unsupported_file_downloader() {
        // Arrange
        let name = "unsupported download tool";
        let destination_folder = ".";

        // Act
        let result = FileDownloadFactory::get_file_downloader(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_file_download_returns_wget_for_wget_file_downloader() {
        // Arrange
        let name = "wget";
        let destination_folder = ".";

        // Act
        let result = FileDownloadFactory::get_file_downloader(name, destination_folder);

        // Assert
        assert!(result.is_some());
        assert!(result.unwrap().as_any().is::<WgetFileDownload>());
    }
}
