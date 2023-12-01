use crate::commands::WingetCommand;
use crate::package_manager::PackageManager;
use crate::package_manager::winget_package_manager::WingetPackageManager;

pub struct PackageManagerFactory;

impl PackageManagerFactory {
    pub fn get_package_manager(name: &str, source: &str) -> Option<Box<dyn PackageManager>> {
        return match name.to_lowercase().as_str() {
            "winget" => Some(WingetPackageManager::new(Box::new(WingetCommand), &source)),
            _ => None
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::package_manager::PackageManagerFactory;
    use crate::package_manager::winget_package_manager::WingetPackageManager;

    #[test]
    fn get_package_manager_returns_none_for_empty_string() {
        // Arrange
        let name = "";
        let source = "source";

        // Act
        let result = PackageManagerFactory::get_package_manager(name, source);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_package_manager_returns_none_for_unsupported_package_manager() {
        // Arrange
        let name = "not-supported-package-manager";
        let source = "source";

        // Act
        let result = PackageManagerFactory::get_package_manager(name, source);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_package_manager_returns_winget_for_winget_package_manager() {
        // Arrange
        let name = "winget";
        let source = "source";

        // Act
        let result = PackageManagerFactory::get_package_manager(name, source);

        // Assert
        assert!(result.is_some());
        assert!(result.unwrap().as_any().is::<WingetPackageManager>());
    }
}