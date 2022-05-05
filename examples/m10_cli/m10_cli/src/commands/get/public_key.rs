use crate::{commands::convert::BinFormat, utils};
use clap::{ArgGroup, Parser};
use m10_sdk::Signer;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("method"), about)]
pub(crate) struct GetPubKeyOptions {
    /// File name of key pair
    key_file: Option<String>,
    /// Set output format (one of 'hex-str', 'int-array', hex-array', 'base64')
    #[clap(short, long, default_value = "base64")]
    format: crate::commands::convert::BinFormat,
}

impl GetPubKeyOptions {
    pub(crate) fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        let pub_key = if let Some(key_file) = &self.key_file {
            utils::load_key_pair(key_file)?.public_key().to_vec()
        } else if let Some(k) = &config.signing_key {
            utils::key_pair_from_env(k)?.public_key().to_vec()
        } else {
            return Err(anyhow::anyhow!("Missing key"));
        };
        match self.format {
            BinFormat::IntBytes => println!("{:?}", pub_key),
            BinFormat::HexBytes => println!("{:02x?}", pub_key),
            BinFormat::HexString => println!("{}", hex::encode(pub_key)),
            BinFormat::Base64 => println!("{}", base64::encode(pub_key)),
            _ => {
                return Err(anyhow::anyhow!("Unsupported format"));
            }
        }
        Ok(())
    }
}
