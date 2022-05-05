use crate::{commands::convert::BinFormat, utils};
use clap::{ArgGroup, Parser};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("method"), about)]
pub(crate) struct GetKeyPairOptions {
    /// File name of key pair
    key_file: Option<String>,
    /// Set output format (one of 'hex-str', 'int-array', hex-array', 'base64')
    #[clap(short, long, default_value = "base64")]
    format: crate::commands::convert::BinFormat,
}

impl GetKeyPairOptions {
    pub(crate) fn get(&self, config: &crate::Config) -> Result<(), anyhow::Error> {
        let key_pair = if let Some(key_file) = &self.key_file {
            utils::load_key_pair_exportable(key_file)?
        } else if let Some(k) = &config.signing_key {
            base64::decode(k)?
        } else {
            return Err(anyhow::anyhow!("Missing key"));
        };
        match self.format {
            BinFormat::IntBytes => println!("{:?}", key_pair),
            BinFormat::HexBytes => println!("{:02x?}", key_pair),
            BinFormat::HexString => println!("{}", hex::encode(key_pair)),
            BinFormat::Base64 => println!("{}", base64::encode(key_pair)),
            _ => {
                return Err(anyhow::anyhow!("Unsupported format"));
            }
        }
        Ok(())
    }
}
