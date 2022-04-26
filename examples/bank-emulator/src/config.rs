#![allow(dead_code)]
use crate::error::{Error, ResultExt};
pub use config::{ConfigError, File};
use m10_sdk::{sdk, Signer};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    net::SocketAddr,
    path::Path,
    sync::Arc,
};
use tokio::sync::OnceCell;
use tracing::error;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub database_url: String,
    pub log_filter: String,
    pub oauth: OAuthConfig,
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
    pub ledger_account_name: String,
    pub account_owner: Option<String>,
    #[serde(skip)]
    pub ledger_account_id: OnceCell<[u8; 16]>,
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

    pub(crate) async fn ledger_account_id(
        &self,
        context: &crate::context::Context,
    ) -> Result<&[u8; 16], Error> {
        self.ledger_account_id
            .get_or_try_init(|| async {
                let owner = self
                    .account_owner
                    .as_ref()
                    .map(|o| base64::decode(o).unwrap())
                    .unwrap_or_else(|| context.signer.public_key().to_vec());
                let mut client = context.ledger.clone();
                let request = context
                    .signer
                    .sign_request(sdk::ListAccountsRequest {
                        filter: Some(sdk::list_accounts_request::Filter::Owner(owner)),
                        page: None,
                    })
                    .await?;
                let docs = client
                    .list_accounts(request)
                    .await
                    .map_err(|err| Error::internal_msg(err.to_string()))?;
                let account = docs
                    .accounts
                    .iter()
                    .find(|a| a.public_name == self.ledger_account_name)
                    .ok_or_else(|| {
                        error!(name = %self.ledger_account_name, "Could not find account");
                        Error::internal_msg("account not found")
                    })?;
                let id = account
                    .id
                    .to_vec()
                    .try_into()
                    .map_err(|_| Error::internal_msg("invalid account id"))?;
                Ok(id)
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
