use std::any::Any;
use std::sync::Arc;
use crate::configuration::{Configuration, DownloadConfiguration, PackageConfiguration, VersionControlConfiguration};
use crate::file_download::FileDownloadFactory;
use crate::package_manager::PackageManagerFactory;
use crate::parser::parser::Parser;
use crate::version_control::VersionControlSystemFactory;

/// Parser for version 1.0.0 of the specification
pub struct Version100Parser {
    package_manager_factory: Arc<dyn PackageManagerFactory>,
    version_control_system_factory: Arc<dyn VersionControlSystemFactory>,
    file_download_factory: Arc<dyn FileDownloadFactory>
}

impl Parser for Version100Parser {
    fn parse(&self, configuration: &Configuration) {
        if let Some(packages) = &configuration.packages {
            self.parse_packages(packages);
        }

        if let Some(version_control_systems) = &configuration.version_control {
            self.parse_version_control(version_control_systems);
        }

        if let Some(downloads) = &configuration.downloads {
            self.parse_download_files(downloads);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Version100Parser {
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

    fn parse_packages(&self, applications_configuration: &Vec<PackageConfiguration>) {
        for application_config in applications_configuration {
            let package_manager = &self.package_manager_factory.get_package_manager(
                &application_config.package_manager,
                &application_config.source
            );

            if let Some(package_manager) = package_manager {
                package_manager.install(&application_config.applications);
            }
        }
    }

    fn parse_version_control(&self, version_control_configuration: &Vec<VersionControlConfiguration>) {
        for version_control_item_config in version_control_configuration {
            let version_control_system = &self.version_control_system_factory.get_version_control_system(
                &version_control_item_config.vcs,
                &version_control_item_config.destination_folder
            );

            if let Some(version_control_system) = version_control_system {
                version_control_system.download(&version_control_item_config.repositories);
            }
        }
    }

    fn parse_download_files(&self, remote_sources: &Vec<DownloadConfiguration>) {
        for remote_source in remote_sources {
            let download_manager = &self.file_download_factory.get_file_downloader(
                &remote_source.download_manager,
                &remote_source.destination_folder
            );

            if let Some(download_manager) = download_manager {
                download_manager.download(&remote_source.files);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::configuration::{Configuration, DownloadConfiguration, PackageConfiguration, VersionControlConfiguration};
    use crate::file_download::{FileDownload, FileDownloadFactory, MockFileDownload};
    use crate::package_manager::{MockPackageManager, PackageManager, PackageManagerFactory};
    use crate::parser::parser::Parser;
    use crate::parser::version_100_parser::Version100Parser;
    use crate::version_control::{MockVersionControlSystem, VersionControlSystem, VersionControlSystemFactory};

    struct MockPackageManagerFactory { package_manager: Arc<dyn PackageManager> }
    struct MockVersionControlSystemFactory { version_control: Arc<dyn VersionControlSystem> }
    struct MockFileDownloadFactory { file_download: Arc<dyn FileDownload> }

    #[test]
    fn parse_should_parse_the_packages_when_they_are_available() {
        // Arrange
        let mut mock_package_manager = MockPackageManager::new();
        let mock_version_control_system = MockVersionControlSystem::new();
        let mock_file_download = MockFileDownload::new();

        // Setup expectations
        mock_package_manager.expect_install()
            .once()
            .withf(|args| args.eq(&vec!("upset".to_string(), "ItDepends".to_string())))
            .returning(|_| {});

        let mock_package_manager_factory = MockPackageManagerFactory { package_manager: Arc::new(mock_package_manager) };
        let mock_version_control_system_factory = MockVersionControlSystemFactory { version_control: Arc::new(mock_version_control_system) };
        let mock_file_download_factory = MockFileDownloadFactory { file_download: Arc::new(mock_file_download) };

        let config = Configuration {
            packages: Some(vec!(
                PackageConfiguration {
                    package_manager: "winget".to_string(),
                    source: "msstore".to_string(),
                    applications: vec!("upset".to_string(), "ItDepends".to_string())
                }
            )),
            version_control: None,
            downloads: None
        };

        // Act
        let sut = Version100Parser::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        _ = sut.parse(&config);
    }

    #[test]
    fn parse_should_parse_the_version_control_systems_when_they_are_available() {
        // Arrange
        let mock_package_manager = MockPackageManager::new();
        let mut mock_version_control_system = MockVersionControlSystem::new();
        let mock_file_download = MockFileDownload::new();

        // Setup expectations
        mock_version_control_system.expect_download()
            .once()
            .withf(|args| args.eq(&vec!(
                "git@github.com:bartkessels/upset.git".to_string(),
                "git@github.com:bartkessels/it-depends.git".to_string()
            )))
            .returning(|_| {});

        let mock_package_manager_factory = MockPackageManagerFactory { package_manager: Arc::new(mock_package_manager) };
        let mock_version_control_system_factory = MockVersionControlSystemFactory { version_control: Arc::new(mock_version_control_system) };
        let mock_file_download_factory = MockFileDownloadFactory { file_download: Arc::new(mock_file_download) };

        let config = Configuration {
            packages: None,
            version_control: Some(vec!(
                VersionControlConfiguration {
                    vcs: "git".to_string(),
                    destination_folder: "~/Git-projects".to_string(),
                    repositories: vec!(
                        "git@github.com:bartkessels/upset.git".to_string(),
                        "git@github.com:bartkessels/it-depends.git".to_string()
                    ),
                }
            )),
            downloads: None
        };

        // Act
        let sut = Version100Parser::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        _ = sut.parse(&config);
    }

    #[test]
    fn parse_should_parse_the_file_downloads_when_they_are_available() {
        // Arrange
        let mock_package_manager = MockPackageManager::new();
        let mock_version_control_system = MockVersionControlSystem::new();
        let mut mock_file_download = MockFileDownload::new();

        // Setup expectations
        mock_file_download.expect_download()
            .once()
            .withf(|args| args.eq(&vec!(
                "https://bartkessels.net/download/upset".to_string(),
                "https://bartkessels.net/download/it-depends".to_string()
            )))
            .returning(|_| {});

        let mock_package_manager_factory = MockPackageManagerFactory { package_manager: Arc::new(mock_package_manager) };
        let mock_version_control_system_factory = MockVersionControlSystemFactory { version_control: Arc::new(mock_version_control_system) };
        let mock_file_download_factory = MockFileDownloadFactory { file_download: Arc::new(mock_file_download) };

        let config = Configuration {
            packages: None,
            version_control: None,
            downloads: Some(vec!(
                DownloadConfiguration {
                    download_manager: "wget".to_string(),
                    destination_folder: "~/Downloads".to_string(),
                    files: vec!(
                        "https://bartkessels.net/download/upset".to_string(),
                        "https://bartkessels.net/download/it-depends".to_string()
                    ),
                }
            ))
        };

        // Act
        let sut = Version100Parser::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        _ = sut.parse(&config);
    }

    #[test]
    fn should_not_parse_anything_when_the_configuration_is_empty() {
        // Arrange
        let mut mock_package_manager = MockPackageManager::new();
        let mut mock_version_control_system = MockVersionControlSystem::new();
        let mut mock_file_download = MockFileDownload::new();

        // Setup expectations
        mock_package_manager.expect_install()
            .never()
            .returning(|_| {});
        mock_version_control_system.expect_download()
            .never()
            .returning(|_| {});
        mock_file_download.expect_download()
            .never()
            .returning(|_| {});

        let mock_package_manager_factory = MockPackageManagerFactory { package_manager: Arc::new(mock_package_manager) };
        let mock_version_control_system_factory = MockVersionControlSystemFactory { version_control: Arc::new(mock_version_control_system) };
        let mock_file_download_factory = MockFileDownloadFactory { file_download: Arc::new(mock_file_download) };

        let config = Configuration {
            packages: None,
            version_control: None,
            downloads: None
        };

        // Act
        let sut = Version100Parser::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        _ = sut.parse(&config);
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

