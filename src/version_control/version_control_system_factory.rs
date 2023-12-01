use crate::commands::GitCommand;
use crate::version_control::git_version_control_system::GitVersionControlSystem;
use crate::version_control::version_control_system::VersionControlSystem;

pub struct VersionControlSystemFactory;

impl VersionControlSystemFactory {
    pub fn get_version_control_system(name: &str, destination_folder: &str) -> Option<Box<dyn VersionControlSystem>> {
        return match name.to_lowercase().as_str() {
            "git" => Some(GitVersionControlSystem::new(Box::new(GitCommand), &destination_folder)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::version_control::git_version_control_system::GitVersionControlSystem;
    use crate::version_control::VersionControlSystemFactory;

    #[test]
    fn get_version_control_system_returns_none_for_empty_string() {
        // Arrange
        let name = "";
        let destination_folder = ".";

        // Act
        let result = VersionControlSystemFactory::get_version_control_system(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_version_control_system_returns_none_for_unsupported_version_control_system() {
        // Arrange
        let name = "unsupported VCS";
        let destination_folder = ".";

        // Act
        let result = VersionControlSystemFactory::get_version_control_system(name, destination_folder);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_version_control_system_returns_git_for_git_version_control_system() {
        // Arrange
        let name = "git";
        let destination_folder = ".";

        // Act
        let result = VersionControlSystemFactory::get_version_control_system(name, destination_folder);

        // Assert
        assert!(result.is_some());
        assert!(result.unwrap().as_any().is::<GitVersionControlSystem>());
    }
}