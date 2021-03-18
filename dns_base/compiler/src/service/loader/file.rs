use std::fs;

pub async fn load_file(path: &str) -> Result<String, ()> {
    let contents = fs::read_to_string(&path).map_err(|_e| ())?;
    println!("Loaded data from:{}", &path);
    Ok(contents)
}
