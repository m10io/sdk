use m10_sdk::{account::AccountId, sdk};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AccountSet {
    pub id: Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountId>,
}

impl TryFrom<sdk::AccountSet> for AccountSet {
    type Error = anyhow::Error;

    fn try_from(other: sdk::AccountSet) -> Result<AccountSet, Self::Error> {
        let sdk::AccountSet {
            id,
            owner,
            accounts,
            ..
        } = other;
        Ok(AccountSet {
            id: Uuid::from_slice(&id).unwrap_or_default(),
            owner: base64::encode(owner),
            accounts: accounts
                .into_iter()
                .map(|account_id| AccountId::try_from_be_slice(&account_id))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .collect(),
        })
    }
}
