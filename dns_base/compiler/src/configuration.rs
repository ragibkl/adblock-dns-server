use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read config
    #[structopt(short = "f", default_value = "data/configuration")]
    config_file: String,

    // The path to the file to output blacklist file
    #[structopt(short = "o", default_value = "data/output.d/blacklist.zone")]
    output_path: String,

    // The path to the file to resolve relative sources
    #[structopt(default_value = ".")]
    workdir: String,
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
    let mut app_config = config::Config::default();

    app_config
        .merge(config::File::with_name(&args.config_file))?
        .set("output_path", args.output_path)?
        .set("workdir", args.workdir)?;

    app_config.try_into()
}
