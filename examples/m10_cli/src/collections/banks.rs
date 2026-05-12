use m10_sdk::sdk;
use std::convert::TryFrom;
use std::str::FromStr;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) enum BankStatus {
    Active,
    Pending,
    Suspended,
    Terminated,
}

impl FromStr for BankStatus {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(BankStatus::Active),
            "pending" => Ok(BankStatus::Pending),
            "suspended" => Ok(BankStatus::Suspended),
            "terminated" => Ok(BankStatus::Terminated),
            _ => Err("no match found"),
        }
    }
}

impl From<BankStatus> for i32 {
    fn from(status: BankStatus) -> i32 {
        match status {
            BankStatus::Active => sdk::bank::BankStatus::Active.into(),
            BankStatus::Pending => sdk::bank::BankStatus::Pending.into(),
            BankStatus::Suspended => sdk::bank::BankStatus::Suspended.into(),
            BankStatus::Terminated => sdk::bank::BankStatus::Terminated.into(),
        }
    }
}

impl From<i32> for BankStatus {
    fn from(status: i32) -> Self {
        match sdk::bank::BankStatus::try_from(status).unwrap_or(sdk::bank::BankStatus::Active) {
            sdk::bank::BankStatus::Active => BankStatus::Active,
            sdk::bank::BankStatus::Pending => BankStatus::Pending,
            sdk::bank::BankStatus::Suspended => BankStatus::Suspended,
            sdk::bank::BankStatus::Terminated => BankStatus::Terminated,
        }
    }
}
