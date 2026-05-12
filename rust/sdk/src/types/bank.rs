use crate::account::AccountId;
use crate::collections::ResourceId;
use crate::error::{M10Error, M10Result};
use crate::types::PublicKey;
use m10_protos::sdk;
use m10_protos::sdk::BankAccountRef;
use parse_display::helpers::once_cell::sync::Lazy;
use parse_display::helpers::regex::Regex;
use reqwest::Url;
use serde::Serialize;

static BIC_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$").unwrap());

#[derive(Clone, Debug, Serialize)]
pub struct Bank {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub short_name: String,
    pub display_name: String,
    pub accounts: Vec<BankAccount>,
    pub status: BankStatus,
    pub country_code: CountryCode,
    pub endpoint: Endpoint,
    pub logo_url: String,
    pub description: String,
    pub bic_swift_code: BicSwiftCode,
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
            status,
            country_code,
            endpoint,
            logo_url,
            description,
            bic_swift_code,
        } = self;
        write!(
            f,
            "Bank{{ id={id} owner={owner} short_name={short_name} display_name={display_name} accounts=[",
        )?;
        for account in accounts {
            write!(f, "{account},")?;
        }
        write!(f, "] status={status} country_code={country_code} endpoint={endpoint} logo_url={logo_url} description={description} bic_swift_code={bic_swift_code} }}")
    }
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("BankAccount{{ id={id} type={account_type} }}")
)]
#[derive(Clone, Debug, Serialize)]
pub struct BankAccount {
    pub id: AccountId,
    pub account_type: BankAccountType,
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, Copy, Serialize)]
pub enum BankAccountType {
    CentralBankDigitalCurrency,
    DigitalRegulatedMoney,
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BankStatus {
    Active,
    Pending,
    Suspended,
    Terminated,
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CountryCode(String);

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Endpoint(String);

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BicSwiftCode(String);

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
            status: BankStatus::from(
                sdk::bank::BankStatus::try_from(bank.status)
                    .map_err(|_| M10Error::InvalidTransaction)?,
            ),
            country_code: CountryCode::try_from(bank.country_code)?,
            endpoint: Endpoint::try_from(bank.endpoint)?,
            logo_url: bank.logo_url,
            description: bank.description,
            bic_swift_code: BicSwiftCode::try_from(bank.bic_swift_code)?,
        })
    }
}

impl From<sdk::bank::BankStatus> for BankStatus {
    fn from(status: sdk::bank::BankStatus) -> Self {
        match status {
            sdk::bank::BankStatus::Active => BankStatus::Active,
            sdk::bank::BankStatus::Pending => BankStatus::Pending,
            sdk::bank::BankStatus::Suspended => BankStatus::Suspended,
            sdk::bank::BankStatus::Terminated => BankStatus::Terminated,
        }
    }
}

impl TryFrom<BankAccountRef> for BankAccount {
    type Error = M10Error;

    fn try_from(account: BankAccountRef) -> Result<Self, Self::Error> {
        Ok(Self {
            id: AccountId::try_from(account.account_id.as_slice())?,
            account_type: BankAccountType::from(
                sdk::bank_account_ref::BankAccountType::try_from(account.account_type)
                    .map_err(|_| M10Error::InvalidTransaction)?,
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

impl TryFrom<String> for CountryCode {
    type Error = M10Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Ok(Self(String::new()));
        }

        if s.len() != 2 || !s.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(M10Error::InvalidTransaction);
        }

        Ok(Self(s))
    }
}

impl TryFrom<String> for Endpoint {
    type Error = M10Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Ok(Self(String::new()));
        }

        let url = Url::parse(&s).map_err(|_| M10Error::InvalidTransaction)?;

        if url.scheme().is_empty() || url.host_str().is_none() {
            return Err(M10Error::InvalidTransaction);
        }

        Ok(Self(s))
    }
}

impl TryFrom<String> for BicSwiftCode {
    type Error = M10Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Ok(Self(String::new()));
        }

        if !BIC_REGEX.is_match(&s) {
            return Err(M10Error::InvalidTransaction);
        }

        Ok(Self(s))
    }
}
