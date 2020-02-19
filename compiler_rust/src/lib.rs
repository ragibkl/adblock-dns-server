pub mod service;

use service::core::*;
use service::loader::get_loader;

use service::config::AppConfig;

pub fn load_config() -> AppConfig {
    AppConfig::new()
}

pub fn run(config: AppConfig) {
    let urls = config.get_blacklist_urls();
    let loaders: Vec<Box<dyn Loader>> = urls
        .iter()
        .map(|u| get_loader(u))
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .collect();

    // println!("{:?}", loaders);
}
