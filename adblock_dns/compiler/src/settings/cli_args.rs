use structopt::StructOpt;

use super::config_source::ConfigSource;

#[derive(StructOpt)]
pub struct CliArgs {
    // The path to the file to read config
    #[structopt(short = "f", default_value = "data/configuration")]
    pub config_file: String,

    // The path to the file to output blacklist file
    #[structopt(short = "o", default_value = "data/output.d/blacklist.zone")]
    pub output_path: String,
}

impl CliArgs {
    pub fn new() -> Self {
        Self::from_args()
    }

    pub fn config_source(&self) -> ConfigSource {
        ConfigSource::new(&self.config_file)
    }

    pub fn config_dir(&self) -> String {
        let parts = self.config_file.split('/');
        let count = parts.clone().count();

        parts
            .take(count - 1)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("/")
    }
}
