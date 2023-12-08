use std::sync::Arc;
use crate::configuration::ConfigFile;
use crate::file_download::FileDownloadFactory;
use crate::package_manager::PackageManagerFactory;
use crate::parser::parser::Parser;
use crate::parser::version_100_parser::Version100Parser;
use crate::version_control::VersionControlSystemFactory;

pub struct ParserFactory {
    package_manager_factory: Arc<dyn PackageManagerFactory>,
    version_control_system_factory: Arc<dyn VersionControlSystemFactory>,
    file_download_factory: Arc<dyn FileDownloadFactory>
}

impl ParserFactory {
    pub fn new(
        package_manager_factory: &Arc<dyn PackageManagerFactory>,
        version_control_system_factory: &Arc<dyn VersionControlSystemFactory>,
        file_download_factory: &Arc<dyn FileDownloadFactory>
    ) -> Arc<Self> {
        return Arc::new(Self {
            package_manager_factory: package_manager_factory.clone(),
            version_control_system_factory: version_control_system_factory.clone(),
            file_download_factory: file_download_factory.clone()
        });
    }

    pub fn get_parser(&self, config_file: &ConfigFile) -> Result<Arc<dyn Parser>, &str> {
        if config_file.version == 1.0 {
            return Ok(Version100Parser::new(
                &self.package_manager_factory,
                &self.version_control_system_factory,
                &self.file_download_factory
            ));
        }

        return Err("Unsupported specification version")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::configuration::{ConfigFile, Configuration};
    use crate::file_download::{FileDownload, FileDownloadFactory, MockFileDownload};
    use crate::package_manager::{MockPackageManager, PackageManager, PackageManagerFactory};
    use crate::parser::ParserFactory;
    use crate::parser::version_100_parser::Version100Parser;
    use crate::version_control::{MockVersionControlSystem, VersionControlSystem, VersionControlSystemFactory};

    struct MockPackageManagerFactory { package_manager: Arc<dyn PackageManager> }
    struct MockVersionControlSystemFactory { version_control: Arc<dyn VersionControlSystem> }
    struct MockFileDownloadFactory { file_download: Arc<dyn FileDownload> }

    #[test]
    fn get_parser_should_return_version100_for_version_100() {
        // Arrange
        let config_file = ConfigFile {
            version: 1.0,
            configuration: Configuration {
                packages: None,
                version_control: None,
                downloads: None,
            }
        };

        let mock_package_manager = MockPackageManager::new();
        let mock_version_control_system = MockVersionControlSystem::new();
        let mock_file_download = MockFileDownload::new();

        let mock_package_manager_factory = MockPackageManagerFactory { package_manager: Arc::new(mock_package_manager) };
        let mock_version_control_system_factory = MockVersionControlSystemFactory { version_control: Arc::new(mock_version_control_system) };
        let mock_file_download_factory = MockFileDownloadFactory { file_download: Arc::new(mock_file_download) };

        // Assert
        let sut = ParserFactory::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        let result = sut.get_parser(&config_file);

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().as_any().is::<Version100Parser>());
    }

    #[test]
    fn get_parser_should_return_err_for_unsupported_version() {
        // Arrange
        let config_file = ConfigFile {
            version: -1.0,
            configuration: Configuration {
                packages: None,
                version_control: None,
                downloads: None,
            }
        };

        let mock_package_manager = MockPackageManager::new();
        let mock_version_control_system = MockVersionControlSystem::new();
        let mock_file_download = MockFileDownload::new();

        let mock_package_manager_factory = MockPackageManagerFactory { package_manager: Arc::new(mock_package_manager) };
        let mock_version_control_system_factory = MockVersionControlSystemFactory { version_control: Arc::new(mock_version_control_system) };
        let mock_file_download_factory = MockFileDownloadFactory { file_download: Arc::new(mock_file_download) };

        // Assert
        let sut = ParserFactory::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        let result = sut.get_parser(&config_file);

        // Assert
        assert!(result.is_err());
    }

    impl PackageManagerFactory for MockPackageManagerFactory {
        fn get_package_manager(&self, _: &String, _: &String) -> Option<Arc<dyn PackageManager>> {
            Some(self.package_manager.clone())
        }
    }

    impl VersionControlSystemFactory for MockVersionControlSystemFactory {
        fn get_version_control_system(&self, _: &str, _: &str) -> Option<Arc<dyn VersionControlSystem>> {
            Some(self.version_control.clone())
        }
    }

    impl FileDownloadFactory for MockFileDownloadFactory {
        fn get_file_downloader(&self, _: &str, _: &str) -> Option<Arc<dyn FileDownload>> {
            Some(self.file_download.clone())
        }
    }
}