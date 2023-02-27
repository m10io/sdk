use clap::Parser;
use m10_sdk::sdk;
use m10_sdk::DocumentUpdate;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateAccountOptions {
    /// Record id
    pub(super) id: Uuid,
    /// Update owner field
    #[clap(short, long)]
    owner: Option<String>,
    /// Update account name
    #[clap(short, long)]
    name: Option<String>,
    /// Update sender name
    #[clap(short, long)]
    public_name: Option<String>,
    /// Update profile image url
    #[clap(long)]
    profile_image_url: Option<String>,
}

impl super::BuildFromOptions for UpdateAccountOptions {
    type Document = sdk::AccountMetadata;

    fn build_from_options(
        &self,
        builder: &mut DocumentUpdate<Self::Document>,
    ) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(owner_key);
        }
        if let Some(name) = &self.name {
            builder.name(name.into());
        }
        if let Some(public_name) = &self.public_name {
            builder.public_name(public_name.into());
        }
        if let Some(profile_image_url) = &self.profile_image_url {
            builder.profile_image_url(profile_image_url.to_string());
        }

        Ok(())
    }
}
