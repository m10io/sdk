use clap::Args;
use m10_sdk::{account::AccountId, ActionsFilter, Format, PrettyPrint, TxnFilter};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct FindActionArgs {
    /// Action name
    #[arg(short, long, default_value_t)]
    name: String,
    /// Set minimum tx id
    #[arg(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[arg(short = 'x', long, default_value_t = u64::MAX)]
    max_tx_id: u64,
    /// Set ledger account filter
    #[arg(short, long)]
    account: Option<AccountId>,
    /// Set limit
    #[arg(short, long, default_value = "20")]
    limit: u64,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindActionArgs {
    pub(crate) async fn find(&self, context: &Context) -> anyhow::Result<()> {
        let filter = if let Some(account_id) = self.account {
            TxnFilter::<ActionsFilter>::by_account(self.name.clone(), account_id)
        } else {
            TxnFilter::<ActionsFilter>::by_context_id(
                self.name.clone(),
                hex::decode(context.context_id())?,
            )
        }
        .limit(self.limit)
        .min_tx(self.min_tx_id)
        .max_tx(self.max_tx_id);
        context
            .ledger_client()
            .list_actions(filter)
            .await?
            .print(self.format)?;
        Ok(())
    }
}
