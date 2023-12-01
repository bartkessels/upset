mod package_manager;
mod configuration;
mod file_reader;
mod version_control;
mod command;
mod commands;
mod arguments;
mod file_download;
mod terminal_output;

use clap::Parser;
use crate::file_reader::YamlFileReader;
use crate::package_manager::PackageManagerFactory;
use crate::version_control::VersionControlSystemFactory;

fn main() {
    let args = arguments::Arguments::parse();
    let config_file = YamlFileReader.read_configuration(args.configuration_file.as_str())
        .expect("Unable to read the config file");

    for package_config in config_file.configuration.packages {
        let package_manager = PackageManagerFactory::get_package_manager(
            &package_config.package_manager, &package_config.source
        ).expect("Unsupported package manager");

        package_manager.install(&package_config.applications)
            .expect("Unable to install applications");
    }

    for vcs_config in config_file.configuration.version_control {
        let version_control_system = VersionControlSystemFactory::get_version_control_system(
            &vcs_config.vcs,
            &vcs_config.destination_folder
        ).expect("Unsupported VCS");

        version_control_system.download(&vcs_config.repositories)
            .expect("Unable to download all repositories");
    }
}
