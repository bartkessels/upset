use std::any::Any;
use crate::configuration::Configuration;

pub trait Parser {
    fn parse(&self, configuration: &Configuration);
    fn as_any(&self) -> &dyn Any;
}