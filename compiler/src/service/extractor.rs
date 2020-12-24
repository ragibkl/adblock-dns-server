use crate::configuration::Source;
use crate::service::loader::load_content;
use crate::service::parser::{parse_cnames, parse_domains, parse_hosts, parse_zones};

fn parse(format: &str, content: &str) -> Vec<String> {
    match format {
        "hosts" => parse_hosts(content),
        "domains" => parse_domains(content),
        "cname" => parse_cnames(content),
        "zone" => parse_zones(content),
        _ => panic!("invalid format"),
    }
}

pub struct ExtractTask {
    source: Source,
}

impl ExtractTask {
    pub fn from_config(source: &Source) -> ExtractTask {
        ExtractTask {
            source: source.clone(),
        }
    }

    pub async fn load_and_parse(&self) -> Vec<String> {
        let res = load_content(&self.source.path).await;

        match res {
            Ok(content) => parse(&self.source.format, &content),
            _ => {
                println!("error loading content");
                Vec::new()
            }
        }
    }
}
