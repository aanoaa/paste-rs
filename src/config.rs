use std::path::Path;

use config::ConfigError;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    /// address to bind
    pub address: String,
    /// number of workers
    pub workers: Option<usize>,
}

impl Config {
    pub fn from(path: &Path) -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::File::from(path))
            .build()?
            .try_deserialize()
    }
}
