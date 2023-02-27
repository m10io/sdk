use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetTransferOptions {
    /// The transaction id
    pub(crate) id: u64,
    /// Set enhanced result
    #[clap(short, long)]
    pub(crate) enhanced: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    pub(crate) format: super::Format,
}
