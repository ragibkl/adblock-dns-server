mod cli_args;
mod config_source;
mod configuration;

use config::builder::AsyncState;
pub use configuration::*;

pub async fn load_config() -> Result<AppConfig, config::ConfigError> {
    let cli_args = cli_args::CliArgs::new();

    let settings = config::ConfigBuilder::<AsyncState>::default()
        .add_async_source(cli_args.config_source())
        .add_source(config::Environment::with_prefix("APP"))
        .set_override("config_dir", cli_args.config_dir())?
        .set_override("output_path", cli_args.output_path.clone())?
        .build()
        .await?;

    settings.try_deserialize()
}
