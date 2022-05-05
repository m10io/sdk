use m10_sdk::account::AccountId;
use m10_sdk::sdk;
use std::convert::TryFrom;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Action {
    pub(crate) tx_id: u64,
    pub(crate) name: String,
    pub(crate) from_account_id: String,
    pub(crate) target: Target,
    pub(crate) context_id: Vec<u8>,
    pub(crate) payload: Vec<u8>,
}

impl TryFrom<sdk::Action> for Action {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Action) -> Result<Action, Self::Error> {
        let sdk::Action {
            tx_id,
            name,
            context_id,
            from_account,
            target,
            payload,
        } = other;
        Ok(Action {
            tx_id,
            name,
            from_account_id: hex::encode(&from_account),
            target: Target::try_from(
                target.ok_or_else(|| anyhow::anyhow!("No target specified"))?,
            )?,
            context_id,
            payload,
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) enum Target {
    AccountId(AccountId),
}

impl TryFrom<sdk::Target> for Target {
    type Error = anyhow::Error;

    fn try_from(target: sdk::Target) -> Result<Self, Self::Error> {
        Ok(
            match target
                .target
                .ok_or_else(|| anyhow::anyhow!("No target specified"))?
            {
                sdk::target::Target::AccountId(id) => {
                    Target::AccountId(AccountId::try_from_be_slice(&id)?)
                }
            },
        )
    }
}
