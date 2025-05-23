use std::{collections::HashMap, path::Path};

pub use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) addr: Option<String>,
    pub(crate) key: Option<String>,
    #[serde(default)]
    pub(crate) profile: HashMap<String, Profile>,
    #[serde(default)]
    pub vault_addr: Option<String>,
    #[serde(default)]
    pub vault_token: Option<String>,
    #[serde(default)]
    pub vault_key_name: Option<String>,
    #[serde(default)]
    pub vault_mount: Option<String>,
    #[serde(default)]
    pub vault_namespace: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Profile {
    pub(crate) key: String,
    pub(crate) addr: Option<String>,
    #[serde(default)]
    pub vault_addr: Option<String>,
    #[serde(default)]
    pub vault_token: Option<String>,
    #[serde(default)]
    pub vault_key_name: Option<String>,
    #[serde(default)]
    pub vault_mount: Option<String>,
    #[serde(default)]
    pub vault_namespace: Option<String>,
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
