use clap::Args;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ledger_accounts::CreateLedgerAccountArgs;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateAccountMetadataArgs {
    /// Ignore error if item exists
    #[arg(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set an account-metada id. Use uuid
    #[arg(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set owner of the account-metadata record
    #[arg(short, long)]
    owner: Option<PublicKey>,
    /// Set an account-metadata name
    #[arg(short, long)]
    name: Option<String>,
    /// Set a name to be shown in transfers as Sender
    #[arg(long, alias = "pn")]
    public_name: Option<String>,
    /// Set profile image url
    #[arg(long, aliases = ["image", "pi"])]
    profile_image_url: Option<String>,
}

impl super::BuildFromArgs for CreateAccountMetadataArgs {
    type Document = sdk::AccountMetadata;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.unwrap_or(default_owner).0;
        Ok(sdk::AccountMetadata {
            id,
            owner,
            name: self.name.unwrap_or_default(),
            public_name: self.public_name.unwrap_or_default(),
            profile_image_url: self.profile_image_url.unwrap_or_default(),
        })
    }
}

impl From<&CreateLedgerAccountArgs> for CreateAccountMetadataArgs {
    fn from(other: &CreateLedgerAccountArgs) -> CreateAccountMetadataArgs {
        let CreateLedgerAccountArgs {
            owner,
            name,
            public_name,
            profile_image_url,
            ..
        } = other;
        CreateAccountMetadataArgs {
            if_not_exists: false,
            id: None,
            owner: owner.clone(),
            name: name.clone(),
            public_name: public_name.clone(),
            profile_image_url: profile_image_url.clone(),
        }
    }
}
