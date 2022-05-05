use crate::utils;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, str::FromStr};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) enum BinFormat {
    HexString,
    Uuid,
    IntBytes,
    HexBytes,
    Base64,
}

impl FromStr for BinFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hex-str" => Ok(BinFormat::HexString),
            "uuid" => Ok(BinFormat::Uuid),
            "int-array" => Ok(BinFormat::IntBytes),
            "hex-array" => Ok(BinFormat::HexBytes),
            "base64" => Ok(BinFormat::Base64),
            _ => Err("no match"),
        }
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct ConvertOptions {
    /// Set input format (one of 'hex-str', 'uuid', 'int-array', hex-array', 'base64')
    #[clap(short = 'f', long, default_value = "hex-str")]
    from: BinFormat,
    /// Set output format (one of 'hex-str', 'uuid', 'int-array', hex-array', 'base64')
    #[clap(short = 't', long, default_value = "hex-str")]
    to: BinFormat,
    /// Input data, format must match --from option
    data: String,
}

impl ConvertOptions {
    fn convert_from(&self) -> Result<Vec<u8>, anyhow::Error> {
        let data = match self.from {
            BinFormat::HexBytes => utils::vec_from_hex_array(&self.data)?,
            BinFormat::Uuid => Uuid::parse_str(&self.data)?.as_bytes().to_vec(),
            BinFormat::IntBytes => utils::vec_from_int_array(&self.data)?,
            BinFormat::HexString => hex::decode(&self.data)?,
            BinFormat::Base64 => base64::decode(&self.data)?,
        };
        Ok(data)
    }

    fn convert_to(&self, data: &[u8]) -> anyhow::Result<()> {
        match self.to {
            BinFormat::HexBytes => println!("{:02x?}", data),
            BinFormat::Uuid => {
                let uuid = Uuid::from_slice(data)?;
                println!("{}", uuid);
            }
            BinFormat::IntBytes => println!("{:?}", data),
            BinFormat::HexString => println!("{:?}", hex::encode(data)),
            BinFormat::Base64 => println!("{:?}", base64::encode(data)),
        }
        Ok(())
    }

    pub(super) fn convert(&self) -> anyhow::Result<()> {
        self.convert_to(&self.convert_from()?)
    }

    pub(super) fn handle_account_chain_from(&self) -> anyhow::Result<()> {
        let bytes = self.convert_from()?;
        if bytes.len() != 16 {
            return Err(anyhow::anyhow!("account id requires 16 bytes"));
        }
        let raw_id = u128::from_be_bytes((&bytes[0..16]).try_into()?);
        let account_id = m10_sdk::account::AccountId::from_raw(raw_id)?;
        utils::pprint_account_id(&account_id);
        Ok(())
    }

    pub(super) fn handle_account_chain_to(&self) -> anyhow::Result<()> {
        let id = utils::account_id_from_str(&self.data)?;
        let raw_id = id.to_be_bytes();
        self.convert_to(&raw_id[..])?;
        Ok(())
    }
}
