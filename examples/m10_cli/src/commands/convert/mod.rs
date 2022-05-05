use clap::Subcommand;
pub(super) use id_convert::BinFormat;
use serde::{Deserialize, Serialize};

mod deserialize;
mod id_convert;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) enum ConvertSubCommands {
    /// Convert between formats
    Id(id_convert::ConvertOptions),
    /// Convert a 16 byte account id to chain notation
    AccountChainFrom(id_convert::ConvertOptions),
    /// Convert an account id in chain notation to bytes
    AccountChainTo(id_convert::ConvertOptions),
    /// Unpack raw wire data into protobuf messages
    #[clap(subcommand)]
    Unpack(deserialize::DeserializeSubCommands),
}

impl ConvertSubCommands {
    pub(crate) fn convert(&self) -> anyhow::Result<()> {
        match self {
            ConvertSubCommands::Id(op) => op.convert(),
            ConvertSubCommands::AccountChainFrom(op) => op.handle_account_chain_from(),
            ConvertSubCommands::AccountChainTo(op) => op.handle_account_chain_to(),
            ConvertSubCommands::Unpack(op) => op.deserialize(),
        }
    }
}
