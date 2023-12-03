use std::any::Any;

pub trait FileDownload {
    fn download(&self, remote_source: &Vec<String>);
    fn as_any(&self) -> &dyn Any;
}