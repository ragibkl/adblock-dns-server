// use std::sync::{Arc,Mutex};

use tokio::sync::Mutex;

use crate::service::config::SourceConfig;
use crate::service::core::*;
use crate::service::loader::HttpLoader;
use crate::service::parser::HostParser;
use crate::service::parser::ListParser;

enum LoaderEnum {
    Http(String),
    File(String),
}

pub struct ExtractTask {
    loader: LoaderEnum,
    parser: Mutex<Box<dyn Parser + Send>>,
}

impl ExtractTask {
    pub fn from_config(config: &SourceConfig) -> ExtractTask {
        let loader: LoaderEnum = match config.kind.as_str() {
            "http" => LoaderEnum::Http(config.path.clone()),
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

    pub async fn load_and_parse(self) -> Vec<String> {
        let res = match self.loader {
            LoaderEnum::Http(url) => {
                let l = HttpLoader { url: url.clone() };
                l.load()
            }
            _ => panic!("invalid kind"),
        }
        .await;

        match res {
            Ok(content) => self.parser.lock().await.parse(&content),
            _ => {
                println!("error loading content");
                Vec::new()
            }
        }
    }
}
