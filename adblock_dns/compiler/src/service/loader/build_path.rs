pub fn build_path(config_dir: &str, path: &str) -> String {
    if path.starts_with("http") || path.starts_with("/") {
        return path.to_string();
    }

    config_dir
        .split("/")
        .chain(path.split("/"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/")
}
