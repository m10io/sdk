use clap::Parser;
use m10_sdk::{account::AccountId, sdk, PublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct CreateAccountSetArgs {
    /// Ignore error if item exists
    #[arg(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[arg(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set account references
    #[arg(short, long)]
    #[serde(default)]
    accounts: Vec<AccountId>,
    /// Set owner of the account set record
    #[arg(short, long)]
    owner: Option<PublicKey>,
}

impl super::BuildFromArgs for CreateAccountSetArgs {
    type Document = sdk::AccountSet;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.clone().unwrap_or(default_owner).0;
        Ok(sdk::AccountSet {
            id,
            owner,
            accounts: self
                .accounts
                .into_iter()
                .map(|account_id| account_id.to_vec())
                .collect(),
        })
    }
}
