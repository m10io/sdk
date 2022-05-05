use crate::collections::transfers::{AccountInfo, EnhancedTransfer, Transfer};
use crate::context::Context;
use crate::utils::print_items;
use clap::Parser;
use m10_sdk::{self, sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindTransferOptions {
    /// Set minimum tx id
    #[clap(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[clap(short = 'x', long)]
    max_tx_id: Option<u64>,
    /// Set account filter
    #[clap(short, long)]
    account: String,
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
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindTransferOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let filter = sdk::list_transfer_request::Filter::AccountId(hex::decode(&self.account)?);
        let max_tx_id = self.max_tx_id.unwrap_or(u64::MAX);
        let request = sdk::ListTransferRequest {
            filter: Some(filter),
            min_tx_id: self.min_tx_id,
            max_tx_id,
            limit: self.limit,
            include_child_accounts: self.include_child_accounts,
        };
        if self.enhanced {
            self.list_enhanced(request, &mut context).await
        } else {
            self.list(request, &mut context).await
        }
    }

    async fn list(
        &self,
        request: sdk::ListTransferRequest,
        context: &mut Context,
    ) -> anyhow::Result<()> {
        let request = context.admin.sign_request(request).await?;
        let txs = context.m10_client.list_transfers(request).await?;

        let items = txs
            .transfers
            .into_iter()
            .map(Transfer::try_from)
            .collect::<Result<_, anyhow::Error>>()?;
        print_items(items, self.format)?;
        Ok(())
    }

    async fn list_enhanced(
        &self,
        request: sdk::ListTransferRequest,
        context: &mut Context,
    ) -> anyhow::Result<()> {
        let request = context.admin.sign_request(request).await?;
        let transfers = context.m10_client.list_transfers(request).await?.transfers;

        let mut items = vec![];
        for transfer in transfers {
            let m10_sdk::EnhancedTransfer {
                transfer,
                mut enhanced_steps,
            } = context
                .m10_client
                .enhance_transfer(transfer, &context.admin)
                .await?;
            let m10_sdk::EnhancedTransferStep { from, to, .. } = enhanced_steps.swap_remove(0);
            items.push(EnhancedTransfer {
                transfer: Transfer::try_from(transfer)?,
                from: from.map(AccountInfo::try_from).transpose()?,
                to: to.map(AccountInfo::try_from).transpose()?,
            })
        }

        print_items(items, self.format)?;
        Ok(())
    }
}
