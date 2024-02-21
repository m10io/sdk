use clap::Args;
use m10_sdk::{protos, DocumentUpdate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct FindNotificationPreferencesArgs {
    /// Set platform filter
    #[arg(short, long)]
    platform: Option<String>,
    /// Set address filter
    #[arg(short, long)]
    address: Option<String>,
    /// Set notification setting filter
    #[arg(short, long)]
    no_notifications: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    pub(super) format: super::Format,
}
