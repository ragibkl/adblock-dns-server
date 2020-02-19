pub mod service;

use compiler_rust::service::core::*;
use compiler_rust::service::loader::HttpLoader;

use service::config::AppConfig;

pub fn load_config() -> AppConfig {
    AppConfig::new()
}

pub fn run(config: AppConfig) {
    
}
