use std::any::Any;
use mockall::automock;

/// Public methods to call on a specific package manager
#[automock]
pub trait PackageManager {
    /// Install an application
    fn install(&self, applications: &Vec<String>);

    /// Get object reference
    fn as_any(&self) -> &dyn Any;
}