mod cli;
mod config_source;
mod configuration;

use config::{builder::AsyncState, ConfigBuilder};

use cli::CliSource;
use config_source::ConfigSource;
pub use configuration::*;

pub async fn load_config() -> Result<AppConfig, config::ConfigError> {
    let settings = ConfigBuilder::<AsyncState>::default()
        .set_default("output_path", "data/output.d/blacklist.zone")?
        .add_source(CliSource::new())
        .add_async_source(ConfigSource::new())
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .await?;

    settings.try_deserialize()
}
