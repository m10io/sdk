use crate::{collections::*, utils};
use clap::{Parser, Subcommand};
use m10_protos::prost::Message;
use m10_sdk::{sdk, Format, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct DeserializeOptions {
    /// Data from log
    data: String,
    /// Output format (one of 'json', 'yaml', 'raw')
    #[clap(short = 'f', long, default_value = "raw")]
    #[serde(default = "Format::default")]
    format: Format,
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) enum DeserializeSubCommands {
    /// Deserialize an account record
    AccountMetadata(DeserializeOptions),
    /// Deserialize an account set record
    AccountSet(DeserializeOptions),
    /// Deserialize a role record
    Role(DeserializeOptions),
    /// Deserialize a role bindings record
    RoleBinding(DeserializeOptions),
}

impl DeserializeSubCommands {
    pub(super) fn deserialize(&self) -> anyhow::Result<()> {
        match self {
            DeserializeSubCommands::AccountMetadata(options) => {
                options.print_data::<sdk::AccountMetadata, accounts::AccountMetadata>()?;
            }
            DeserializeSubCommands::AccountSet(options) => {
                options.print_data::<sdk::AccountSet, account_sets::AccountSet>()?;
            }
            DeserializeSubCommands::Role(options) => {
                options.print_data::<sdk::Role, roles::Role>()?;
            }
            DeserializeSubCommands::RoleBinding(options) => {
                options.print_data::<sdk::RoleBinding, role_bindings::RoleBinding>()?;
            }
        }
        Ok(())
    }
}

impl DeserializeOptions {
    fn print_data<D, I>(&self) -> anyhow::Result<()>
    where
        D: Message + Default,
        I: TryFrom<D, Error = anyhow::Error> + Serialize,
    {
        let buf = utils::vec_from_int_array(&self.data)?;
        let item = D::decode(buf.as_slice())?;
        let printable = I::try_from(item)?;
        printable.print(self.format)?;
        Ok(())
    }
}
