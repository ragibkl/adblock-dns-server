use compiler_rust::lib::core::*;
use compiler_rust::lib::loader::HttpLoader;

fn main() {
    println!("Hello, world!");
    let a = HttpLoader { url: "https://httpbin.org/ip".to_string() };
    a.load();
}
