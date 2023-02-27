use crate::account::AccountId;
use crate::error::M10Error;
use crate::transaction_ext::TransactionExt;
use crate::{AccountUpdate, Action, Transfer, TxnFilter, DEFAULT_TXN_LIMIT};
use m10_protos::sdk;

#[derive(Debug, serde::Serialize)]
pub enum Transaction {
    Transfer(Transfer),
    InitiateTransfer(Transfer),
    CommitTransfer(Transfer),
    AccountUpdate(AccountUpdate),
    // TODO @sadroeck - fixme
    DocumentOperations,
    Action(Action),
}

impl TryFrom<sdk::FinalizedTransaction> for Transaction {
    type Error = M10Error;

    fn try_from(txn: sdk::FinalizedTransaction) -> Result<Self, Self::Error> {
        use sdk::transaction_data::Data;
        let tx = match txn.data().ok_or(M10Error::InvalidTransaction)? {
            Data::Transfer(_) | Data::InitiateTransfer(_) | Data::CommitTransfer(_) => {
                Transaction::Transfer(Transfer::try_from(txn)?)
            }
            Data::CreateLedgerAccount(_)
            | Data::SetFreezeState(_)
            | Data::SetInstrument(_)
            | Data::SetBalanceLimit(_) => Transaction::AccountUpdate(AccountUpdate::try_from(txn)?),
            Data::InvokeAction(_) => Transaction::Action(Action::try_from(txn)?),
            // TODO @sadroeck - fixme
            Data::DocumentOperations(_) => Transaction::DocumentOperations,
        };
        Ok(tx)
    }
}

pub struct ContextFilter(pub Vec<u8>);

impl TxnFilter<ContextFilter> {
    pub fn context_id(id: Vec<u8>) -> Self {
        Self {
            filter: ContextFilter(id),
            min: 0,
            max: u64::MAX,
            limit: DEFAULT_TXN_LIMIT,
        }
    }
}

impl From<TxnFilter<ContextFilter>> for sdk::ListTransactionsRequest {
    fn from(filter: TxnFilter<ContextFilter>) -> Self {
        sdk::ListTransactionsRequest {
            context_id: filter.filter.0,
            min_tx_id: filter.min,
            max_tx_id: filter.max,
            limit: filter.limit,
        }
    }
}

pub struct GroupingFilter {
    account_id: AccountId,
}

impl GroupingFilter {
    pub fn account(account_id: AccountId) -> Self {
        Self { account_id }
    }
}

impl From<TxnFilter<GroupingFilter>> for sdk::GroupTransactionsRequest {
    fn from(filter: TxnFilter<GroupingFilter>) -> Self {
        sdk::GroupTransactionsRequest {
            account_id: filter.filter.account_id.to_vec(),
            min_tx_id: filter.min,
            max_tx_id: filter.max,
            limit_groups: filter.limit,
        }
    }
}

impl TxnFilter<GroupingFilter> {
    pub fn account(account_id: AccountId) -> Self {
        Self {
            filter: GroupingFilter { account_id },
            min: 0,
            max: u64::MAX,
            limit: DEFAULT_TXN_LIMIT,
        }
    }
}
