#![allow(dead_code)]
pub use config::ConfigError;
use m10_bank_emulator_protos::emulator::rtgs::{
    bank_registration_request::BankAccountType, open_account_request::HolderType,
    requisition_funds_request::RequisitionType, rtgs_service_client::RtgsServiceClient,
    AccountRequest, AccountType, BankRegistrationRequest, OpenAccountRequest,
    RequisitionFundsRequest,
};
use m10_sdk::{BankAccountType as SdkBankAccountType, Signer};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::Acquire;
use std::{collections::HashMap, convert::TryFrom, net::SocketAddr, path::Path, sync::Arc};
use tokio::sync::OnceCell;
use tonic::{
    transport::{Channel, ClientTlsConfig, Endpoint, Uri},
    Code, Request,
};
use uuid::Uuid;

use crate::{
    bank::Bank,
    context::Context,
    error::{Error, ResultExt},
    models::Currency,
    requiem::RequiemServiceConfig,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub database_url: String,
    pub log_filter: String,
    pub oauth: OAuthConfig,
    pub bank_name: String,
    pub short_name: String,
    pub swift_code: String,
    pub bank: BankConfig,
    pub currencies: HashMap<String, CurrencyConfig>,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub ledger_addr: Endpoint,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub directory_addr: Endpoint,
    pub key_pair: Arc<m10_sdk::Ed25519>,
    pub prometheus_port: u16,
    pub requiem: Option<RequiemServiceConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OAuthConfig {
    pub issuer: String,
    pub audience: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CurrencyConfig {
    pub code: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub rtgs_addr: Endpoint,
    pub cb_name: String,
    pub reserve_config: ReserveConfig,
    pub cbdc_config: Option<CbdcConfig>,
    pub drc_config: Option<DrcConfig>,
    #[serde(default = "CurrencyConfig::default_decimals")]
    pub decimals: u32,
    pub asset: bool,
    #[serde(skip)]
    pub asset_id: OnceCell<Uuid>,
    pub test: Option<TestConfig>,
}

impl CurrencyConfig {
    #[cfg(test)]
    pub(crate) fn new_test() -> Self {
        Self {
            code: "usd".to_string(),
            rtgs_addr: Endpoint::from_static("localhost"),
            cb_name: String::new(),
            reserve_config: ReserveConfig::default(),
            cbdc_config: None,
            drc_config: None,
            decimals: 2,
            asset: false,
            asset_id: OnceCell::default(),
            test: None,
        }
    }

    const fn default_decimals() -> u32 {
        2
    }

    pub(crate) async fn get_or_register(&self, context: &Context) -> Result<Currency, Error> {
        if self.test.is_some() {
            return Ok(Currency::default());
        }
        let mut conn = context.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(0xBE000C)
            .execute(&mut *txn)
            .await?;
        let mut currency = Currency::get(&self.code, &mut *txn).await?;
        if currency.bank_id.is_some() {
            txn.rollback().await?;
            return Ok(currency);
        }

        let mut rtgs = RtgsServiceClient::new(self.rtgs_addr.connect_lazy());
        let mut accounts = vec![BankAccountType::Drm as i32];
        if self.cbdc_config.is_some() {
            accounts.push(BankAccountType::Cbdc as i32);
        }
        let registration_request = BankRegistrationRequest {
            institute: context.config.bank_name.clone(),
            swift_code: context.config.swift_code.clone(),
            public_key: context.ledger.signer()?.public_key().to_vec(),
            currency_code: self.code.to_uppercase(),
            display_name: context.config.bank_name.clone(),
            short_display: context.config.short_name.clone(),
            accounts,
        };
        let bank_data = rtgs.register_bank(registration_request).await?.into_inner();
        currency.bank_id = Some(bank_data.bank_meta_data.to_vec());
        let bank_meta_data = context.ledger.get_bank(bank_data.bank_meta_data).await?;
        for a in bank_meta_data.accounts {
            match a.account_type {
                SdkBankAccountType::CentralBankDigitalCurrency => {
                    currency.cbdc_account = Some(a.id.to_vec())
                }
                SdkBankAccountType::DigitalRegulatedMoney => {
                    currency.regulated_account = Some(a.id.to_vec())
                }
            };
        }
        currency.update(&mut *txn).await?;
        txn.commit().await?;
        Ok(currency)
    }

    pub(crate) async fn reserve_account_id(
        &self,
        context: &crate::context::Context,
    ) -> Result<&i32, Error> {
        self.reserve_config
            .reserve_account_id
            .get_or_try_init(|| async {
                let mut rtgs = RtgsServiceClient::new(self.rtgs_addr.connect_lazy());

                let account_req = AccountRequest {
                    institute: context.config.bank_name.clone(),
                    account_type: AccountType::Reserve as i32,
                    currency_code: self.code.to_uppercase(),
                };

                let reserve_account_id = match rtgs.find_account(Request::new(account_req)).await {
                    Ok(resp) => {
                        let account = resp.into_inner();
                        let bank = context.bank.clone();
                        let loan_account = bank
                            .find_account_by_name(&format!(
                                "{} reserve loans",
                                self.code.to_uppercase()
                            ))
                            .await?;
                        self.reserve_config
                            .reserve_loan_account_id
                            .set(loan_account.id)?;

                        account.account_number
                    }
                    // Open reserve account if it doesn't exists
                    Err(s) if s.code() == Code::NotFound => {
                        self.open_reserve_account(&self.reserve_config, rtgs.clone(), context)
                            .await?
                    }
                    // propagate all other errors up
                    Err(err) => return Err(err.into()),
                };

                Ok(reserve_account_id)
            })
            .await
    }

    pub fn decimal_from_cents(&self, amount: u64) -> Result<Decimal, Error> {
        Ok(Decimal::new(
            i64::try_from(amount).map_err(|_| Error::internal_msg("amount too large"))?,
            self.decimals,
        ))
    }

    pub fn cents_from_decimal(&self, mut decimal: Decimal) -> Result<u64, Error> {
        decimal.rescale(self.decimals);
        u64::try_from(decimal.mantissa()).internal_error("decimal too large")
    }

    async fn open_reserve_account(
        &self,
        rtgs_config: &ReserveConfig,
        mut rtgs_client: RtgsServiceClient<Channel>,
        context: &Context,
    ) -> Result<i32, Error> {
        // open internal loan account to track loans
        let mut bank = context.bank.clone();
        let loan_account = bank
            .create_loan_account(&format!("{} reserve loans", self.code.to_uppercase()))
            .await?;
        rtgs_config.reserve_loan_account_id.set(loan_account.id)?;

        // Open account at CB
        let open_req = OpenAccountRequest {
            institute: context.config.bank_name.clone(),
            holder_type: HolderType::Bank as i32,
            account_type: AccountType::Reserve as i32,
            currency_code: self.code.to_uppercase(),
            name: format!(
                "{} {} Reserves",
                context.config.bank_name,
                self.code.to_uppercase()
            ),
            cbdc_account: context.get_currency_regulated_account(&self.code).await?,
            reference: serde_json::Value::from(loan_account).to_string(),
        };

        let resp = rtgs_client.open_account(Request::new(open_req)).await?;
        let account = resp.into_inner();

        // Self fund account with initial amount as given in config.
        // This represents the traditional money the bank has already at the CB
        let fund_req = RequisitionFundsRequest {
            account: account.account_number,
            amount: rtgs_config.nominal_balance.try_into()?,
            currency_code: self.code.to_uppercase(),
            instructions: "".into(),
            requisition_type: RequisitionType::Initial as i32,
        };

        rtgs_client
            .requisition_funds(Request::new(fund_req))
            .await?;

        Ok(account.account_number)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ReserveConfig {
    #[serde(skip)]
    reserve_account_id: OnceCell<i32>,
    #[serde(skip)]
    pub(crate) reserve_loan_account_id: OnceCell<i64>,
    pub nominal_balance: u64,
    pub balance_threshold: u64,
}

impl ReserveConfig {
    pub fn reserve_balance_low_bound(&self) -> u64 {
        (self.nominal_balance / (100 + self.balance_threshold)) * 100
    }

    pub fn reserve_balance_high_bound(&self) -> u64 {
        (self.nominal_balance * (100 + self.balance_threshold)) / 100
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CbdcConfig {
    pub customer_limit: u64,
    pub nominal_margin: u64,
    margin_threshold: u64,
}

impl CbdcConfig {
    pub fn reserve_balance_low_bound(&self) -> u64 {
        (self.nominal_margin / (100 + self.margin_threshold)) * 100
    }

    pub fn reserve_balance_high_bound(&self) -> u64 {
        (self.nominal_margin * (100 + self.margin_threshold)) / 100
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DrcConfig {
    nominal_fraction: u64,
    fraction_threshold: u64,
}

impl DrcConfig {
    pub fn nominal_reserve(&self, balance: u64) -> u64 {
        balance / 100 * self.nominal_fraction
    }

    pub fn reserve_high_bound(&self, balance: u64) -> u64 {
        balance / 100
            * self
                .nominal_fraction
                .saturating_add(self.fraction_threshold)
    }

    pub fn reserve_low_bound(&self, balance: u64) -> u64 {
        balance / 100
            * self
                .nominal_fraction
                .saturating_sub(self.fraction_threshold)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TestConfig {
    account_owner: Option<String>,
    ledger_account_name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum BankConfig {
    BankConnector(BankConnectorConfig),
    Emulator(BankEmulatorConfig),
}

#[derive(Clone, Debug, Deserialize)]
pub struct BankConnectorConfig {
    pub username: String,
    pub password: String,
    pub url: String,
    pub account_id: Uuid,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BankEmulatorConfig {
    pub database_url: Option<String>,
    pub currencies: Vec<EmulatorHoldingAccount>,
    pub checking_account_start: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EmulatorHoldingAccount {
    pub currency: String,
    pub account_name: String,
    #[serde(default)]
    pub pre_fund_range: std::ops::Range<u64>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::from(Path::new("/etc/m10/config.toml")).required(false))
            .add_source(config::File::from(Path::new("/etc/m10/config-patch.toml")).required(false))
            .add_source(
                config::File::from(Path::new("/root/.config/m10/config.toml")).required(false),
            )
            .add_source(config::File::from(Path::new("./config.toml")).required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
                    .ignore_empty(true),
            )
            .build()?;

        config.try_deserialize()
    }
}

pub fn make_endpoint(endpoint: Uri) -> Result<Endpoint, Error> {
    let secure = endpoint.scheme_str() == Some("https");
    let endpoint = Channel::builder(endpoint);

    if secure {
        let tls_config = ClientTlsConfig::with_enabled_roots(Default::default());
        return Ok(endpoint.tls_config(tls_config)?);
    }

    Ok(endpoint)
}
