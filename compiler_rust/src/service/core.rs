pub trait Loader {
    fn load(&self) -> Result<String, ()>;
}

pub trait Parser {
    fn parse(&self, content: &str) -> Vec<String>;
}
