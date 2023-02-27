use clap::Parser;
use m10_sdk::Format;
use serde::{Deserialize, Serialize};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetActionOptions {
    /// The transaction id
    pub(crate) id: u64,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    pub(crate) format: Format,
}
