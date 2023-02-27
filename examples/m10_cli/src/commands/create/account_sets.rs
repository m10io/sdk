use clap::Parser;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateAccountSetOptions {
    /// Ignore error if item exists
    #[clap(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[clap(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set account references
    #[clap(short, long, multiple_values = true)]
    #[serde(default)]
    accounts: Vec<sdk::AccountRef>,
    /// Set owner of the account set record
    #[clap(short, long)]
    owner: Option<PublicKey>,
}

impl super::BuildFromOptions for CreateAccountSetOptions {
    type Document = sdk::AccountSet;
    fn build_from_options(
        &self,
        default_owner: PublicKey,
    ) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.clone().unwrap_or(default_owner).0;
        Ok(sdk::AccountSet {
            id,
            owner,
            accounts: self.accounts.clone(),
        })
    }
}
