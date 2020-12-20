use crate::configuration::AppConfig;
use crate::service::core::*;
use crate::service::loader::load_content;
use crate::service::parser::{CnameParser, HostParser, ListParser, ZoneParser};
use std::sync::Arc;

async fn parse(format: &str, content: &str) -> Vec<String> {
    let parser: Arc<dyn Parser> = match format {
        "hosts" => Arc::new(HostParser::new()),
        "domains" => Arc::new(ListParser::new()),
        "cname" => Arc::new(CnameParser::new()),
        "zone" => Arc::new(ZoneParser::new()),
        _ => panic!("invalid format"),
    };

    parser.parse(content)
}

pub async fn extract_blacklist(config: AppConfig) -> Vec<String> {
    let mut handles = Vec::new();
    for source in &config.blacklist {
        let kind = source.kind.clone();
        let path = source.path.clone();
        let format = source.format.clone();

        let task = tokio::spawn(async move {
            let content = load_content(&kind, &path).await.unwrap_or_default();
            parse(&format, content.as_str()).await
        });

        handles.push(task);
    }

    let mut results = Vec::new();
    for handle in handles {
        let s = handle.await.unwrap();
        results.extend(s);
    }

    results
}
