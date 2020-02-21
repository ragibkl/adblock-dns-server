extern crate reqwest;

use crate::service::core::Loader;

pub struct HttpLoader {
    pub url: String,
}

impl Loader for HttpLoader {
    fn load(&self) -> Result<String, ()> {
        for _i in 0..5 {
            let response = reqwest::blocking::get(&self.url);

            if let Some(r) = response.ok() {
                println!("Done fetch from: {}, {}", &self.url, r.status());
                return r.text().map_err(|e| {
                    println!("Error reading from: {}, {}", &self.url, e);
                    ()
                });
            }
        }
        println!("Cannot fetch from: {}", &self.url);
        Err(())
    }
}
