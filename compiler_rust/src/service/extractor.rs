use crate::service::config::SourceConfig;
use crate::service::core::*;
use crate::service::loader::{FileLoader, HttpLoader};
use crate::service::parser::{HostParser, ListParser};
use std::sync::Arc;

pub struct ExtractTask {
    loader: Arc<dyn Loader>,
    parser: Arc<dyn Parser>,
}

impl ExtractTask {
    pub fn from_config(config: &SourceConfig) -> ExtractTask {
        let loader: Arc<dyn Loader> = match config.kind.as_str() {
            "http" => Arc::new(HttpLoader {
                url: config.path.clone(),
            }),
            "file" => Arc::new(FileLoader {
                path: config.path.clone(),
            }),
            _ => panic!("invalid kind"),
        };

        let parser: Arc<dyn Parser> = match config.format.as_str() {
            "hosts" => Arc::new(HostParser::new()),
            "domains" => Arc::new(ListParser::new()),
            _ => panic!("invalid format"),
        };

        ExtractTask {
            loader: loader,
            parser: parser,
        }
    }

    pub async fn load_and_parse(&self) -> Vec<String> {
        let loader = Arc::clone(&self.loader);
        let parser = Arc::clone(&self.parser);
        let res = loader.load().await;

        match res {
            Ok(content) => parser.parse(&content),
            _ => {
                println!("error loading content");
                Vec::new()
            }
        }
    }
}
