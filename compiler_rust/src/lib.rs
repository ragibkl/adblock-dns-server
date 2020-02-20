#[macro_use]
extern crate lazy_static;
pub mod service;

use service::config::AppConfig;
use service::extractor::ExtractTask;

pub fn load_config() -> AppConfig {
    AppConfig::new()
}

pub fn run(config: AppConfig) {
    let urls = config.get_blacklist_srcs();
    let tasks: Vec<ExtractTask> = urls.iter().map(|u| ExtractTask::from_config(u)).collect();

    let mut domains: Vec<String> = Vec::new();
    for t in tasks {
        let d = t.load_and_parse();
        domains.extend(d);
    }
    println!("{:?}", domains);
}
