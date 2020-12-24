use crate::configuration::AppConfig;
use crate::service::loader::load_content;
use crate::service::parser::{parse_domains, parse_hosts};

fn parse(format: &str, content: &str) -> Vec<String> {
    match format {
        "hosts" => parse_hosts(content),
        "domains" => parse_domains(content),
        _ => panic!("invalid format"),
    }
}

pub async fn extract_blacklist(config: AppConfig) -> Vec<String> {
    let mut handles = Vec::new();
    for source in &config.blacklist {
        let path = source.path.clone();
        let format = source.format.clone();

        let task = tokio::spawn(async move {
            let content = load_content(&path).await.unwrap_or_default();
            parse(&format, content.as_str())
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
