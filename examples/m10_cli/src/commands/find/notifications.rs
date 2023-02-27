use clap::Parser;
use m10_sdk::protos;
use m10_sdk::DocumentUpdate;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindNotificationPreferencesOptions {
    /// Set platform filter
    #[clap(short, long)]
    platform: Option<String>,
    /// Set address filter
    #[clap(short, long)]
    address: Option<String>,
    /// Set notification setting filter
    #[clap(short, long)]
    no_notifications: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    pub(super) format: super::Format,
}
