use structopt::StructOpt;

use crate::service::loader::load_content;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read config
    #[structopt(short = "f", default_value = "data/configuration")]
    config_file: String,

    // The path to the file to output blacklist file
    #[structopt(short = "o", default_value = "data/output.d/blacklist.zone")]
    output_path: String,
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
    pub config_dir: String,
    pub output_path: String,

    pub blacklist: Vec<Blacklist>,
    pub whitelist: Vec<Whitelist>,
    pub overrides: Vec<Overrides>,
}

fn get_config_dir(config_path: &str) -> String {
    let parts = config_path.split('/');
    let count = parts.clone().count();
    parts
        .take(count - 1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/")
}

pub async fn load_config() -> Result<AppConfig, config::ConfigError> {
    let args = Cli::from_args();
    let config_path = args.config_file;

    let config_content = load_content(&config_path).await.unwrap();

    let mut app_config = config::Config::default();
    app_config
        .merge(config::File::from_str(
            &config_content,
            config::FileFormat::Yaml,
        ))?
        .set("config_dir", get_config_dir(&config_path))?
        .set("output_path", args.output_path)?;
    app_config.try_into()
}
