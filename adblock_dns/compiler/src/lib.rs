#[macro_use]
extern crate lazy_static;

pub mod service;
pub mod settings;

pub use settings::{load_config, AppConfig};

use service::blacklist::extract_blacklist;
use service::hosts_writer;
use service::overrides::extract_overrides;
use service::whitelist::{extract_whitelist, filter_whitelist};

pub async fn run(config: AppConfig) {
    let blacklist_handle = tokio::spawn(extract_blacklist(config.clone()));
    let whitelist_handle = tokio::spawn(extract_whitelist(config.clone()));
    let overrides_handle = tokio::spawn(extract_overrides(config.clone()));

    let blacklists = blacklist_handle.await.unwrap();
    let whitelists = whitelist_handle.await.unwrap();
    let overrides = overrides_handle.await.unwrap();

    let domains = filter_whitelist(&blacklists, &whitelists);
    let content = hosts_writer::build_content(domains, overrides);

    hosts_writer::write_to_file(&config.output_path, &content);
}
