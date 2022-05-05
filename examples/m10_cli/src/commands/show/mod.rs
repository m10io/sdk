use crate::collections::contracts::show_contract;
use crate::commands::Format;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod account;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(super) struct ShowItemOptions {
    /// file path
    path: String,
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum ShowSubCommands {
    /// Show contract details
    Contract(ShowItemOptions),
    /// Show account details
    #[clap(subcommand)]
    Account(account::ShowAccountCommands),
}

impl ShowSubCommands {
    pub(super) async fn show(&self, format: Format) -> anyhow::Result<()> {
        match self {
            ShowSubCommands::Contract(options) => show_contract(&options.path, format).await,
            ShowSubCommands::Account(cmds) => cmds.show(),
        }
    }
}
