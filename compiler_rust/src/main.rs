use compiler_rust::service::config;
use compiler_rust::service::core::*;
use compiler_rust::service::loader::HttpLoader;

use compiler_rust::{load_config, run};

fn main() {
    println!("Hello, world!");
    let config = load_config();
    run(config);
}
