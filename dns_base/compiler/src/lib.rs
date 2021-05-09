#[macro_use]
extern crate lazy_static;

use std::env;

pub mod configuration;
pub mod service;

pub use configuration::load_config;
use configuration::AppConfig;

use service::blacklist::extract_blacklist;
use service::hosts_writer;
use service::overrides::extract_overrides;
use service::whitelist::{extract_whitelist, filter_whitelist};

pub async fn run(config: AppConfig) {
    env::set_current_dir(&config.workdir).unwrap();

    let blacklist_handle = tokio::spawn(extract_blacklist(config.clone()));
    let whitelist_handle = tokio::spawn(extract_whitelist(config.clone()));
    let overrides_handle = tokio::spawn(extract_overrides(config.clone()));

    let blacklists = blacklist_handle.await.unwrap();
    let whitelists = whitelist_handle.await.unwrap();

    let domains = filter_whitelist(&blacklists, &whitelists);

    let overrides_list = overrides_handle.await.unwrap();
    let content = hosts_writer::build_content(domains, overrides_list);
    hosts_writer::write_to_file(&config.output_path, &content);
}
