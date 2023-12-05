mod package_manager;
mod configuration;
mod file_reader;
mod version_control;
mod command;
mod commands;
mod arguments;
mod file_download;
mod terminal;
mod parser;

use clap::Parser;
use crate::file_download::{FileDownloadFactory, FileDownloadFactoryImpl};
use crate::file_reader::YamlFileReader;
use crate::package_manager::{PackageManagerFactory, PackageManagerFactoryImpl};
use crate::parser::ParserFactory;
use crate::terminal::{TerminalOutputFactory, TerminalOutputFactoryImpl};
use crate::terminal::TerminalOutputType::Spinner;
use crate::version_control::{VersionControlSystemFactory, VersionControlSystemFactoryImpl};

fn main() {
    let yaml_file_reader = YamlFileReader::new();

    let terminal_output_factory = TerminalOutputFactoryImpl::new();
    let terminal_output = terminal_output_factory.get_terminal_output(Spinner);

    let package_manager_factory = PackageManagerFactoryImpl::new(&terminal_output);
    let version_control_system_factory = VersionControlSystemFactoryImpl::new(&terminal_output);
    let file_download_factory = FileDownloadFactoryImpl::new(&terminal_output);
    let parser_factory = ParserFactory::new(
        &package_manager_factory,
        &version_control_system_factory,
        &file_download_factory
    );

    let args = arguments::Arguments::parse();
    let configuration = yaml_file_reader.read_configuration(&args.configuration_file)
        .expect("Unable to read the configuration file");
    let parser = parser_factory.get_parser(&configuration)
        .expect("Unable to parser configuration file");
    parser.parse(&configuration.configuration);
}
