use crate::collections::ResourceId;
use crate::error::{M10Error, M10Result};
use crate::types::PublicKey;
use m10_protos::sdk;
use m10_protos::sdk::BankAccountRef;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Bank {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub short_name: String,
    pub display_name: String,
    pub accounts: Vec<BankAccount>,
}

#[cfg(feature = "format")]
impl std::fmt::Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            id,
            owner,
            short_name,
            display_name,
            accounts,
        } = self;
        write!(
            f,
            "Bank{{ id={id} owner={owner} short_name={short_name} display_name={display_name} accounts=[",
        )?;
        for account in accounts {
            write!(f, "{account},")?;
        }
        write!(f, "] }}")
    }
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("BankAccount{{ id={id} type={account_type} }}")
)]
#[derive(Clone, Debug, Serialize)]
pub struct BankAccount {
    pub id: ResourceId,
    pub account_type: BankAccountType,
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, Copy, Serialize)]
pub enum BankAccountType {
    CentralBankDigitalCurrency,
    DigitalRegulatedMoney,
}

impl TryFrom<sdk::Bank> for Bank {
    type Error = M10Error;

    fn try_from(bank: sdk::Bank) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(bank.id.as_slice())?,
            owner: PublicKey(bank.owner),
            short_name: bank.short_name,
            display_name: bank.display_name,
            accounts: bank
                .accounts
                .into_iter()
                .map(BankAccount::try_from)
                .collect::<M10Result<_>>()?,
        })
    }
}

impl TryFrom<BankAccountRef> for BankAccount {
    type Error = M10Error;

    fn try_from(account: BankAccountRef) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(account.account_id.as_slice())?,
            account_type: BankAccountType::from(
                sdk::bank_account_ref::BankAccountType::from_i32(account.account_type)
                    .ok_or(M10Error::InvalidTransaction)?,
            ),
        })
    }
}

impl From<sdk::bank_account_ref::BankAccountType> for BankAccountType {
    fn from(account_type: sdk::bank_account_ref::BankAccountType) -> Self {
        match account_type {
            sdk::bank_account_ref::BankAccountType::Cbdc => {
                BankAccountType::CentralBankDigitalCurrency
            }
            sdk::bank_account_ref::BankAccountType::Drm => BankAccountType::DigitalRegulatedMoney,
        }
    }
}
