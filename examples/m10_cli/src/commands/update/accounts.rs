use clap::Args;
use m10_sdk::{sdk, DocumentUpdate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateAccountArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
    /// Update account name
    #[arg(short, long)]
    name: Option<String>,
    /// Update sender name
    #[arg(long, alias = "pn")]
    public_name: Option<String>,
    /// Update profile image url
    #[arg(long, aliases = ["image", "pi"])]
    profile_image_url: Option<String>,
}

impl super::BuildFromArgs for UpdateAccountArgs {
    type Document = sdk::AccountMetadata;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(owner_key);
        }
        if let Some(name) = self.name {
            builder.name(name);
        }
        if let Some(public_name) = self.public_name {
            builder.public_name(public_name);
        }
        if let Some(profile_image_url) = self.profile_image_url {
            builder.profile_image_url(profile_image_url);
        }

        Ok(())
    }
}
