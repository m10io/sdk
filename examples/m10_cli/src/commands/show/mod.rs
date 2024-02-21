use clap::Subcommand;
use m10_sdk::Format;
use serde::{Deserialize, Serialize};

use crate::collections::contracts::show_contract;

mod account;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Show {
    /// Show account details
    #[command(alias = "a")]
    Account {
        /// Account Id
        id: String,
        #[command(subcommand)]
        cmd: account::Account,
    },
    /// Show contract details
    #[command(alias = "c")]
    Contract {
        /// file path
        path: String,
    },
}

impl Show {
    pub(super) async fn run(self, format: Format) -> anyhow::Result<()> {
        match self {
            Show::Contract { path } => show_contract(&path, format).await,
            Show::Account { id, cmd } => cmd.run(id),
        }
    }
}
