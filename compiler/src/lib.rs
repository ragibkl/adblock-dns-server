#[macro_use]
extern crate lazy_static;
extern crate async_std;

use std::collections::HashSet;

pub mod configuration;
pub mod service;

pub use configuration::load_config;
use configuration::{AppConfig, Source};
use service::extractor::ExtractTask;
use service::hosts_writer;

async fn fetch_list(urls: Vec<Source>) -> HashSet<String> {
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
    let blacklist_urls = config.blacklist;
    let whitelist_urls = config.whitelist;
    let overrides_urls = config.overrides;

    let blacklist_handle = tokio::spawn(fetch_list(blacklist_urls));
    let whitelist_handle = tokio::spawn(fetch_list(whitelist_urls));
    let overrides_handle = tokio::spawn(fetch_list(overrides_urls));

    let blacklist_set = blacklist_handle.await.unwrap();
    let whitelist_set = whitelist_handle.await.unwrap();
    let domains: Vec<String> = blacklist_set
        .difference(&whitelist_set)
        .map(|s| s.clone())
        .collect();

    let overrides_set = overrides_handle.await.unwrap();
    let overrides_list = overrides_set.into_iter().collect::<Vec<_>>();
    let content = hosts_writer::build_content(domains, overrides_list);
    hosts_writer::write_to_file(&config.output_path, &content);
}
