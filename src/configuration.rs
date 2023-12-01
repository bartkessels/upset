#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ConfigFile {
    pub version: f64,
    pub configuration: Configuration
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Configuration {
    pub packages: Vec<PackageConfiguration>,
    pub version_control: Vec<VersionControlConfiguration>,
    pub downloads: Vec<DownloadConfiguration>
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DownloadConfiguration {
    pub download_manager: String,
    pub destination_folder: String,
    pub files: Vec<String>
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct PackageConfiguration {
    pub package_manager: String,
    pub source: String,
    pub applications: Vec<String>
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct VersionControlConfiguration {
    pub vcs: String,
    pub destination_folder: String,
    pub repositories: Vec<String>
}