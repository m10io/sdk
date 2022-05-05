use crate::context::Context;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetLedgerAccountOptions {
    /// Account id
    id: String,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl GetLedgerAccountOptions {
    pub(super) async fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let id = hex::decode(&self.id)?;
        let request = context
            .admin
            .sign_request(sdk::GetAccountRequest { id })
            .await?;
        let indexed_account = context.m10_client.get_indexed_account(request).await?;
        let item = crate::collections::accounts::IndexedAccount::try_from(indexed_account)?;
        super::print_item(item, self.format)?;
        Ok(())
    }
}
