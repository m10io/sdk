use crate::account::AccountId;
use crate::error::M10Error;
use crate::types::TxId;
use crate::TransactionExt;
use m10_protos::sdk;
use m10_protos::sdk::transaction_data::Data;
use m10_protos::sdk::IndexedAccount;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("id={id} balance={balance} frozen={frozen} currency={code}({decimals}) limit={balance_limit} issuance={issuance:?}")
)]
#[derive(Clone, Debug)]
pub struct Account {
    pub id: AccountId,
    pub balance: u64,
    pub frozen: bool,
    pub code: String,
    pub decimals: u32,
    pub balance_limit: u64,
    pub issuance: Option<Issuance>,
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("balance={balance} issuance_accounts={issuance_accounts} holding_accounts={holding_accounts}")
)]
#[derive(Clone, Debug)]
pub struct Issuance {
    pub balance: u64,
    pub issuance_accounts: u64,
    pub holding_accounts: u64,
}

#[derive(Clone, Debug)]
pub struct AccountUpdate {
    pub tx_id: TxId,
    pub context_id: Vec<u8>,
    pub success: bool,
    pub timestamp: SystemTime,
    pub update_type: AccountUpdateType,
}

#[cfg(feature = "format")]
impl std::fmt::Display for AccountUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            tx_id,
            context_id,
            success,
            timestamp,
            update_type,
        } = self;
        write!(f, "AccountUpdate{{ tx_id={tx_id} context_id={} success=${success} timestamp={timestamp:?} update_type=${update_type} }}", hex::encode(&context_id))
    }
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug)]
pub enum AccountUpdateType {
    #[cfg_attr(
        feature = "format",
        display("SetFrozen{{ account_id={account_id} is_frozen={is_frozen} }}")
    )]
    SetFrozen {
        account_id: AccountId,
        is_frozen: bool,
    },
    #[cfg_attr(
        feature = "format",
        display("SetBalanceLimit{{ account_id={account_id} limit={limit} }}")
    )]
    SetBalanceLimit { account_id: AccountId, limit: u64 },
    #[cfg_attr(
        feature = "format",
        display("SetInstrument{{ account_id={account_id} code={code}({decimals}) }}")
    )]
    SetInstrument {
        account_id: AccountId,
        code: String,
        decimals: u32,
    },
    #[cfg_attr(
        feature = "format",
        display("NewAccount{{ account_id={account_id:?} }}")
    )]
    NewAccount { account_id: Option<AccountId> },
}

impl TryFrom<IndexedAccount> for Account {
    type Error = M10Error;

    fn try_from(account: IndexedAccount) -> Result<Self, Self::Error> {
        let instrument = account.instrument.ok_or(M10Error::InvalidTransaction)?;
        Ok(Self {
            id: AccountId::try_from_be_slice(&account.id)?,
            balance: account.balance,
            frozen: account.frozen,
            code: instrument.code,
            decimals: instrument.decimal_places,
            balance_limit: account.balance_limit,
            issuance: account.issuance.map(Issuance::try_from).transpose()?,
        })
    }
}

impl TryFrom<sdk::indexed_account::Issuance> for Issuance {
    type Error = M10Error;

    fn try_from(issuance: sdk::indexed_account::Issuance) -> Result<Self, Self::Error> {
        Ok(Self {
            balance: issuance.issued_balance,
            issuance_accounts: issuance.non_leaf_children,
            holding_accounts: issuance.leaf_children,
        })
    }
}

impl TryFrom<sdk::FinalizedTransaction> for AccountUpdate {
    type Error = M10Error;

    fn try_from(txn: sdk::FinalizedTransaction) -> Result<Self, Self::Error> {
        let response = txn.response.as_ref().ok_or(M10Error::InvalidTransaction)?;
        let context_id = txn
            .request
            .as_ref()
            .ok_or(M10Error::InvalidTransaction)?
            .context_id
            .clone();
        let success = response.error.is_none();
        let timestamp = UNIX_EPOCH + Duration::from_micros(response.timestamp);
        let tx_id = response.tx_id;
        let update_type = match txn.data().ok_or(M10Error::InvalidTransaction)? {
            Data::CreateLedgerAccount(_) => Ok(AccountUpdateType::NewAccount {
                account_id: AccountId::try_from_be_slice(&response.account_created).ok(),
            }),
            Data::SetFreezeState(freeze) => Ok(AccountUpdateType::SetFrozen {
                is_frozen: freeze.frozen,
                account_id: AccountId::try_from_be_slice(&freeze.account_id)?,
            }),
            Data::SetBalanceLimit(limit) => Ok(AccountUpdateType::SetBalanceLimit {
                limit: limit.balance_limit,
                account_id: AccountId::try_from_be_slice(&limit.account_id)?,
            }),
            Data::SetInstrument(instrument) => Ok(AccountUpdateType::SetInstrument {
                account_id: AccountId::try_from_be_slice(&instrument.account_id)?,
                code: instrument.code.clone(),
                decimals: instrument.decimal_places,
            }),
            _ => Err(M10Error::InvalidTransaction),
        }?;
        Ok(Self {
            tx_id,
            context_id,
            success,
            timestamp,
            update_type,
        })
    }
}
