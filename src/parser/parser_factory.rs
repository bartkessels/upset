use std::sync::Arc;
use crate::configuration::ConfigFile;
use crate::file_download::FileDownloadFactory;
use crate::package_manager::PackageManagerFactory;
use crate::parser::parser::Parser;
use crate::parser::version_100_parser::Version100Parser;
use crate::version_control::VersionControlSystemFactory;

pub struct ParserFactory {
    package_manager_factory: Arc<PackageManagerFactory>,
    version_control_system_factory: Arc<VersionControlSystemFactory>,
    file_download_factory: Arc<FileDownloadFactory>
}

impl ParserFactory {
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