#![allow(dead_code)]
pub use config::{ConfigError, File};
use m10_bank_emulator_protos::rtgs::{
    open_account_request::HolderType, rtgs_service_client::RtgsServiceClient, AccountRequest,
    AccountType, AquireFundsRequest, BalanceRequest, OpenAccountRequest,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::{collections::HashMap, convert::TryFrom, net::SocketAddr, path::Path, sync::Arc};
use tokio::sync::OnceCell;
use tonic::{Code, Request};
use uuid::Uuid;

use crate::{
    context::Context,
    error::{Error, ResultExt},
    utils::find_ledger_account,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub database_url: String,
    pub log_filter: String,
    pub oauth: OAuthConfig,
    pub bank_name: String,
    pub bank: BankConfig,
    pub currencies: HashMap<String, CurrencyConfig>,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub ledger_addr: tonic::transport::Endpoint,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub directory_addr: tonic::transport::Endpoint,
    pub key_pair: Arc<m10_sdk::Ed25519>,
    pub prometheus_port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OAuthConfig {
    pub issuer: String,
    pub audience: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CurrencyConfig {
    pub code: String,
    pub rtgs: Option<RtgsConfig>,
    pub ledger_account_name: String,
    pub account_owner: Option<String>,
    #[serde(skip)]
    pub ledger_account_id: OnceCell<[u8; 16]>,
    pub cbdc: Option<CbdcConfig>,
    #[serde(default = "CurrencyConfig::default_decimals")]
    pub decimals: u32,
    pub asset: bool,
    #[serde(skip)]
    pub asset_id: OnceCell<Uuid>,
    #[serde(default)]
    pub test: bool,
}

impl CurrencyConfig {
    const fn default_decimals() -> u32 {
        2
    }

    pub(crate) async fn ledger_account_id(&self, context: &Context) -> Result<&[u8; 16], Error> {
        self.ledger_account_id
            .get_or_try_init(|| async {
                let id = find_ledger_account(
                    &self.ledger_account_name,
                    self.account_owner.as_ref(),
                    context,
                )
                .await?
                .try_into()
                .map_err(|_| Error::internal_msg("invalid account id"))?;
                Ok(id)
            })
            .await
    }

    pub(crate) async fn reserve_account_id(
        &self,
        context: &crate::context::Context,
    ) -> Result<&i32, Error> {
        let rtgs_config = self
            .rtgs
            .as_ref()
            .ok_or_else(|| Error::internal_msg("no rtgs configured"))?;

        rtgs_config
            .reserve_account_id
            .get_or_try_init(|| async {
                let mut rtgs = RtgsServiceClient::new(rtgs_config.addr.connect_lazy()?);

                let account_req = AccountRequest {
                    institute: context.config.bank_name.clone(),
                    account_type: AccountType::Reserve as i32,
                    currency_symbol: self.code.to_uppercase(),
                };

                let reserve_account_id = match rtgs.find_account(Request::new(account_req)).await {
                    Ok(resp) => {
                        let account = resp.into_inner();
                        account.account_number
                    }
                    // Open reserve account if it doesn't exists
                    Err(s) if s.code() == Code::NotFound => {
                        let open_req = OpenAccountRequest {
                            institute: context.config.bank_name.clone(),
                            holder_type: HolderType::Bank as i32,
                            account_type: AccountType::Reserve as i32,
                            currency_symbol: self.code.to_uppercase(),
                            name: format!(
                                "{} {} Reserves",
                                context.config.bank_name,
                                self.code.to_uppercase()
                            ),
                            cbdc_account: self.ledger_account_id(context).await?.to_vec(),
                        };

                        let resp = rtgs.open_account(Request::new(open_req)).await?;
                        let account = resp.into_inner();
                        account.account_number
                    }
                    // propagate all other errors up
                    Err(err) => return Err(err.into()),
                };

                let balance_req = BalanceRequest {
                    account_number: reserve_account_id,
                };

                let balance = rtgs
                    .get_balance(Request::new(balance_req))
                    .await?
                    .into_inner();

                if balance.balance < rtgs_config.reserve_balance_low_bound() {
                    let fund_req = AquireFundsRequest {
                        account: reserve_account_id,
                        amount: rtgs_config.reserve_balance_high_bound() - balance.balance,
                        currency_symbol: self.code.to_uppercase(),
                        instructions: "".into(),
                    };

                    rtgs.aquire_funds(Request::new(fund_req)).await?;
                }
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
}

#[derive(Clone, Debug, Deserialize)]
pub struct RtgsConfig {
    pub institute_name: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub addr: tonic::transport::Endpoint,
    #[serde(skip)]
    reserve_account_id: OnceCell<i32>,
    pub reserve_balance: i64,
    pub reserve_threshold: i64,
}

impl RtgsConfig {
    pub fn reserve_balance_low_bound(&self) -> i64 {
        (self.reserve_balance / (100 + self.reserve_threshold)) * 100
    }

    pub fn reserve_balance_high_bound(&self) -> i64 {
        (self.reserve_balance * (100 + self.reserve_threshold)) / 100
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CbdcConfig {
    pub ledger_account_name: String,
    pub customer_limit: u64,
    #[serde(skip)]
    pub ledger_account_id: OnceCell<[u8; 16]>,
    pub reserve_balance: i64,
    pub reserve_threshold: i64,
}

impl CbdcConfig {
    pub(crate) async fn ledger_account_id(
        &self,
        account_owner: Option<&String>,
        context: &Context,
    ) -> Result<&[u8; 16], Error> {
        self.ledger_account_id
            .get_or_try_init(|| async {
                let id = find_ledger_account(&self.ledger_account_name, account_owner, context)
                    .await?
                    .try_into()
                    .map_err(|_| Error::internal_msg("invalid account id"))?;
                Ok(id)
            })
            .await
    }

    pub fn reserve_balance_low_bound(&self) -> i64 {
        (self.reserve_balance / (100 + self.reserve_threshold)) * 100
    }

    pub fn reserve_balance_high_bound(&self) -> i64 {
        (self.reserve_balance * (100 + self.reserve_threshold)) / 100
    }
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
        let mut config = config::Config::default();
        let config_files = &[
            "/etc/m10/config.toml",
            "/etc/m10/config-patch.toml",
            "/root/.config/m10/config.toml",
            "./config.toml",
        ];
        for config_file in config_files {
            config.merge(config::File::from(Path::new(config_file)).required(false))?;
        }
        config.merge(
            config::Environment::with_prefix("M10")
                .separator("__")
                .ignore_empty(true),
        )?;
        config.try_into()
    }
}
