extern crate reqwest;

pub async fn load_http(url: &str) -> Result<String, ()> {
    for i in 0..3 {
        println!("Start fetch from: {}", url);
        let response = reqwest::get(url).await;
        if let Some(r) = response.ok() {
            if let Some(text) = r.text().await.ok() {
                println!("Done fetch from: {}", &url);
                return Ok(text);
            }
            println!("Retry fetch from: {}, retries: {}", &url, i + 1);
        }
    }

    println!("Cannot fetch from: {}", &url);
    Err(())
}
