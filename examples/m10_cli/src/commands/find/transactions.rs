use clap::Args;
use m10_sdk::{account::AccountId, ContextFilter, Format, GroupingFilter, PrettyPrint, TxnFilter};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct FindTransactionArgs {
    /// Set minimum tx id
    #[arg(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[arg(short = 'x', long, default_value = "u64::MAX")]
    max_tx_id: u64,
    /// Set limit
    #[arg(short, long, default_value = "20")]
    limit: u64,
    /// Set group filter
    #[arg(short, long)]
    account: Option<AccountId>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindTransactionArgs {
    pub(crate) async fn find(&self, context: &Context) -> anyhow::Result<()> {
        if let Some(account) = self.account {
            context
                .ledger_client()
                .group_transactions(
                    TxnFilter::<GroupingFilter>::account(account)
                        .min_tx(self.min_tx_id)
                        .max_tx(self.max_tx_id)
                        .limit(self.limit),
                )
                .await?
                .print(self.format)?;
        } else {
            let context_id = hex::decode(context.context_id())?;
            context
                .ledger_client()
                .list_transactions(TxnFilter::<ContextFilter>::context_id(context_id))
                .await?
                .print(self.format)?;
        }
        Ok(())
    }
}
