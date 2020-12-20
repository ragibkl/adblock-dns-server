use super::file::load_file;
use super::http::load_http;

pub async fn load_content(kind: &str, path: &str) -> Result<String, ()> {
    match kind {
        "http" => load_http(path).await,
        "file" => load_file(path).await,
        _ => panic!("invalid kind"),
    }
}
