use std::fmt::Debug;

use clap::Parser;
use serde::{Deserialize, Serialize};

mod contracts;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum EndorsementSubCommands {
    /// Endorse contracts
    Contract(contracts::EndorseContractOptions),
}

impl EndorsementSubCommands {
    pub(super) async fn endorse(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            EndorsementSubCommands::Contract(options) => options.endorse(config).await,
        }
    }
}
