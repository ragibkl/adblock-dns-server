use crate::configuration::Source;
use crate::service::core::*;
use crate::service::loader::load_content;
use crate::service::parser::{CnameParser, HostParser, ListParser, ZoneParser};
use std::sync::Arc;

pub struct ExtractTask {
    source: Source,
    parser: Arc<dyn Parser>,
}

impl ExtractTask {
    pub fn from_config(config: &Source) -> ExtractTask {
        let parser: Arc<dyn Parser> = match config.format.as_str() {
            "hosts" => Arc::new(HostParser::new()),
            "domains" => Arc::new(ListParser::new()),
            "cname" => Arc::new(CnameParser::new()),
            "zone" => Arc::new(ZoneParser::new()),
            _ => panic!("invalid format"),
        };

        ExtractTask {
            source: config.clone(),
            parser,
        }
    }

    pub async fn load_and_parse(&self) -> Vec<String> {
        let parser = Arc::clone(&self.parser);
        let res = load_content(&self.source.kind, &self.source.path).await;

        match res {
            Ok(content) => parser.parse(&content),
            _ => {
                println!("error loading content");
                Vec::new()
            }
        }
    }
}
