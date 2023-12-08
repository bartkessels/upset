use std::any::Any;
use mockall::automock;

/// Public methods to call on a specific download manager
#[automock]
pub trait FileDownload {
    fn download(&self, remote_source: &Vec<String>);
    fn as_any(&self) -> &dyn Any;
}