use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceConfig {
    pub kind: String,
    pub format: String,
    pub path: String,
}

pub struct AppConfig {
    http_blacklist_path: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
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
