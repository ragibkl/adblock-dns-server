use crate::service::config::SourceConfig;
use crate::service::core::*;
use crate::service::loader::HttpLoader;
use crate::service::parser::HostParser;

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
        let parser = match config.format.as_str() {
            "hosts" => HostParser::new(),
            _ => panic!("invalid format"),
        };
        ExtractTask {
            loader: Box::new(loader),
            parser: Box::new(parser),
        }
    }

    pub fn load_and_parse(&self) -> Vec<String> {
        let content = self.loader.load().unwrap();
        let domains = self.parser.parse(content);
        domains
    }
}
