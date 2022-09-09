use std::{fmt::Debug, fs};

use config::{AsyncSource, ConfigError, FileFormat, Format, Map};

use async_trait::async_trait;

#[derive(Debug)]
pub enum ConfigSource {
    Http { uri: String },
    File { path: String },
}

impl ConfigSource {
    pub fn new(path: &str) -> Self {
        if path.starts_with("http") {
            Self::Http {
                uri: path.to_string(),
            }
        } else {
            Self::File {
                path: path.to_string(),
            }
        }
    }
}

#[async_trait]
impl AsyncSource for ConfigSource {
    async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
        let format = FileFormat::Yaml;

        match &self {
            Self::Http { uri } => {
                reqwest::get(uri)
                    .await
                    .map_err(|e| ConfigError::Foreign(Box::new(e)))? // error conversion is possible from custom AsyncSource impls
                    .text()
                    .await
                    .map_err(|e| ConfigError::Foreign(Box::new(e)))
                    .and_then(|text| {
                        format
                            .parse(Some(uri), &text)
                            .map_err(|e| ConfigError::Foreign(e))
                    })
            }
            Self::File { path } => fs::read_to_string(path)
                .map_err(|e| ConfigError::Foreign(Box::new(e)))
                .and_then(|text| {
                    format
                        .parse(Some(path), &text)
                        .map_err(|e| ConfigError::Foreign(e))
                }),
        }
    }
}
