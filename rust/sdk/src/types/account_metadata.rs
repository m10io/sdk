use crate::collections::ResourceId;
use crate::error::M10Error;
use crate::types::PublicKey;
use m10_protos::sdk;

#[derive(Clone, Debug, serde::Serialize)]
pub struct AccountMetadata {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub profile_image_url: String,
    pub name: String,
    pub public_name: String,
    pub isin: String,
    pub dti: String,
    pub issuer_bank_id: String,
}

#[cfg(feature = "format")]
impl std::fmt::Display for AccountMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            id,
            owner,
            profile_image_url,
            name,
            public_name,
            isin,
            dti,
            issuer_bank_id,
        } = self;
        write!(
            f,
            "AccountSet{{ id={id} owner={owner} profile_image_url={profile_image_url} name={name} public_name={public_name} isin={isin} dti={dti} issuer_bank_id={issuer_bank_id}}}"
        )
    }
}

impl TryFrom<sdk::AccountMetadata> for AccountMetadata {
    type Error = M10Error;

    fn try_from(meta: sdk::AccountMetadata) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(meta.id.as_ref())?,
            owner: PublicKey(meta.owner),
            profile_image_url: meta.profile_image_url,
            name: meta.name,
            public_name: meta.public_name,
            isin: meta.isin,
            dti: meta.dti,
            issuer_bank_id: meta.issuer_bank_id,
        })
    }
}
