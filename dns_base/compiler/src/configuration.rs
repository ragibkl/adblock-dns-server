use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read config
    #[structopt(short = "f")]
    config_file: Option<String>,

    // The path to the file to output blacklist file
    #[structopt(short = "o")]
    output_path: Option<String>,

    // The path to the file to resolve relative sources
    workdir: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Source {
    pub format: String,
    pub path: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub output_path: String,
    pub workdir: std::path::PathBuf,

    pub blacklist: Vec<Source>,
    pub whitelist: Vec<Source>,
    pub overrides: Vec<Source>,
}

pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let args = Cli::from_args();
    let config_file = args.config_file.unwrap_or("data/configuration".to_string());
    let output_path = args
        .output_path
        .unwrap_or("data/output.d/blacklist.zone".to_string());
    let workdir = args.workdir.unwrap_or(".".to_string());

    let mut app_config = config::Config::default();
    app_config
        .merge(config::File::with_name(&config_file))?
        .set("output_path", output_path)?
        .set("workdir", workdir)?;

    app_config.try_into()
}
