use std::any::Any;
use crate::package_manager::InstallationStatus;

/// Public methods to call on a specific package manager
pub trait PackageManager {
    /// Install an application
    fn install(&self, applications: &Vec<String>) -> Result<InstallationStatus, &str>;

    /// Get object reference
    fn as_any(&self) -> &dyn Any;
}
