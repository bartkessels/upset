use std::any::Any;
use mockall::automock;

#[automock]
pub trait VersionControlSystem {
    /// Download repositories to the local computer
    fn download(&self, repositories: &Vec<String>);

    fn as_any(&self) -> &dyn Any;
}