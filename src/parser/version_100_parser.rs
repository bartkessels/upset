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
    use crate::configuration::{Configuration, PackageConfiguration};
    use crate::file_download::{FileDownload, FileDownloadFactory, MockFileDownload, MockFileDownloadFactory};
    use crate::package_manager::{MockPackageManager, MockPackageManagerFactory, PackageManager, PackageManagerFactory};
    use crate::parser::parser::Parser;
    use crate::parser::version_100_parser::Version100Parser;
    use crate::version_control::{MockVersionControlSystem, MockVersionControlSystemFactory, VersionControlSystem, VersionControlSystemFactory};

    #[test]
    fn parse_should_parse_the_packages_when_they_are_available() {
        // Arrange
        let mut mock_package_manager_factory = MockPackageManagerFactory::new();
        let mut mock_version_control_system_factory = MockVersionControlSystemFactory::new();
        let mut mock_file_download_factory = MockFileDownloadFactory::new();
        let mut mock_package_manager = MockPackageManager::new();
        let mut mock_version_control_system = MockVersionControlSystem::new();
        let mut mock_file_download = MockFileDownload::new();

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

        // Setup expectations
        mock_package_manager.expect_install()
            .once()
            .returning(|_| {});

        mock_package_manager_factory.expect_get_package_manager()
            .returning(|t,a| Some(Arc::new(mock_package_manager) as Arc<dyn PackageManager>));
        mock_version_control_system_factory.expect_get_version_control_system()
            .returning(|_, _| Some(Arc::new(mock_version_control_system) as Arc<dyn VersionControlSystem>));
        mock_file_download_factory.expect_get_file_downloader()
            .returning(|_, _| Some(Arc::new(mock_file_download) as Arc<dyn FileDownload>));

        // Act
        let sut = Version100Parser::new(
            &(Arc::new(mock_package_manager_factory) as Arc<dyn PackageManagerFactory>),
            &(Arc::new(mock_version_control_system_factory) as Arc<dyn VersionControlSystemFactory>),
            &(Arc::new(mock_file_download_factory) as Arc<dyn FileDownloadFactory>)
        );
        _ = sut.parse(&config);
    }
}