use chrono::NaiveDateTime;
use m10_sdk::account::AccountId;
use m10_sdk::contract::{FinalizedContractExt, TransferInfo};
use m10_sdk::prost::{Any, Message};
use m10_sdk::sdk::{self, commit_transfer, finalized_transfer::TransferState};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, serde::Serialize)]
pub(crate) struct AccountInfo {
    pub(crate) account_id: AccountId,
    pub(crate) parent_account_id: Option<AccountId>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) public_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) profile_image_url: String,
    pub(crate) code: String,
    pub(crate) decimal_places: u32,
}

impl TryFrom<sdk::AccountInfo> for AccountInfo {
    type Error = anyhow::Error;

    fn try_from(other: sdk::AccountInfo) -> Result<AccountInfo, Self::Error> {
        let sdk::AccountInfo {
            account_id,
            parent_account_id,
            public_name,
            profile_image_url,
            code,
            decimal_places,
        } = other;
        Ok(AccountInfo {
            account_id: AccountId::try_from_be_slice(&account_id)?,
            parent_account_id: if parent_account_id.is_empty() {
                None
            } else {
                Some(AccountId::try_from_be_slice(&parent_account_id)?)
            },
            public_name,
            profile_image_url,
            code,
            decimal_places,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct EnhancedTransfer {
    #[serde(flatten)]
    pub(crate) transfer: Transfer,
    pub(crate) from: Option<AccountInfo>,
    pub(crate) to: Option<AccountInfo>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Transfer {
    pub(crate) tx_id: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) context_id: Vec<u8>,
    pub(crate) timestamp: String,
    pub(crate) failed: bool,
    pub(crate) steps: Vec<TransferStep>,
    pub(crate) state: TransferState,
}

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
pub(crate) struct TransferStep {
    pub(crate) from_account_id: String,
    pub(crate) to_account_id: String,
    pub(crate) amount: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) metadata: Vec<Metadata>,
}

impl TryFrom<sdk::FinalizedTransfer> for Transfer {
    type Error = anyhow::Error;

    fn try_from(other: sdk::FinalizedTransfer) -> Result<Transfer, Self::Error> {
        let state = other.state();
        let sdk::FinalizedTransfer {
            tx_id,
            context_id,
            transfer_steps,
            error,
            timestamp,
            ..
        } = other;
        Ok(Transfer {
            tx_id,
            context_id,
            steps: transfer_steps
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, Self::Error>>()?,
            failed: error.is_some(),
            timestamp: NaiveDateTime::from_timestamp(
                (timestamp / 1_000_000) as i64,
                ((timestamp % 1_000_000) * 1000) as u32,
            )
            .to_string(),
            state,
        })
    }
}

impl TryFrom<sdk::TransferStep> for TransferStep {
    type Error = anyhow::Error;

    fn try_from(other: sdk::TransferStep) -> Result<TransferStep, Self::Error> {
        let sdk::TransferStep {
            from_account_id,
            to_account_id,
            amount,
            metadata,
        } = other;
        Ok(TransferStep {
            from_account_id: hex::encode(from_account_id),
            to_account_id: hex::encode(to_account_id),
            amount,
            metadata: metadata
                .into_iter()
                .map(Metadata::try_from)
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct CreateTransfer {
    pub(crate) steps: Vec<TransferStep>,
}

impl TryFrom<sdk::CreateTransfer> for CreateTransfer {
    type Error = anyhow::Error;

    fn try_from(other: sdk::CreateTransfer) -> Result<CreateTransfer, Self::Error> {
        let sdk::CreateTransfer { transfer_steps } = other;
        Ok(CreateTransfer {
            steps: transfer_steps
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, Self::Error>>()?,
        })
    }
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
