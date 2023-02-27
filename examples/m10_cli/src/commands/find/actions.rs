use crate::context::Context;

use clap::Parser;
use m10_sdk::{account::AccountId, ActionsFilter, Format, PrettyPrint, TxnFilter};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindActionOptions {
    /// Action name
    #[clap(short, long, default_value_t)]
    name: String,
    /// Set minimum tx id
    #[clap(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[clap(short = 'x', long, default_value = "u64::MAX")]
    max_tx_id: u64,
    /// Set account filter
    #[clap(short, long)]
    account: Option<AccountId>,
    /// Set limit
    #[clap(short, long, default_value = "20")]
    limit: u64,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindActionOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        let filter = if let Some(account_id) = self.account {
            TxnFilter::<ActionsFilter>::by_account(self.name.clone(), account_id)
        } else {
            TxnFilter::<ActionsFilter>::by_context_id(
                self.name.clone(),
                hex::decode(&config.context_id)?,
            )
        }
        .limit(self.limit)
        .min_tx(self.min_tx_id)
        .max_tx(self.max_tx_id);
        context
            .m10_client
            .list_actions(filter)
            .await?
            .print(self.format)?;
        Ok(())
    }
}
