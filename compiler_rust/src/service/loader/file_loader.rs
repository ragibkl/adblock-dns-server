use std::error::Error;
use std::fs;

use crate::service::core::Loader;

pub struct FileLoader {
    pub path: String,
}

impl Loader for FileLoader {
    fn load(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(&self.path)?;
        println!("{}", contents);

        Ok(contents)
    }
}
