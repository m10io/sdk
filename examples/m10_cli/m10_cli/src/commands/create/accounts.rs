use super::ledger_accounts::CreateLedgerAccountOptions;
use clap::Parser;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateAccountOptions {
    /// Ignore error if item exists
    #[clap(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set an account id
    #[clap(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set owner of the account record
    #[clap(short, long)]
    owner: Option<String>,
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

impl super::BuildFromOptions for CreateAccountOptions {
    type Document = sdk::Account;
    fn build_from_options(&self, default_owner: Vec<u8>) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self
            .owner
            .as_ref()
            .map_or::<Result<Vec<u8>, _>, _>(Ok(default_owner), base64::decode)?;
        Ok(sdk::Account {
            id,
            owner,
            name: self.name.clone().unwrap_or_default(),
            public_name: self.public_name.clone().unwrap_or_default(),
            profile_image_url: self.profile_image_url.clone().unwrap_or_default(),
        })
    }
}

impl From<&CreateLedgerAccountOptions> for CreateAccountOptions {
    fn from(other: &CreateLedgerAccountOptions) -> CreateAccountOptions {
        let CreateLedgerAccountOptions {
            owner,
            name,
            public_name,
            profile_image_url,
            ..
        } = other;
        CreateAccountOptions {
            if_not_exists: false,
            id: None,
            owner: owner.clone(),
            name: name.clone(),
            public_name: public_name.clone(),
            profile_image_url: profile_image_url.clone(),
        }
    }
}
