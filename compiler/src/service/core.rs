pub trait Parser: Send + Sync {
    fn parse(&self, content: &str) -> Vec<String>;
}
