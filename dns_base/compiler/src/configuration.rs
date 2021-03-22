#[derive(serde::Deserialize, Debug, Clone)]
pub struct Source {
    pub format: String,
    pub path: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub output_path: String,
    pub blacklist: Vec<Source>,
    pub whitelist: Vec<Source>,
    pub overrides: Vec<Source>,
}

pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("data/configuration"))?;
    settings.try_into()
}
