use std::any::Any;

/// Public methods to call on a specific download manager
pub trait FileDownload {
    fn download(&self, remote_source: &Vec<String>);
    fn as_any(&self) -> &dyn Any;
}