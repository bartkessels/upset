use std::any::Any;
use crate::command::Command;
use crate::file_download::file_download::FileDownload;

pub struct WgetFileDownload {
    /// The wget command
    wget_command: Box<dyn Command>,

    /// The folder where the files are downloaded to
    destination_folder: String
}

impl FileDownload for WgetFileDownload {
    fn download(&self, remote_sources: &Vec<String>) -> Result<bool, &str> {
        let mut is_failure = false;

        for remote_source in remote_sources {
            is_failure &= self.download_file(&remote_source);
        }

        return Ok(is_failure);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WgetFileDownload {
    pub fn new(wget_command: Box<dyn Command>, destination_folder: &str) -> Box<Self> {
        return Box::new(Self {
            wget_command,
            destination_folder: destination_folder.to_string()
        });
    }

    pub fn download_file(&self, remote_source: &String) -> bool {
        let result = self.wget_command
            .execute(&[
                remote_source.to_string(),
                "-O".to_string(),
                self.destination_folder.to_string()
            ]);
        return result.is_ok_and(|output| output);
    }
}