use m10_sdk::sdk;
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bank {
    pub id: Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    pub short_name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<BankAccountRef>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BankAccountRef {
    pub(crate) account_id: String,
    account_type: BankAccountType,
}

impl TryFrom<sdk::Bank> for Bank {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Bank) -> Result<Bank, Self::Error> {
        let sdk::Bank {
            id,
            owner,
            short_name,
            display_name,
            accounts,
        } = other;
        Ok(Bank {
            id: Uuid::from_slice(&id).unwrap_or_default(),
            owner: base64::encode(owner),
            short_name,
            display_name,
            accounts: accounts
                .into_iter()
                .map(|a| BankAccountRef {
                    account_type: match a.account_type() {
                        sdk::bank_account_ref::BankAccountType::Cbdc => BankAccountType::Cbdc,
                        sdk::bank_account_ref::BankAccountType::Drm => BankAccountType::Drm,
                    },
                    account_id: hex::encode(a.account_id),
                })
                .collect(),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) enum BankAccountType {
    Cbdc,
    Drm,
}

impl FromStr for BankAccountType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cbdc" => Ok(BankAccountType::Cbdc),
            "drm" => Ok(BankAccountType::Drm),
            _ => Err("no match found"),
        }
    }
}

impl From<BankAccountType> for i32 {
    fn from(t: BankAccountType) -> i32 {
        match t {
            BankAccountType::Cbdc => sdk::bank_account_ref::BankAccountType::Cbdc.into(),
            BankAccountType::Drm => sdk::bank_account_ref::BankAccountType::Drm.into(),
        }
    }
}
