use std::{path::Path, sync::Arc};

pub use config::{ConfigError, File};
use m10_sdk::client::Endpoint;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub ledger_addr: Endpoint,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub directory_addr: Endpoint,
    pub operator_key: Arc<m10_sdk::Ed25519>,
    pub central_bank_key: Arc<m10_sdk::Ed25519>,
    pub banks_key: Arc<m10_sdk::Ed25519>,
    pub sandbox_issuer_role: Uuid,
    pub sandbox_nostro_role: Uuid,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = config::Config::default();
        let config_files = &[
            "./resources/config.toml",
            "./config.toml",
            "/Applications/m10-sdk-tester.app/Contents/Resources/resources/config.toml",
        ];
        for config_file in config_files {
            config.merge(config::File::from(Path::new(config_file)).required(false))?;
        }
        config.merge(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
                .ignore_empty(true),
        )?;
        config.try_into()
    }
}
