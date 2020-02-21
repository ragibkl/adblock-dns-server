use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceConfig {
    pub kind: String,
    pub format: String,
    pub path: String,
}

pub struct AppConfig {
    blacklist_src_path: String,
    whitelist_src_path: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            blacklist_src_path: "data/blacklist-src-urls.json".to_string(),
            whitelist_src_path: "data/whitelist-src-urls.json".to_string(),
        }
    }

    pub fn get_blacklist_srcs(&self) -> Vec<SourceConfig> {
        let file_content: String = fs::read_to_string(&self.blacklist_src_path)
            .unwrap()
            .parse()
            .unwrap();
        serde_json::from_str(&file_content).unwrap()
    }

    pub fn get_whitelist_srcs(&self) -> Vec<SourceConfig> {
        let file_content: String = fs::read_to_string(&self.whitelist_src_path)
            .unwrap()
            .parse()
            .unwrap();
        serde_json::from_str(&file_content).unwrap()
    }
}
