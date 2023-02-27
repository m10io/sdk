use crate::context::Context;

use clap::Parser;
use m10_sdk::{account::AccountId, Format, GroupingFilter, TxnFilter};
use m10_sdk::{ContextFilter, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindTransactionOptions {
    /// Set minimum tx id
    #[clap(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[clap(short = 'x', long, default_value = "u64::MAX")]
    max_tx_id: u64,
    /// Set limit
    #[clap(short, long, default_value = "20")]
    limit: u64,
    /// Set group filter
    #[clap(short, long)]
    account: Option<AccountId>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindTransactionOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        if let Some(account) = self.account {
            context
                .m10_client
                .group_transactions(
                    TxnFilter::<GroupingFilter>::account(account)
                        .min_tx(self.min_tx_id)
                        .max_tx(self.max_tx_id)
                        .limit(self.limit),
                )
                .await?
                .print(self.format)?;
        } else {
            let context_id = hex::decode(config.context_id.clone())?;
            context
                .m10_client
                .list_transactions(TxnFilter::<ContextFilter>::context_id(context_id))
                .await?
                .print(self.format)?;
        }
        Ok(())
    }
}
