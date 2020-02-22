extern crate reqwest;

use crate::service::core::Loader;
use async_trait::async_trait;

pub struct HttpLoader {
    pub url: String,
}

#[async_trait]
impl Loader for HttpLoader {
    async fn load(self) -> Result<String, ()> {
        println!("Start fetch from: {}", &self.url);
        for _i in 0..5 {
            let response = reqwest::get(&self.url).await;
            if let Some(r) = response.ok() {
                println!("Done fetch from: {}, {}", &self.url, r.status());

                if let Some(text) = r.text().await.ok() {
                    return Ok(text);
                }
            }
        }
        println!("Cannot fetch from: {}", &self.url);
        Err(())
    }
}
