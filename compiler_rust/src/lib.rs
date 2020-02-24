#[macro_use]
extern crate lazy_static;
extern crate async_std;

use std::collections::HashSet;

pub mod service;
use service::config::{AppConfig, SourceConfig};
use service::extractor::ExtractTask;
use service::hosts_writer;

pub fn load_config() -> AppConfig {
    AppConfig::new()
}

async fn fetch_list(urls: Vec<SourceConfig>) -> HashSet<String> {
    let mut handles = Vec::new();
    for u in urls {
        let handle = tokio::spawn(async move {
            let t = ExtractTask::from_config(&u);
            t.load_and_parse().await.into_iter().collect::<HashSet<_>>()
        });
        handles.push(handle);
    }

    println!("Start merging set");
    let mut domain_set = HashSet::new();
    for h in handles {
        let s = h.await;
        if let Some(s) = s.ok() {
            domain_set.extend(s);
        }
    }
    println!("Done merging set");

    domain_set
}

pub async fn run(config: AppConfig) {
    let blacklist_urls = config.get_blacklist_srcs();
    let whitelist_urls = config.get_whitelist_srcs();
    let overrides_urls = config.get_overrides_srcs();

    let blacklist_handle = tokio::spawn(async move { fetch_list(blacklist_urls).await });
    let whitelist_handle = tokio::spawn(async move { fetch_list(whitelist_urls).await });
    let overrides_handle = tokio::spawn(async move { fetch_list(overrides_urls).await });

    let blacklist_set = blacklist_handle.await.unwrap();
    let whitelist_set = whitelist_handle.await.unwrap();
    let domains: Vec<String> = blacklist_set
        .difference(&whitelist_set)
        .map(|s| s.clone())
        .collect();

    let overrides_set = overrides_handle.await.unwrap();
    let overrides_list = overrides_set.into_iter().collect::<Vec<_>>();
    let content = hosts_writer::build_content(domains, overrides_list);
    hosts_writer::write_to_file(&config.get_output_path(), &content);
}
