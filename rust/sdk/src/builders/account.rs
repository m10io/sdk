use crate::account::AccountId;
use crate::types::TxId;
use core::convert::From;
use m10_protos::sdk;

#[derive(Default)]
pub struct AccountFilter<T = ()> {
    accounts: Vec<AccountId>,
    starting_from: Option<TxId>,
    extra: T,
}

impl<T> AccountFilter<T> {
    pub fn involves(mut self, id: AccountId) -> Self {
        self.accounts.push(id);
        self
    }

    pub fn starting_from(mut self, tx_id: TxId) -> Self {
        self.starting_from = Some(tx_id);
        self
    }
}

impl From<AccountFilter<()>> for sdk::ObserveAccountsRequest {
    fn from(filter: AccountFilter) -> Self {
        sdk::ObserveAccountsRequest {
            involved_accounts: filter.accounts.into_iter().map(AccountId::to_vec).collect(),
            starting_from: filter.starting_from.map(|tx_id| sdk::TxId { tx_id }),
        }
    }
}

#[derive(Default)]
pub struct NamedAction {
    name: String,
}

impl AccountFilter<NamedAction> {
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            extra: NamedAction { name: name.into() },
            ..Default::default()
        }
    }
}

impl From<AccountFilter<NamedAction>> for sdk::ObserveActionsRequest {
    fn from(filter: AccountFilter<NamedAction>) -> Self {
        sdk::ObserveActionsRequest {
            involves_accounts: filter.accounts.into_iter().map(AccountId::to_vec).collect(),
            starting_from: filter.starting_from.map(|tx_id| sdk::TxId { tx_id }),
            name: filter.extra.name,
        }
    }
}
