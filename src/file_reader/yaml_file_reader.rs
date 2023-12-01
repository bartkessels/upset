use std::fs::File;
use crate::configuration::ConfigFile;

pub struct YamlFileReader;

impl YamlFileReader {
    pub fn read_configuration(&self, file_path: &str) -> Option<ConfigFile> {
        let yaml_file = File::open(&file_path)
            .expect("Cannot op the specified file");
        let yaml: ConfigFile = serde_yaml::from_reader(yaml_file)
            .expect("Unable to read the contents of the specified file");

        return Some(yaml);
    }
}