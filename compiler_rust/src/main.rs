use compiler_rust::service::config;
use compiler_rust::service::core::*;
use compiler_rust::service::loader::HttpLoader;

fn main() {
    println!("Hello, world!");
    let a = HttpLoader {
        url: "https://httpbin.org/ip".to_string(),
    };
    a.load();

    let c = config::Config::new();
    c.get_blacklist_urls();
}
