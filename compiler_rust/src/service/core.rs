use std::error::Error;

pub trait Loader {
    fn load(&self) -> Result<String, Box<dyn Error>>;
}

pub trait Parser {
    fn parse(&self, content: String) -> Vec<String>;
}

pub struct ExtractTask {
    loader: Box<dyn Loader>,
    parser: Box<dyn Parser>,
}

impl ExtractTask {
    pub fn load_and_parse(&self) -> Vec<String> {
        let content = self.loader.load().unwrap();
        let domains = self.parser.parse(content);
        domains
    }
}
