#[macro_use]
extern crate lazy_static;
extern crate rayon;

use std::collections::HashSet;

use rayon::prelude::*;

pub mod service;

use service::config::AppConfig;
use service::extractor::ExtractTask;
use service::hosts_writer;

pub fn load_config() -> AppConfig {
    AppConfig::new()
}

pub fn run(config: AppConfig) {
    let urls = config.get_blacklist_srcs();
    let sets: Vec<HashSet<String>> = urls
        .par_iter()
        .map(|u| {
            let t = ExtractTask::from_config(u);
            t.load_and_parse().into_iter().collect()
        })
        .collect();

    let mut domain_set = HashSet::new();
    for s in sets {
        domain_set.extend(s);
    }

    let domains: Vec<String> = domain_set.into_iter().collect();
    let content = hosts_writer::build_content(domains);
    hosts_writer::write_to_file(&config.get_output_path(), &content);
}
