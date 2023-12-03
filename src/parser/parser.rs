use crate::configuration::Configuration;

pub trait Parser {
    fn parse(&self, configuration: &Configuration);
}