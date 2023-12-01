use std::any::Any;

pub trait FileDownload {
    fn download(&self, remote_source: &Vec<String>) -> Result<bool, &str>;
    fn as_any(&self) -> &dyn Any;
}