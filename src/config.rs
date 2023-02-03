use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use config::ConfigError;

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Config {
    /// address to bind
    pub address: String,
    /// worker count
    pub workers: Option<usize>,
    /// local path to file upload
    pub upload_path: PathBuf,
    /// max content length
    pub max_content_length: usize,
    pub delete_expired_enabled: Option<bool>,
    #[serde(default, with = "humantime_serde")]
    pub delete_expired_ttl: Option<Duration>,
}

impl Config {
    pub fn from(path: &Path) -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::File::from(path))
            .build()?
            .try_deserialize()
    }
}
