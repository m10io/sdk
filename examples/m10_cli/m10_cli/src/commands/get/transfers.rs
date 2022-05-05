use crate::collections::transfers::{AccountInfo, EnhancedTransfer, Transfer};
use crate::context::Context;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetTransferOptions {
    /// The transaction id
    id: u64,
    /// Set enhanced result
    #[clap(short, long)]
    enhanced: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl GetTransferOptions {
    pub(super) async fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let request = context
            .admin
            .sign_request(sdk::GetTransferRequest { tx_id: self.id })
            .await?;
        if self.enhanced {
            let transfer = context.m10_client.get_transfer(request).await?;
            let m10_sdk::EnhancedTransfer {
                transfer,
                mut enhanced_steps,
            } = context
                .m10_client
                .enhance_transfer(transfer, &context.admin)
                .await?;
            let m10_sdk::EnhancedTransferStep { from, to, .. } = enhanced_steps.swap_remove(0);
            super::print_item(
                EnhancedTransfer {
                    transfer: Transfer::try_from(transfer)?,
                    from: from.map(AccountInfo::try_from).transpose()?,
                    to: to.map(AccountInfo::try_from).transpose()?,
                },
                self.format,
            )?;
        } else {
            let tx = context.m10_client.get_transfer(request).await?;
            super::print_item(
                crate::collections::transfers::Transfer::try_from(tx)?,
                self.format,
            )?;
        }
        Ok(())
    }
}
