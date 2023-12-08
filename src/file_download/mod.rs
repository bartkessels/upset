mod file_download;
mod wget_file_download;
mod file_download_factory;

pub use file_download::MockFileDownload;
pub use file_download::FileDownload;
pub use file_download_factory::FileDownloadFactory;
pub use file_download_factory::FileDownloadFactoryImpl;