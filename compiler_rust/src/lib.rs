#[macro_use]
extern crate lazy_static;
extern crate async_std;
extern crate rayon;

use std::collections::HashSet;

pub mod service;
use service::config::AppConfig;
use service::extractor::ExtractTask;
use service::hosts_writer;

pub fn load_config() -> AppConfig {
    AppConfig::new()
}

pub async fn run(config: AppConfig) {
    let urls = config.get_blacklist_srcs();

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

    let domains: Vec<String> = domain_set.into_iter().collect();
    let content = hosts_writer::build_content(domains);
    hosts_writer::write_to_file(&config.get_output_path(), &content);
}
