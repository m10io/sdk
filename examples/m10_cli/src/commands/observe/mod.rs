use crate::commands::top_level_cmds::Format;
use clap::Parser;
use m10_sdk::account::AccountId;
use m10_sdk::sdk;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    io::{self, LineWriter},
};

mod accounts;
mod actions;
mod metrics;
mod transfers;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(super) struct ObserveAccountOptions {
    /// Account IDs
    #[clap(short, multiple_values = true)]
    ids: Vec<String>,
    /// Transaction ID to start observing from
    #[clap(short)]
    starting_from: Option<u64>,
    /// output format
    #[clap(short = 'f', long, default_value = "raw")]
    #[serde(default = "Format::default")]
    format: Format,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(super) struct ObserveActionOptions {
    /// Name of the action
    #[clap(short)]
    name: String,
    /// Account IDs
    #[clap(short, multiple_values = true)]
    ids: Vec<String>,
    /// Transaction ID to start observing from
    #[clap(short)]
    starting_from: Option<u64>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short = 'f', long, default_value = "raw")]
    #[serde(default = "Format::default")]
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
                transfers::observe(
                    &options.ids,
                    options.starting_from.map(|tx_id| sdk::TxId { tx_id }),
                    options.format,
                    config,
                )
                .await
            }
            ObserveSubcommands::Actions(options) => {
                actions::observe(
                    &options.name,
                    &options.ids,
                    options.starting_from.map(|tx_id| sdk::TxId { tx_id }),
                    options.format,
                    config,
                )
                .await
            }
            ObserveSubcommands::Accounts(options) => {
                accounts::observe(
                    &options.ids,
                    options.starting_from.map(|tx_id| sdk::TxId { tx_id }),
                    options.format,
                    config,
                )
                .await
            }
            ObserveSubcommands::Metrics(options) => {
                metrics::observe(
                    &options.ids,
                    options.starting_from.map(|tx_id| sdk::TxId { tx_id }),
                    options.format,
                    config,
                )
                .await
            }
        }
    }
}

fn print_doc(item: impl Serialize, format: Format) -> anyhow::Result<()> {
    match format {
        Format::Json => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let writer = LineWriter::new(handle);
            serde_json::to_writer_pretty(writer, &item)?;
        }
        Format::Yaml => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let writer = LineWriter::new(handle);
            serde_yaml::to_writer(writer, &item)?;
        }
        Format::Raw => {
            let pretty = PrettyConfig::new()
                .with_depth_limit(4)
                .with_separate_tuple_members(true);
            let s = to_string_pretty(&item, pretty)?;
            println!("{}", s);
        }
    }
    Ok(())
}

fn parse_account_id(val: &str) -> anyhow::Result<AccountId> {
    let bytes = hex::decode(&val)?;
    let id = AccountId::try_from_be_slice(&bytes)?;
    Ok(id)
}
