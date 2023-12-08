use std::fs::File;
use std::sync::Arc;
use crate::configuration::ConfigFile;

pub struct YamlFileReader;

impl YamlFileReader {
    pub fn new() -> Arc<Self> {
        return Arc::new(Self);
    }

    pub fn read_configuration(&self, file_path: &str) -> Result<ConfigFile, &str> {
        if let Ok(file) = File::open(&file_path) {
            let configuration: serde_yaml::Result<ConfigFile> = serde_yaml::from_reader(file);

            if let Ok(configuration) = configuration {
                return Ok(configuration);
            }

            return Err("Unable to parse the configuration file. Make sure it's structured correctly");
        }

        return Err("Unable to read the configuration file. Make sure the location is correct");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use crate::file_reader::YamlFileReader;

    #[test]
    fn read_returns_error_if_the_file_does_not_exists() {
        // Arrange
        let file_path = "./does-not-exist.yaml";

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn read_returns_error_if_the_file_contains_no_yaml() {
        // Arrange
        let file_path = write_test_file("{}", "invalid-yaml");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_err());

        // Teardown
        delete_test_file(&file_path);
    }

    #[test]
    fn read_returns_error_if_the_file_does_not_contain_the_version() {
        // Arrange
        let configuration = "configuration:";
        let file_path = write_test_file(&configuration, "no-version");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_err());

        // Teardown
        delete_test_file(&file_path);
    }

    #[test]
    fn read_returns_error_if_the_file_does_not_contain_configuration() {
        // Arrange
        let configuration = "version: 1.0";
        let file_path = write_test_file(&configuration, "no-config");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_err());

        // Teardown
        delete_test_file(&file_path);
    }

    #[test]
    fn read_configuration_returns_none_for_configuration_items_if_configuration_is_empty() {
        // Arrange
        let configuration = "version: 1.0\nconfiguration:";
        let file_path = write_test_file(&configuration, "empty-config");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(1.0, result.version);
            assert!(result.configuration.packages.is_none());
            assert!(result.configuration.version_control.is_none());
            assert!(result.configuration.downloads.is_none());
        }

        // Teardown
        delete_test_file(&file_path);
    }

    #[test]
    fn read_configuration_should_parse_the_packages() {
        let configuration = "\
version: 1.0
configuration:
  packages:
    - package_manager: winget
      source: msstore
      applications:
        - upset
        - git.git
";
        let file_path = write_test_file(&configuration, "packages");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert!(result.configuration.packages.is_some());

            if let Some(packages) = result.configuration.packages {
                assert_eq!("winget".to_string(), packages.first().unwrap().package_manager);
                assert_eq!("msstore".to_string(), packages.first().unwrap().source);
                assert_eq!(vec!(
                    "upset".to_string(),
                    "git.git".to_string()
                ), packages.first().unwrap().applications);
            }
        }

        // Teardown
        delete_test_file(&file_path);
    }

    #[test]
    fn read_configuration_should_parse_the_version_control() {
        // Arrange
        let configuration = "\
version: 1.0
configuration:
  version_control:
    - vcs: git
      destination_folder: ~/Git-projects
      repositories:
        - git@github.com:bartkessels/upset.git
        - git@github.com:bartkessels/it-depends.git
";
        let file_path = write_test_file(&configuration, "vcs");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert!(result.configuration.version_control.is_some());

            if let Some(vcs) = result.configuration.version_control {
                assert_eq!("git".to_string(), vcs.first().unwrap().vcs);
                assert_eq!("~/Git-projects".to_string(), vcs.first().unwrap().destination_folder);
                assert_eq!(vec!(
                    "git@github.com:bartkessels/upset.git".to_string(),
                    "git@github.com:bartkessels/it-depends.git".to_string()
                ), vcs.first().unwrap().repositories);
            }
        }

        // Teardown
        delete_test_file(&file_path);
    }

    #[test]
    fn read_configuration_should_parse_the_file_downloads() {
        // Arrange
        let configuration = "\
version: 1.0
configuration:
  downloads:
    - download_manager: wget
      destination_folder: ~/Downloads
      files:
        - https://bartkessels.net/download/upset
        - https://bartkessels.net/download/it-depends

";
        let file_path = write_test_file(&configuration, "downloads");

        // Act
        let sut = YamlFileReader::new();
        let result = sut.read_configuration(&file_path);

        // Assert
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert!(result.configuration.downloads.is_some());

            if let Some(downloads) = result.configuration.downloads {
                assert_eq!("wget".to_string(), downloads.first().unwrap().download_manager);
                assert_eq!("~/Downloads".to_string(), downloads.first().unwrap().destination_folder);
                assert_eq!(vec!(
                    "https://bartkessels.net/download/upset".to_string(),
                    "https://bartkessels.net/download/it-depends".to_string()
                ), downloads.first().unwrap().files);
            }
        }

        // Teardown
        delete_test_file(&file_path);
    }

    fn write_test_file(contents: &str, name: &str) -> String {
        let file_path = &format!("./{}.yml", &name);
        let mut file = File::create(file_path).expect("Unable to create test file");
        file.write(contents.as_bytes()).expect("Unable to write to test file");
        file.flush().expect("Unable to flush the test file");

        return file_path.clone();
    }

    fn delete_test_file(file_path: &str) {
        fs::remove_file(&file_path).expect("Unable to delete the test file");
    }
}