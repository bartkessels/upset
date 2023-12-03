use std::any::Any;

pub trait VersionControlSystem {
    /// Download repositories to the local computer
    fn download(&self, repositories: &Vec<String>);

    fn as_any(&self) -> &dyn Any;
}