use std::convert::TryInto;

use clap::{Subcommand, ValueEnum};
use m10_sdk::Format;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils;

mod deserialize;

#[derive(Clone, ValueEnum, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum BinFormat {
    HexStr,
    Uuid,
    IntArray,
    HexArray,
    Base64,
}

impl BinFormat {
    pub(crate) fn print_bytes(&self, bytes: &[u8], exclude: Vec<Self>) -> anyhow::Result<()> {
        if exclude.contains(self) {
            return Err(anyhow::anyhow!("Unsupported format"));
        }
        match self {
            BinFormat::IntArray => println!("{:?}", bytes),
            BinFormat::HexArray => println!(
                "[{}]",
                bytes
                    .iter()
                    .map(|b| format!("{b:#02x?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            BinFormat::HexStr => println!("{}", hex::encode(bytes)),
            BinFormat::Base64 => println!("{}", base64::encode(bytes)),
            BinFormat::Uuid => println!("{}", Uuid::from_slice(bytes)?),
        }
        Ok(())
    }
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Convert {
    /// Convert between formats
    #[command(alias = "i")]
    Id {
        /// Set output format
        #[arg(short = 't', long, value_enum, default_value = "hex-str")]
        to: BinFormat,
        /// Input data
        data: String,
    },
    /// Convert a 16 byte account id to chain notation
    #[command(alias = "ta")]
    ToAccountChain {
        /// Input data
        data: String,
    },
    /// Convert an account id in chain notation to bytes
    #[command(alias = "a")]
    AccountChain {
        /// Set output format
        #[arg(short = 't', long, value_enum, default_value = "hex-str")]
        to: BinFormat,
        /// Input data
        data: String,
    },
    /// Unpack raw wire data into protobuf messages
    #[command(alias = "m")]
    Message {
        /// Data from log
        data: String,
        /// Output format (one of 'json', 'yaml', 'raw')
        #[arg(short = 'f', long, default_value = "raw")]
        #[serde(default)]
        format: Format,
        #[command(subcommand)]
        cmd: deserialize::Deserialize,
    },
}

impl Convert {
    pub(crate) fn convert(self) -> anyhow::Result<()> {
        match self {
            Convert::Id { to, data } => Self::convert_id(to, &data),
            Convert::ToAccountChain { data } => Self::convert_account_chain_from(&data),
            Convert::AccountChain { to, data } => Self::convert_account_chain_to(to, &data),
            Convert::Message { data, format, cmd } => cmd.run(data, format),
        }
    }

    fn try_convert_from(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        utils::vec_from_hex_array(data)
            .or_else(|_| {
                utils::vec_from_int_array(data).or_else(|_| {
                    hex::decode(data).or_else(|_| {
                        base64::decode(data)
                            .or_else(|_| Uuid::parse_str(data).map(|u| u.as_bytes().to_vec()))
                    })
                })
            })
            .map_err(|_| anyhow::anyhow!("unsuported data format"))
    }

    fn convert_id(to: BinFormat, data: &str) -> anyhow::Result<()> {
        to.print_bytes(&Self::try_convert_from(data)?, vec![])
    }

    fn convert_account_chain_from(data: &str) -> anyhow::Result<()> {
        let bytes = Self::try_convert_from(data)?;
        if bytes.len() != 16 {
            return Err(anyhow::anyhow!("account id requires 16 bytes"));
        }
        let raw_id = u128::from_be_bytes((&bytes[0..16]).try_into()?);
        let account_id = m10_sdk::account::AccountId::from_raw(raw_id)?;
        utils::pprint_account_id(&account_id);
        Ok(())
    }

    fn convert_account_chain_to(to: BinFormat, data: &str) -> anyhow::Result<()> {
        let id = utils::account_id_from_str(data)?;
        let raw_id = id.to_be_bytes();
        to.print_bytes(&raw_id[..], vec![])?;
        Ok(())
    }
}
