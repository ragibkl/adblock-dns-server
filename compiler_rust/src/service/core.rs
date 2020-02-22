use async_trait::async_trait;

#[async_trait]
pub trait Loader: Send {
    async fn load(self) -> Result<String, ()>;
}

pub trait Parser: Send {
    fn parse(&self, content: &str) -> Vec<String>;
}
