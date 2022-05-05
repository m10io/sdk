use crate::collections::*;
use crate::commands::top_level_cmds::Format;
use crate::utils;
use clap::{Parser, Subcommand};
use m10_sdk::{prost::Message, sdk};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    fmt::Debug,
    io::{self, LineWriter},
};

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
    Account(DeserializeOptions),
    /// Deserialize an account set record
    AccountSet(DeserializeOptions),
    /// Deserialize a role record
    Role(DeserializeOptions),
    /// Deserialize a role bindings record
    RoleBinding(DeserializeOptions),
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

        match self.format {
            Format::Json => {
                let stdout = io::stdout();
                let handle = stdout.lock();
                let writer = LineWriter::new(handle);
                serde_json::to_writer_pretty(writer, &printable)?;
            }
            Format::Yaml => {
                let stdout = io::stdout();
                let handle = stdout.lock();
                let writer = LineWriter::new(handle);
                serde_yaml::to_writer(writer, &printable)?;
            }
            Format::Raw => {
                let pretty = PrettyConfig::new()
                    .with_depth_limit(4)
                    .with_separate_tuple_members(true);
                let s = to_string_pretty(&printable, pretty)?;
                println!("{}", s);
            }
        }
        Ok(())
    }
}
impl DeserializeSubCommands {
    pub(super) fn deserialize(&self) -> anyhow::Result<()> {
        match self {
            DeserializeSubCommands::Account(options) => {
                options.print_data::<sdk::Account, accounts::Account>()?;
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
