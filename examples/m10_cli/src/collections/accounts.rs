use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize)]
pub(crate) struct AccountEntry {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) owner: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) frozen: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) public_name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) balance: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) issued_balance: Option<u64>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) profile_image_url: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) instrument: Option<Instrument>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub public_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub profile_image_url: String,
}

impl TryFrom<sdk::Account> for Account {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Account) -> Result<Account, Self::Error> {
        let sdk::Account {
            id,
            owner,
            name,
            public_name,
            profile_image_url,
            ..
        } = other;
        Ok(Account {
            id: hex::encode(id),
            owner: base64::encode(owner),
            name,
            public_name,
            profile_image_url,
        })
    }
}

#[derive(Serialize)]
pub(crate) struct Instrument {
    code: String,
    decimals: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
}

impl From<sdk::Instrument> for Instrument {
    fn from(other: sdk::Instrument) -> Self {
        Self {
            code: other.code,
            decimals: other.decimal_places,
            description: other.description,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct IndexedAccount {
    pub(crate) balance: u64,
    pub(crate) frozen: bool,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) issued_balance: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) non_leaf_children: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) leaf_children: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    pub(crate) instrument: Option<Instrument>,
}

impl TryFrom<sdk::IndexedAccount> for IndexedAccount {
    type Error = anyhow::Error;

    fn try_from(other: sdk::IndexedAccount) -> Result<IndexedAccount, Self::Error> {
        let sdk::IndexedAccount {
            balance,
            frozen,
            issuance,
            instrument,
            ..
        } = other;
        Ok(IndexedAccount {
            balance,
            frozen,
            issued_balance: issuance.as_ref().map(|a| a.issued_balance),
            non_leaf_children: issuance.as_ref().map(|a| a.non_leaf_children),
            leaf_children: issuance.as_ref().map(|a| a.leaf_children),
            instrument: instrument.map(|i| i.into()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AccountCreated {
    pub account_id: String,
    pub is_frozen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SetFrozen {
    pub account_id: String,
    pub is_frozen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SetInstrument {
    pub account_id: String,
    pub code: String,
    pub decimals: u32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AccountRef {
    pub(crate) ledger_id: String,
    pub(crate) account_id: String,
}

impl From<sdk::AccountRef> for AccountRef {
    fn from(other: sdk::AccountRef) -> Self {
        Self {
            ledger_id: other.ledger_id,
            account_id: hex::encode(&other.account_id),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AccountInfo {
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub public_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub profile_image_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub code: String,
    pub decimals: u32,
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
            id: hex::encode(account_id),
            parent_id: hex::encode(parent_account_id),
            public_name,
            profile_image_url,
            code,
            decimals: decimal_places,
        })
    }
}
