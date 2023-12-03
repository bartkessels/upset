mod package_manager;
mod configuration;
mod file_reader;
mod version_control;
mod command;
mod commands;
mod arguments;
mod file_download;
mod terminal;

use clap::Parser;
use crate::file_reader::YamlFileReader;
use crate::package_manager::PackageManagerFactory;
use crate::terminal::TerminalOutputFactory;
use crate::terminal::TerminalOutputType::Spinner;
use crate::version_control::VersionControlSystemFactory;

fn main() {
    let terminal_output_factory = TerminalOutputFactory::new();
    let terminal_output = terminal_output_factory.get_terminal_output(Spinner);

    let package_manager_factory = PackageManagerFactory::new(&terminal_output);
    let version_control_system_factory = VersionControlSystemFactory::new(&terminal_output);

    let args = arguments::Arguments::parse();
    let config_file = YamlFileReader.read_configuration(args.configuration_file.as_str())
        .expect("Unable to read the config file");

    for package_config in config_file.configuration.packages {
        let package_manager = package_manager_factory.get_package_manager(
            &package_config.package_manager, &package_config.source
        ).expect("Unsupported package manager");

        package_manager.install(&package_config.applications);
    }

    for vcs_config in config_file.configuration.version_control {
        let version_control_system = version_control_system_factory.get_version_control_system(
            &vcs_config.vcs,
            &vcs_config.destination_folder
        ).expect("Unsupported VCS");

        version_control_system.download(&vcs_config.repositories);
    }
}
