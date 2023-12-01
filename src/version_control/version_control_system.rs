use std::any::Any;
use crate::version_control::download_status::DownloadStatus;

pub trait VersionControlSystem {
    /// Download repositories to the local computer
    fn download(&self, repositories: &Vec<String>) -> Result<DownloadStatus, &str>;

    fn as_any(&self) -> &dyn Any;
}