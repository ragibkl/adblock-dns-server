use crate::service::loader::{build_path, load_content};
use crate::service::parser::parse_cnames;
use crate::settings::{AppConfig, OverrideFormat};

fn parse(format: &OverrideFormat, content: &str) -> Vec<String> {
    match format {
        OverrideFormat::Cname => parse_cnames(content),
    }
}

pub async fn extract_overrides(config: AppConfig) -> Vec<String> {
    let mut handles = Vec::new();
    for source in &config.overrides {
        let path = build_path(&config.config_dir, &source.path);
        let format = source.format.clone();

        let task = tokio::spawn(async move {
            let content = load_content(&path).await.unwrap_or_default();
            let lines = parse(&format, content.as_str());
            println!(
                "[parsed overrides] - Done parsing {} domains from {}",
                lines.len(),
                &path
            );
            lines
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
