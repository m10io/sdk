use crate::{commands::convert::BinFormat, utils};
use clap::{ArgGroup, Parser};
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("method"), about)]
pub(crate) struct CreateKeyPairOptions {
    /// File name to store keypair
    key_file: Option<String>,
    /// Set format of output (one of 'hex-str', 'int-array', hex-array', 'base64')
    #[clap(short = 'f', long, default_value = "base64")]
    format: BinFormat,
    /// Set algorithm to ED25519 (default)
    #[clap(short, long, group = "method")]
    ed25519: bool,
    /// Set algorithm to P256
    #[clap(short, long, group = "method")]
    p256: bool,
}

impl CreateKeyPairOptions {
    pub(super) fn create(&self) -> anyhow::Result<()> {
        let method = if self.p256 {
            sdk::signature::Algorithm::P256Sha256Asn1
        } else {
            sdk::signature::Algorithm::Ed25519
        };
        let buf = if let Some(key_file) = &self.key_file {
            let k = utils::create_key_pair(key_file, method)?;
            eprintln!("public key is:");
            k
        } else {
            let k = utils::create_exportable_key_pair(method)?;
            eprintln!("key pair is:");
            k
        };
        match self.format {
            BinFormat::IntBytes => println!("{:?}", buf),
            BinFormat::HexBytes => println!("{:02x?}", buf),
            BinFormat::HexString => println!("{:?}", hex::encode(buf)),
            BinFormat::Base64 => println!("{:?}", base64::encode(buf)),
            _ => {
                return Err(anyhow::anyhow!("unsuported format"));
            }
        }
        Ok(())
    }
}
