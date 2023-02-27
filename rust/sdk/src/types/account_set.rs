use crate::account::{AccountId, AccountIdError};
use crate::collections::ResourceId;
use crate::error::M10Error;
use crate::types::PublicKey;
use m10_protos::sdk;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct AccountSet {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub accounts: Vec<AccountId>,
}

#[cfg(feature = "format")]
impl std::fmt::Display for AccountSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            id,
            owner,
            accounts,
        } = self;
        write!(f, "AccountSet{{ id={id} owner={owner} accounts=[",)?;
        for account in accounts {
            write!(f, "{account},")?;
        }
        write!(f, "] }}")
    }
}

impl TryFrom<sdk::AccountSet> for AccountSet {
    type Error = M10Error;

    fn try_from(set: sdk::AccountSet) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(set.id.as_ref())?,
            accounts: set
                .accounts
                .into_iter()
                .map(|a| AccountId::try_from_be_slice(&a.account_id))
                .collect::<Result<_, AccountIdError>>()?,
            owner: PublicKey(set.owner),
        })
    }
}
