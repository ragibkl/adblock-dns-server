use std::error::Error;
extern crate reqwest;

use crate::service::core::Loader;

pub struct HttpLoader {
    pub url: String,
}

impl Loader for HttpLoader {
    fn load(&self) -> Result<String, Box<dyn Error>> {
        let contents = reqwest::blocking::get(&self.url)?.text()?;
        Ok(contents)
    }
}
