use m10_sdk::sdk;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Account {
    pub(crate) name: String,
    pub(crate) balance: u64,
    pub(crate) issued: Option<u64>,
    pub(crate) frozen: bool,
}

impl From<(sdk::IndexedAccount, sdk::Account)> for Account {
    fn from(other: (sdk::IndexedAccount, sdk::Account)) -> Self {
        let sdk::IndexedAccount {
            balance,
            frozen,
            issuance,
            ..
        } = other.0;
        let sdk::Account { public_name, .. } = other.1;
        Self {
            name: public_name,
            balance,
            frozen,
            issued: issuance.as_ref().map(|a| a.issued_balance),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AssetInfo {
    pub(crate) account_id: String,
    pub(crate) name: String,
    pub(crate) code: String,
    pub(crate) decimals: u32,
    pub(crate) issued: u64,
}

#[derive(Serialize)]
pub(crate) struct LedgerInfo {
    pub(crate) url: String,
    pub(crate) height: u64,
}

pub(crate) fn to_tauri_error(error: anyhow::Error) -> tauri::Error {
    tauri::api::Error::Command(error.to_string()).into()
}

#[derive(Clone, serde::Serialize)]
pub(crate) struct InitError {
    pub(crate) message: String,
}
