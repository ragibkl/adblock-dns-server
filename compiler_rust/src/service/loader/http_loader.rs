extern crate reqwest;

use crate::service::core::Loader;
use async_trait::async_trait;

pub struct HttpLoader {
    pub url: String,
}

#[async_trait]
impl Loader for HttpLoader {
    async fn load(&self) -> Result<String, ()> {
        for i in 0..3 {
            println!("Start fetch from: {}", &self.url);
            let response = reqwest::get(&self.url).await;
            if let Some(r) = response.ok() {
                if let Some(text) = r.text().await.ok() {
                    println!("Done fetch from: {}", &self.url);
                    return Ok(text);
                }
                println!("Retry fetch from: {}, retries: {}", &self.url, i + 1);
            }
        }

        println!("Cannot fetch from: {}", &self.url);
        Err(())
    }
}
