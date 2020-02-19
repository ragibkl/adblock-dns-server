use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceConfig {
    kind: String,
    format: String,
    path: String,
}

pub struct AppConfig {
    http_blacklist_path: String,
}

impl AppConfig {
    pub fn new() -> Config {
        Config {
            http_blacklist_path: "data/blacklist-src-urls.json".to_string(),
        }
    }

    pub fn get_blacklist_urls(&self) -> Vec<SourceConfig> {
        let file_content: String = fs::read_to_string(&self.http_blacklist_path)
            .unwrap()
            .parse()
            .unwrap();
        serde_json::from_str(&file_content).unwrap()
    }
}
