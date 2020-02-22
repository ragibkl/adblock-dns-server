use std::sync::Arc;
use tokio::sync::Mutex;

use crate::service::config::SourceConfig;
use crate::service::core::*;
use crate::service::loader::{FileLoader, HttpLoader};
use crate::service::parser::{HostParser, ListParser};

pub struct ExtractTask {
    loader: Arc<dyn Loader>,
    parser: Mutex<Box<dyn Parser + Send>>,
}

impl ExtractTask {
    pub fn from_config(config: &SourceConfig) -> ExtractTask {
        let loader: Arc<dyn Loader> = match config.kind.as_str() {
            "http" => Arc::new(HttpLoader{ url: config.path.clone() }),
            "file" => Arc::new(FileLoader{ path: config.path.clone() }),
            _ => panic!("invalid kind"),
        };

        let parser: Mutex<Box<dyn Parser + Send>> = match config.format.as_str() {
            "hosts" => Mutex::new(Box::new(HostParser::new())),
            "domains" => Mutex::new(Box::new(ListParser::new())),
            _ => panic!("invalid format"),
        };

        ExtractTask {
            loader: loader,
            parser: parser,
        }
    }

    pub async fn load_and_parse(&self) -> Vec<String> {
        let loader = Arc::clone(&self.loader);
        let res = loader.load().await;

        match res {
            Ok(content) => self.parser.lock().await.parse(&content),
            _ => {
                println!("error loading content");
                Vec::new()
            }
        }
    }
}
