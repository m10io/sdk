use std::{collections::HashMap, path::Path};

pub use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) addr: Option<String>,
    #[serde(default)]
    pub(crate) key: Option<String>,
    #[serde(default)]
    pub(crate) profile: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Profile {
    pub(crate) key: String,
    pub(crate) addr: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = config::Config::builder();
        let config_files = &["./cli_config.toml"];
        for config_file in config_files {
            config = config.add_source(config::File::from(Path::new(config_file)).required(false));
        }
        config.build()?.try_deserialize()
    }
}
