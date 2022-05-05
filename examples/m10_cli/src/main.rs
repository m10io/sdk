use clap::Parser;
use config::{ConfigError, File, Source, Value};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::path::Path;

mod collections;
mod commands;
mod context;
mod utils;

#[serde_as]
#[derive(Clone, Parser, Debug, Deserialize)]
#[clap(about)]
pub(crate) struct Opts {
    /// Load config file for top level options
    #[clap(short, long)]
    pub(crate) config: Option<String>,
    /// Ledger server address
    #[clap(short, long)]
    pub(crate) server: Option<String>,
    /// Disable TLS (e.g. for localhost connection)
    #[clap(long)]
    pub(crate) no_tls: bool,
    /// Load key file (pkcs8) used for request signing
    #[clap(short, long)]
    pub(crate) key_file: Option<String>,
    /// Set context ID filter. Parsed as Hex
    #[clap(long)]
    pub(crate) context_id: Option<String>,
    #[clap(subcommand)]
    cmd: commands::Commands,
}

impl Source for Opts {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut map: HashMap<String, Value> = HashMap::new();
        if let Some(server) = self.server.as_ref() {
            map.insert("server".to_string(), Value::from(server.as_str()));
        }
        if self.no_tls {
            map.insert("no_tls".to_string(), Value::from(self.no_tls));
        }
        if let Some(key) = self.key_file.as_ref() {
            map.insert("key_file".to_string(), Value::from(key.as_str()));
        }
        if let Some(context_id) = &self.context_id {
            map.insert("context_id".to_string(), Value::from(context_id.as_str()));
        }

        Ok(map)
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) server: String,
    pub(crate) no_tls: bool,
    pub(crate) key_file: Option<String>,
    pub(crate) signing_key: Option<String>,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub(crate) context_id: Vec<u8>,
}

impl Config {
    pub fn new(opts: Opts) -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.set_default("server", "localhost")?;
        config.set_default("key_file", std::option::Option::<String>::None)?;
        config.set_default("no_tls", false)?;
        config.set_default("context_id", String::new())?;

        if let Some(path) = opts.config.as_ref() {
            let path = Path::new(path);
            if path.exists() {
                config.merge(File::from(path))?;
            }
        }
        config.merge(opts)?;
        config.merge(config::Environment::with_prefix("M10").ignore_empty(true))?;
        config.try_into()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let config = Config::new(opts.clone())?;
    opts.cmd.run(&config).await?;
    Ok(())
}
