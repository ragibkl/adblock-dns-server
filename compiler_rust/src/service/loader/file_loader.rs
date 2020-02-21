use std::fs;

use crate::service::core::Loader;

pub struct FileLoader {
    pub path: String,
}

impl Loader for FileLoader {
    fn load(&self) -> Result<String, ()> {
        let contents = fs::read_to_string(&self.path).map_err(|_e| ())?;
        println!("{}", contents);

        Ok(contents)
    }
}
