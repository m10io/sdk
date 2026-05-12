use m10_sdk::sdk;
use std::convert::TryFrom;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AccountMetadata {
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub public_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub profile_image_url: String,
}

impl TryFrom<sdk::AccountMetadata> for AccountMetadata {
    type Error = anyhow::Error;

    fn try_from(other: sdk::AccountMetadata) -> Result<AccountMetadata, Self::Error> {
        let sdk::AccountMetadata {
            id,
            owner,
            name,
            public_name,
            profile_image_url,
            ..
        } = other;
        Ok(AccountMetadata {
            id: hex::encode(id),
            owner: base64::encode(owner),
            name,
            public_name,
            profile_image_url,
        })
    }
}
