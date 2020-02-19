use compiler_rust::{load_config, run};

fn main() {
    let config = load_config();
    run(config);
}
