use async_trait::async_trait;

#[async_trait]
pub trait Loader: Send + Sync {
    async fn load(&self) -> Result<String, ()>;
}

pub trait Parser: Send + Sync {
    fn parse(&self, content: &str) -> Vec<String>;
}
