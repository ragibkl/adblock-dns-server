use std::error::Error;

pub trait Loader {
    fn load(&self) -> Result<String, Box<dyn Error>>;
}

pub trait Parser {
    fn parse(&self, content: String) -> Vec<String>;
}
