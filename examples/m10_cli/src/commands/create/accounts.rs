use super::ledger_accounts::CreateLedgerAccountOptions;
use clap::Parser;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateAccountMetadataOptions {
    /// Ignore error if item exists
    #[clap(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set an account id
    #[clap(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set owner of the account record
    #[clap(short, long)]
    owner: Option<PublicKey>,
    /// Set an account name
    #[clap(short, long)]
    name: Option<String>,
    /// Set a name to be shown in transfers as sender
    #[clap(short, long)]
    public_name: Option<String>,
    /// Set profile image url
    #[clap(long)]
    profile_image_url: Option<String>,
}

impl super::BuildFromOptions for CreateAccountMetadataOptions {
    type Document = sdk::AccountMetadata;
    fn build_from_options(
        &self,
        default_owner: PublicKey,
    ) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.clone().unwrap_or(default_owner).0;
        Ok(sdk::AccountMetadata {
            id,
            owner,
            name: self.name.clone().unwrap_or_default(),
            public_name: self.public_name.clone().unwrap_or_default(),
            profile_image_url: self.profile_image_url.clone().unwrap_or_default(),
        })
    }
}

impl From<&CreateLedgerAccountOptions> for CreateAccountMetadataOptions {
    fn from(other: &CreateLedgerAccountOptions) -> CreateAccountMetadataOptions {
        let CreateLedgerAccountOptions {
            owner,
            name,
            public_name,
            profile_image_url,
            ..
        } = other;
        CreateAccountMetadataOptions {
            if_not_exists: false,
            id: None,
            owner: owner.clone(),
            name: name.clone(),
            public_name: public_name.clone(),
            profile_image_url: profile_image_url.clone(),
        }
    }
}
