use std::sync::Arc;
use crate::commands::WingetCommand;
use crate::package_manager::package_manager::PackageManager;
use crate::package_manager::winget_package_manager::WingetPackageManager;
use crate::terminal::TerminalOutput;

pub trait PackageManagerFactory {
    fn get_package_manager(&self, name: &String, source: &String) -> Option<Arc<dyn PackageManager>>;
}

pub struct PackageManagerFactoryImpl {
    terminal_output: Arc<dyn TerminalOutput>
}

impl PackageManagerFactory for PackageManagerFactoryImpl {
    fn get_package_manager(&self, name: &String, source: &String) -> Option<Arc<dyn PackageManager>> {
        return match name.to_lowercase().as_str() {
            "winget" => Some(
                WingetPackageManager::new(&WingetCommand::new(), &source, &self.terminal_output)
            ),
            _ => None
        };
    }
}

impl PackageManagerFactoryImpl {
    pub fn new(terminal_output: &Arc<dyn TerminalOutput>) -> Arc<dyn PackageManagerFactory> {
        return Arc::new(Self {
            terminal_output: terminal_output.clone()
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::package_manager::package_manager_factory::PackageManagerFactoryImpl;
    use crate::package_manager::winget_package_manager::WingetPackageManager;
    use crate::terminal::{MockTerminalOutput, TerminalOutput};

    #[test]
    fn get_package_manager_returns_none_for_empty_string() {
        // Arrange
        let name = String::default();
        let source = "source".to_string();
        let terminal_output = MockTerminalOutput::new();

        // Act
        let sut = PackageManagerFactoryImpl::new(&(Arc::new(terminal_output) as Arc<dyn TerminalOutput>));
        let result = sut.get_package_manager(&name, &source);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_package_manager_returns_none_for_unsupported_package_manager() {
        // Arrange
        let name = "not-supported-package-manager".to_string();
        let source = "source".to_string();
        let terminal_output = MockTerminalOutput::new();

        // Act
        let sut = PackageManagerFactoryImpl::new(&(Arc::new(terminal_output) as Arc<dyn TerminalOutput>));
        let result = sut.get_package_manager(&name, &source);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_package_manager_returns_winget_for_winget_package_manager() {
        // Arrange
        let name = "winget".to_string();
        let source = "source".to_string();
        let terminal_output = MockTerminalOutput::new();

        // Act
        let sut = PackageManagerFactoryImpl::new(&(Arc::new(terminal_output) as Arc<dyn TerminalOutput>));
        let result = sut.get_package_manager(&name, &source);

        // Assert
        assert!(result.is_some());
        assert!(result.unwrap().as_any().is::<WingetPackageManager>());
    }
}