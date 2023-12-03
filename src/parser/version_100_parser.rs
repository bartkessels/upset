use std::sync::Arc;
use crate::configuration::{Configuration, DownloadConfiguration, PackageConfiguration, VersionControlConfiguration};
use crate::file_download::FileDownloadFactory;
use crate::package_manager::PackageManagerFactory;
use crate::parser::parser::Parser;
use crate::version_control::VersionControlSystemFactory;

/// Parser for version 1.0.0 of the specification
pub struct Version100Parser {
    package_manager_factory: Arc<PackageManagerFactory>,
    version_control_system_factory: Arc<VersionControlSystemFactory>,
    file_download_factory: Arc<FileDownloadFactory>
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
        package_manager_factory: &Arc<PackageManagerFactory>,
        version_control_system_factory: &Arc<VersionControlSystemFactory>,
        file_download_factory: &Arc<FileDownloadFactory>
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