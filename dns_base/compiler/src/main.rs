use compiler::{load_config, run};

#[tokio::main]
async fn main() {
    let config = load_config().unwrap();
    run(config).await;
}
