use crate::account::AccountId;
use crate::types::TxId;
use crate::WithContext;
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

    pub fn with_accounts(mut self, ids: Vec<AccountId>) -> Self {
        self.accounts.extend(ids.iter());
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

pub struct OwnerFilter {
    owner: Vec<u8>,
}

impl OwnerFilter {
    pub fn new(owner: Vec<u8>) -> Self {
        Self { owner }
    }
}

impl From<OwnerFilter> for sdk::list_account_metadata_request::Filter {
    fn from(filter: OwnerFilter) -> Self {
        sdk::list_account_metadata_request::Filter::Owner(filter.owner)
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

pub struct AccountBuilder {
    parent_id: Option<AccountId>,
    issuance: bool,
    frozen: bool,
    balance_limit: u64,
    instrument: Option<sdk::Instrument>,
}

impl AccountBuilder {
    pub fn new() -> Self {
        Self {
            parent_id: None,
            issuance: false,
            frozen: false,
            balance_limit: 0,
            instrument: None,
        }
    }

    pub fn parent(id: AccountId) -> Self {
        Self {
            parent_id: Some(id),
            issuance: false,
            frozen: false,
            balance_limit: 0,
            instrument: None,
        }
    }

    pub fn root(
        code: impl Into<String>,
        decimals: u32,
        description: Option<impl Into<String>>,
    ) -> Self {
        Self {
            parent_id: None,
            issuance: false,
            frozen: false,
            balance_limit: 0,
            instrument: None,
        }
        .instrument(code, decimals, description)
    }

    pub fn issuance(mut self, flag: bool) -> Self {
        self.issuance = flag;
        self
    }

    pub fn frozen(mut self, flag: bool) -> Self {
        self.frozen = flag;
        self
    }

    pub fn balance_limit(mut self, limit: u64) -> Self {
        self.balance_limit = limit;
        self
    }

    pub fn instrument(
        mut self,
        code: impl Into<String>,
        decimals: u32,
        description: Option<impl Into<String>>,
    ) -> Self {
        self.instrument = Some(sdk::Instrument {
            code: code.into(),
            decimal_places: decimals,
            description: description.map(|d| d.into()).unwrap_or_default(),
        });
        self
    }
}

impl From<AccountBuilder> for sdk::CreateLedgerAccount {
    fn from(builder: AccountBuilder) -> Self {
        sdk::CreateLedgerAccount {
            parent_id: builder.parent_id.map_or(vec![], |id| id.to_vec()),
            frozen: builder.frozen,
            issuance: builder.issuance,
            instrument: builder.instrument,
            balance_limit: builder.balance_limit,
        }
    }
}

impl WithContext for AccountBuilder {}

impl Default for AccountBuilder {
    fn default() -> Self {
        Self::new()
    }
}
