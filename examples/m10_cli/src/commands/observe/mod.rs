use clap::Parser;
use m10_sdk::account::AccountId;
use m10_sdk::Format;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod accounts;
mod actions;
mod metrics;
mod transfers;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(super) struct ObserveAccountOptions {
    /// Account IDs
    #[clap(short, multiple_values = true)]
    ids: Vec<AccountId>,
    /// Transaction ID to start observing from
    #[clap(short, long)]
    starting_from: Option<u64>,
    /// output format
    #[clap(short = 'f', long, default_value_t)]
    format: Format,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(super) struct ObserveActionOptions {
    /// Name of the action
    #[clap(short, long)]
    name: String,
    /// Account IDs
    #[clap(short, long, multiple_values = true)]
    ids: Vec<AccountId>,
    /// Transaction ID to start observing from
    #[clap(short, long)]
    starting_from: Option<u64>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short = 'f', long, default_value_t)]
    format: Format,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ObserveSubcommands {
    #[clap(about = "Observe transfers involving the provided account ids")]
    Transfers(ObserveAccountOptions),
    #[clap(about = "Observe an action involving the provided account ids")]
    Actions(ObserveActionOptions),
    #[clap(about = "Observe account updates involves the provided account ids")]
    Accounts(ObserveAccountOptions),
    #[clap(about = "Observe transaction metrics involving the provided account ids")]
    Metrics(ObserveAccountOptions),
}

impl ObserveSubcommands {
    pub(super) async fn observe(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            ObserveSubcommands::Transfers(options) => {
                transfers::observe(&options.ids, options.starting_from, options.format, config)
                    .await
            }
            ObserveSubcommands::Actions(options) => {
                actions::observe(
                    &options.name,
                    &options.ids,
                    options.starting_from,
                    options.format,
                    config,
                )
                .await
            }
            ObserveSubcommands::Accounts(options) => {
                accounts::observe(&options.ids, options.starting_from, options.format, config).await
            }
            ObserveSubcommands::Metrics(options) => {
                metrics::observe(&options.ids, options.starting_from, options.format, config).await
            }
        }
    }
}
