use config::{ConfigError, Source, Value};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliArgs {
    // The path to the file to read config
    #[structopt(short = "f", default_value = "data/configuration")]
    pub config_file: String,

    // The path to the file to output blacklist file
    #[structopt(short = "o")]
    pub output_path: Option<String>,
}

impl CliArgs {
    pub fn new() -> Self {
        Self::from_args()
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

#[derive(Debug, Clone, Copy)]
pub struct CliSource;

impl CliSource {
    pub fn new() -> Self {
        Self {}
    }
}

impl Source for CliSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(*self)
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, ConfigError> {
        let cli_args = CliArgs::new();

        let mut map = config::Map::new();

        map.insert("config_dir".to_string(), Value::from(cli_args.config_dir()));
        if let Some(s) = &cli_args.output_path {
            map.insert("output_path".to_string(), Value::from(s.to_string()));
        }

        Ok(map)
    }
}
