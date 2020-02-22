use std::fs;

use crate::service::core::Loader;
use async_trait::async_trait;

pub struct FileLoader {
    pub path: String,
}

#[async_trait]
impl Loader for FileLoader {
    async fn load(&self) -> Result<String, ()> {
        let contents = fs::read_to_string(&self.path).map_err(|_e| ())?;
        println!("{}", contents);

        Ok(contents)
    }
}
