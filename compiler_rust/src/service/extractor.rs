use crate::service::config::SourceConfig;
use crate::service::core::*;
use crate::service::loader::HttpLoader;
use crate::service::parser::HostParser;
use crate::service::parser::ListParser;

pub struct ExtractTask {
    loader: Box<dyn Loader>,
    parser: Box<dyn Parser>,
}

impl ExtractTask {
    pub fn from_config(config: &SourceConfig) -> ExtractTask {
        let loader = match config.kind.as_str() {
            "http" => HttpLoader {
                url: config.path.clone(),
            },
            _ => panic!("invalid kind"),
        };
        let parser: Box<dyn Parser> = match config.format.as_str() {
            "hosts" => Box::new(HostParser::new()),
            "domains" => Box::new(ListParser::new()),
            _ => panic!("invalid format"),
        };
        ExtractTask {
            loader: Box::new(loader),
            parser,
        }
    }

    pub fn load_and_parse(&self) -> Vec<String> {
        let res = self.loader.load();
        match res {
            Ok(content) => self.parser.parse(&content),
            _ => {
                println!("error loading content");
                Vec::new()
            }
        }
    }
}
