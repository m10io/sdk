use clap::Parser;
use m10_sdk::sdk;
use m10_sdk::DocumentUpdate;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateAccountSetOptions {
    /// Record id
    pub(super) id: Uuid,
    /// Add accounts
    #[clap(short, long, multiple_values = true)]
    accounts: Option<Vec<sdk::AccountRef>>,
    /// Update owner field
    #[clap(short, long)]
    owner: Option<String>,
}

impl super::BuildFromOptions for UpdateAccountSetOptions {
    type Document = sdk::AccountSet;

    fn build_from_options(
        &self,
        builder: &mut DocumentUpdate<Self::Document>,
    ) -> anyhow::Result<()> {
        if let Some(accounts) = &self.accounts {
            builder.accounts(accounts.clone());
        }
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(owner_key);
        }
        Ok(())
    }
}
