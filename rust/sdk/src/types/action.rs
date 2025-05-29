use crate::account::{AccountId, AccountIdError};
use crate::error::M10Error;
use crate::types::TxId;
use crate::TransactionExt;
use core::convert::{From, TryFrom};
use core::result::Result;
use m10_protos::sdk;
use m10_protos::sdk::transaction_data::Data;
use serde::Serialize;
use serde_with::serde_as;

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, Serialize)]
pub enum Target {
    Any,
    #[cfg_attr(feature = "format", display("Account({0})"))]
    Account(AccountId),
}

impl From<Target> for sdk::Target {
    fn from(target: Target) -> Self {
        let target = match target {
            Target::Any => sdk::target::Target::AnyAccount(()),
            Target::Account(id) => sdk::target::Target::AccountId(id.to_vec()),
        };
        Self {
            target: Some(target),
        }
    }
}

impl TryFrom<sdk::Target> for Target {
    type Error = M10Error;

    fn try_from(target: sdk::Target) -> Result<Self, Self::Error> {
        Ok(match target.target {
            Some(sdk::target::Target::AnyAccount(())) => Target::Any,
            Some(sdk::target::Target::AccountId(id)) => {
                Target::Account(AccountId::try_from_be_slice(&id)?)
            }
            None => return Err(M10Error::InvalidAccountId(AccountIdError::MissingRoot)),
        })
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize)]
pub struct Action {
    pub tx_id: TxId,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub context_id: Vec<u8>,
    pub name: String,
    pub from_account: AccountId,
    pub target: Target,
    pub payload: Vec<u8>,
    // @sadroeck TODO: Add timestamps to protobuf
    // pub timestamp: SystemTime,
}

#[cfg(feature = "format")]
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            tx_id,
            context_id,
            name,
            target,
            payload,
            ..
        } = self;
        write!(
            f,
            "Action{{ tx_id={tx_id} name={name} context={} target={target} payload={}",
            hex::encode(context_id),
            hex::encode(payload)
        )
    }
}

impl TryFrom<sdk::Action> for Action {
    type Error = M10Error;

    fn try_from(action: sdk::Action) -> Result<Self, Self::Error> {
        Ok(Self {
            tx_id: action.tx_id,
            context_id: action.context_id,
            // timestamp: UNIX_EPOCH + Duration::from_micros(action.timestamp),
            name: action.name,
            from_account: AccountId::try_from_be_slice(&action.from_account)?,
            target: Target::try_from(action.target.unwrap())?,
            payload: action.payload,
        })
    }
}

impl TryFrom<sdk::FinalizedTransaction> for Action {
    type Error = M10Error;

    fn try_from(txn: sdk::FinalizedTransaction) -> Result<Self, Self::Error> {
        let response = txn.response.as_ref().ok_or(M10Error::InvalidTransaction)?;
        let context_id = txn
            .request
            .as_ref()
            .ok_or(M10Error::InvalidTransaction)?
            .context_id
            .clone();
        let tx_id = response.tx_id;
        // let timestamp = UNIX_EPOCH + Duration::from_micros(response.timestamp);
        match txn.into_data().ok_or(M10Error::InvalidTransaction)? {
            Data::InvokeAction(action) => Ok(Self {
                tx_id,
                context_id,
                // timestamp,
                name: action.name,
                from_account: AccountId::try_from_be_slice(&action.from_account)?,
                target: Target::Any,
                payload: action.payload,
            }),
            _ => Err(M10Error::InvalidTransaction),
        }
    }
}
