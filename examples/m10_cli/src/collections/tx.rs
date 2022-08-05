use crate::collections::accounts::{AccountCreated, SetBalanceLimit, SetFrozen, SetInstrument};
use crate::collections::actions::Target;
use crate::collections::transfers::CommitTransfer;
use crate::collections::transfers::CreateTransfer;
use chrono::NaiveDateTime;
use m10_sdk::sdk::{self, transaction_data::Data};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, serde::Serialize)]
pub(crate) struct Tx {
    pub(crate) tx_id: u64,
    pub(crate) timestamp: String,
    pub(crate) data: RequestData,
    pub(crate) context_id: Option<String>,
    pub(crate) error: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) enum RequestData {
    Transfer(CreateTransfer),
    CreateAccount(AccountCreated),
    SetFrozen(SetFrozen),
    DocumentOperations,
    Action(InvokeAction),
    InitiateTransfer(CreateTransfer),
    CommitTransfer(CommitTransfer),
    SetInstrument(SetInstrument),
    SetBalanceLimit(SetBalanceLimit),
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct InvokeAction {
    pub(crate) name: String,
    pub(crate) from_account_id: String,
    pub(crate) target: Target,
    pub(crate) payload: Vec<u8>,
}

impl TryFrom<sdk::FinalizedTransaction> for Tx {
    type Error = anyhow::Error;

    fn try_from(other: sdk::FinalizedTransaction) -> Result<Tx, Self::Error> {
        let sdk::FinalizedTransaction { request, response } = other;
        let sdk::TransactionRequestPayload {
            context_id, data, ..
        } = request.ok_or_else(|| anyhow::anyhow!("Missing transaction"))?;
        let sdk::TransactionResponse {
            tx_id,
            account_created,
            timestamp,
            error,
            ..
        } = response.ok_or_else(|| anyhow::anyhow!("Missing result"))?;
        let data = data
            .and_then(|d| d.data)
            .ok_or_else(|| anyhow::anyhow!("Missing tx data"))?;

        let data = match data {
            Data::Transfer(transfer) => RequestData::Transfer(CreateTransfer::try_from(transfer)?),
            Data::CreateLedgerAccount(account) => RequestData::CreateAccount(AccountCreated {
                account_id: hex::encode(&account_created),
                is_frozen: account.frozen,
            }),
            Data::SetFreezeState(frozen) => RequestData::SetFrozen(SetFrozen {
                account_id: hex::encode(&frozen.account_id),
                is_frozen: frozen.frozen,
            }),
            Data::DocumentOperations(_) => RequestData::DocumentOperations,
            Data::InvokeAction(action) => RequestData::Action(InvokeAction::try_from(action)?),

            //TODO: Will complete with https://m10.atlassian.net/browse/M10-2673
            Data::InitiateTransfer(transfer) => RequestData::InitiateTransfer(transfer.try_into()?),
            Data::CommitTransfer(commit) => RequestData::CommitTransfer(commit.try_into()?),
            Data::SetInstrument(instrument) => RequestData::SetInstrument(SetInstrument {
                account_id: hex::encode(&instrument.account_id),
                code: instrument.code,
                decimals: instrument.decimal_places,
                description: instrument.description,
            }),
            Data::SetBalanceLimit(limit) => RequestData::SetBalanceLimit(SetBalanceLimit {
                account_id: hex::encode(&limit.account_id),
                balance_limit: limit.balance_limit,
            }),
        };

        Ok(Self {
            tx_id,
            timestamp: NaiveDateTime::from_timestamp(
                (timestamp / 1_000_000) as i64,
                ((timestamp % 1_000_000) * 1000) as u32,
            )
            .to_string(),
            data,
            context_id: context_id.is_empty().then(|| hex::encode(&context_id)),
            error: error.map(|err| format!("{} ({:?})", err.message, err.code)),
        })
    }
}

impl TryFrom<sdk::InvokeAction> for InvokeAction {
    type Error = anyhow::Error;

    fn try_from(other: sdk::InvokeAction) -> Result<Self, Self::Error> {
        let sdk::InvokeAction {
            name,
            from_account,
            target,
            payload,
        } = other;
        Ok(InvokeAction {
            name,
            from_account_id: hex::encode(&from_account),
            target: Target::try_from(
                target.ok_or_else(|| anyhow::anyhow!("No target specified"))?,
            )?,
            payload,
        })
    }
}
