use crate::context::Context;

use clap::ArgGroup;
use clap::Parser;
use m10_sdk::account::AccountId;
use m10_sdk::TransferFilter;
use m10_sdk::TxnFilter;
use m10_sdk::{self, Format, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("filter").required(true), about)]
pub(crate) struct FindTransferOptions {
    /// Set minimum tx id
    #[clap(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[clap(short = 'x', long)]
    max_tx_id: Option<u64>,
    /// Set account filter
    #[clap(short, long, group = "filter")]
    account: Option<AccountId>,
    /// Set contextID filter
    #[clap(short, long, group = "filter")]
    context_id: Option<String>,
    /// Set limit
    #[clap(short, long, default_value = "20")]
    limit: u64,
    /// Include child accounts in result
    #[clap(short, long)]
    include_child_accounts: bool,
    /// Set enhanced result
    #[clap(short, long)]
    enhanced: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindTransferOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        let max_tx_id = self.max_tx_id.unwrap_or(u64::MAX);

        let filter = if let Some(account) = self.account {
            TxnFilter::<TransferFilter>::by_account(account)
        } else if let Some(context_id) = &self.context_id {
            TxnFilter::<TransferFilter>::by_context_id(hex::decode(context_id)?)
        } else {
            anyhow::bail!("Missing account or context_id filter")
        }
        .min_tx(self.min_tx_id)
        .max_tx(max_tx_id)
        .limit(self.limit)
        .include_child_accounts(self.include_child_accounts);

        if self.enhanced {
            context
                .m10_client
                .get_enhanced_transfers(filter)
                .await?
                .print(self.format)?;
        } else {
            context
                .m10_client
                .list_transfers(filter)
                .await?
                .print(self.format)?;
        }
        Ok(())
    }
}
