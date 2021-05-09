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
pub enum BlacklistFormat {
    hosts,
    domains,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum WhitelistFormat {
    hosts,
    domains,
    cname,
    zone,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum OverrideFormat {
    cname,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Source<T> {
    pub format: T,
    pub path: String,
}

pub type Blacklist = Source<BlacklistFormat>;
pub type Whitelist = Source<WhitelistFormat>;
pub type Overrides = Source<OverrideFormat>;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub output_path: String,
    pub workdir: std::path::PathBuf,

    pub blacklist: Vec<Blacklist>,
    pub whitelist: Vec<Whitelist>,
    pub overrides: Vec<Overrides>,
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
