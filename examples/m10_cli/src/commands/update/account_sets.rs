use clap::Args;
use m10_sdk::{account::AccountId, sdk, DocumentUpdate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateAccountSetArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Add accounts
    #[arg(short, long)]
    accounts: Option<Vec<AccountId>>,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
}

impl super::BuildFromArgs for UpdateAccountSetArgs {
    type Document = sdk::AccountSet;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<()> {
        if let Some(accounts) = self.accounts {
            builder.accounts(accounts);
        }
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(owner_key);
        }
        Ok(())
    }
}
