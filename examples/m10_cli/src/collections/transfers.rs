use m10_sdk::contract::{FinalizedContractExt, TransferInfo};
use m10_sdk::prost::{Any, Message};
use m10_sdk::sdk::{self, commit_transfer};
use m10_sdk::TransferStep;
use std::convert::{TryFrom, TryInto};

const ATTACHMENT_TYPE_URL: &str = "m10.sdk.metadata.Attachment";
const MEMO_TYPE_URL: &str = "m10.sdk.metadata.Memo";
const FEE_TYPE_URL: &str = "m10.sdk.metadata.Fee";
const WITHDRAW_TYPE_URL: &str = "m10.sdk.metadata.Withdraw";
const DEPOSIT_TYPE_URL: &str = "m10.sdk.metadata.Deposit";
const CONTRACT_TYPE_URL: &str = "m10.sdk.metadata.Contract";

#[derive(Debug, serde::Serialize)]
pub(crate) struct Attachment {
    object_id: String,
}

impl TryFrom<sdk::Attachment> for Attachment {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Attachment) -> Result<Attachment, Self::Error> {
        let sdk::Attachment { object_id, .. } = other;
        Ok(Attachment { object_id })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Memo {
    plaintext: String,
}

impl TryFrom<sdk::Memo> for Memo {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Memo) -> Result<Memo, Self::Error> {
        let sdk::Memo { plaintext } = other;
        Ok(Memo { plaintext })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Fee {}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Withdraw {
    bank_account_id: String,
}

impl TryFrom<sdk::Withdraw> for Withdraw {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Withdraw) -> Result<Withdraw, Self::Error> {
        let sdk::Withdraw { bank_account_id } = other;
        Ok(Withdraw { bank_account_id })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Deposit {
    bank_account_id: String,
}

impl TryFrom<sdk::Deposit> for Deposit {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Deposit) -> Result<Deposit, Self::Error> {
        let sdk::Deposit { bank_account_id } = other;
        Ok(Deposit { bank_account_id })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Contract {
    transfers: Vec<TransferInfo>,
    endorsements: Vec<super::contracts::Endorsement>,
}
impl TryFrom<sdk::Contract> for Contract {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Contract) -> Result<Contract, Self::Error> {
        Ok(Contract {
            transfers: other.transfer_info()?,
            endorsements: other
                .endorsements
                .into_iter()
                .map(super::contracts::Endorsement::try_from)
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) enum Metadata {
    Attachment(Attachment),
    Memo(Memo),
    Fee(Fee),
    Withdraw(Withdraw),
    Deposit(Deposit),
    Contract(Contract),
    Unknown,
}

impl TryFrom<Any> for Metadata {
    type Error = anyhow::Error;

    fn try_from(other: Any) -> Result<Metadata, Self::Error> {
        Ok(match other.type_url.as_str() {
            ATTACHMENT_TYPE_URL => {
                Metadata::Attachment(sdk::Attachment::decode(other.value.as_slice())?.try_into()?)
            }
            MEMO_TYPE_URL => Metadata::Memo(sdk::Memo::decode(other.value.as_slice())?.try_into()?),
            FEE_TYPE_URL => Metadata::Fee(Fee {}),
            WITHDRAW_TYPE_URL => {
                Metadata::Withdraw(sdk::Withdraw::decode(other.value.as_slice())?.try_into()?)
            }
            DEPOSIT_TYPE_URL => {
                Metadata::Deposit(sdk::Deposit::decode(other.value.as_slice())?.try_into()?)
            }
            CONTRACT_TYPE_URL => {
                Metadata::Contract(sdk::Contract::decode(other.value.as_slice())?.try_into()?)
            }
            _ => Metadata::Unknown,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct CreateTransfer {
    pub(crate) steps: Vec<TransferStep>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct CommitTransfer {
    pub(crate) pending_tx_id: u64,
    pub(crate) new_state: commit_transfer::TransferState,
}

impl TryFrom<sdk::CommitTransfer> for CommitTransfer {
    type Error = anyhow::Error;

    fn try_from(value: sdk::CommitTransfer) -> Result<Self, Self::Error> {
        Ok(CommitTransfer {
            pending_tx_id: value.pending_tx_id,
            new_state: value.new_state(),
        })
    }
}
