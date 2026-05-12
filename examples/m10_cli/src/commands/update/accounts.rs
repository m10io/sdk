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
    /// Update ISIN
    #[arg(long)]
    isin: Option<String>,
    /// Update DTI
    #[arg(long)]
    dti: Option<String>,
    /// Update Issuer Bank Id
    #[arg(long)]
    issuer_bank_id: Option<String>,
}

impl super::BuildFromArgs for UpdateAccountArgs {
    type Document = sdk::AccountMetadata;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<bool> {
        let changed = self.owner.is_some()
            || self.name.is_some()
            || self.public_name.is_some()
            || self.profile_image_url.is_some()
            || self.isin.is_some()
            || self.dti.is_some()
            || self.issuer_bank_id.is_some();

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
        if let Some(isin) = self.isin {
            builder.isin(isin);
        }
        if let Some(dti) = self.dti {
            builder.dti(dti);
        }
        if let Some(issuer_bank_id) = self.issuer_bank_id {
            builder.issuer_bank_id(issuer_bank_id);
        }
        Ok(changed)
    }
}
