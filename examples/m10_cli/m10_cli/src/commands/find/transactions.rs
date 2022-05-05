use crate::collections::tx::Tx;
use crate::context::Context;
use crate::utils::print_items;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindTransactionOptions {
    /// Set minimum tx id
    #[clap(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[clap(short = 'x', long)]
    max_tx_id: Option<u64>,
    /// Set limit
    #[clap(short, long, default_value = "20")]
    limit: u64,
    /// Set group filter
    #[clap(short, long)]
    account: Option<String>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindTransactionOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let max_tx_id = self.max_tx_id.unwrap_or(u64::MAX);
        if let Some(account) = &self.account {
            let account_id = hex::decode(account)?;
            let request = context
                .admin
                .sign_request(sdk::GroupTransactionsRequest {
                    account_id,
                    min_tx_id: self.min_tx_id,
                    max_tx_id,
                    limit_groups: self.limit,
                })
                .await?;
            let mut txn_groups = context.m10_client.group_transactions(request).await?;
            let mut groups = vec![];
            for group in txn_groups.groups.drain(..) {
                let items = group
                    .transactions
                    .into_iter()
                    .map(Tx::try_from)
                    .collect::<anyhow::Result<Vec<Tx>>>()?;
                groups.push(items);
            }
            print_items(groups, self.format)?;
        } else {
            let context_id = hex::decode(config.context_id.clone())?;
            let request = context
                .admin
                .sign_request(sdk::ListTransactionsRequest {
                    context_id,
                    min_tx_id: self.min_tx_id,
                    max_tx_id,
                    limit: self.limit,
                })
                .await?;
            let txs = context.m10_client.list_transactions(request).await?;

            let items = txs
                .transactions
                .into_iter()
                .map(Tx::try_from)
                .collect::<anyhow::Result<_>>()?;
            print_items(items, self.format)?;
        }
        Ok(())
    }
}
