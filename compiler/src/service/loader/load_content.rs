use super::file::load_file;
use super::http::load_http;

pub async fn load_content(path: &str) -> Result<String, ()> {
    if path.starts_with("http") {
        return load_http(path).await;
    }

    load_file(path).await
}
