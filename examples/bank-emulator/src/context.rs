#![allow(dead_code)]
use std::sync::Arc;

use m10_rds_pool::{bb8, RdsManager};
use m10_sdk::M10CoreClient;
use m10_sdk::{directory::directory_service_client::DirectoryServiceClient, GrpcClient};
use sqlx::Acquire;

use crate::emulator::BankEmulator;
use crate::models::Currency;
use crate::{
    config::{BankConfig, Config, CurrencyConfig},
    error::Error,
    signer::ProxySigner,
};

#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) ledger: Arc<Box<dyn M10CoreClient<Signer = ProxySigner> + Send + Sync>>,
    pub(crate) directory: DirectoryServiceClient<tonic::transport::Channel>,
    pub(crate) bank: BankEmulator,
    pub(crate) db_pool: bb8::Pool<RdsManager>,
    pub(crate) config: Config,
}

impl Context {
    pub async fn new_from_config(config: Config) -> Result<Self, Error> {
        let db_pool = RdsManager::new(config.database_url.clone()).pool().await?;
        match &config.bank {
            BankConfig::Emulator(emulator_config) => {
                let bank_db_pool = if let Some(db_url) = emulator_config.database_url.as_ref() {
                    m10_rds_pool::RdsManager::new(db_url.into()).pool().await?
                } else {
                    db_pool.clone()
                };
                Ok(Self {
                    ledger: Arc::new(Box::new(GrpcClient::new(
                        config.ledger_addr.clone(),
                        Some(std::sync::Arc::new(ProxySigner::new(&config))),
                    )?)),
                    directory: DirectoryServiceClient::new(config.directory_addr.connect_lazy()?),
                    bank: BankEmulator::new_from_config(emulator_config, bank_db_pool).await?,
                    db_pool,
                    config,
                })
            }
            _ => Err(Error::internal_msg("Unsupported bank configuration")),
        }
    }

    pub(crate) async fn init(&self) -> Result<(), Error> {
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        for currency in self.config.currencies.values() {
            if Currency::get(&currency.code, &mut txn).await.is_err() {
                let mut entry = Currency {
                    code: currency.code.to_string(),
                    ..Default::default()
                };
                entry.insert(&mut txn).await?;
            }
        }
        txn.commit().await?;
        Ok(())
    }

    pub(crate) fn get_currency(&self, code: &str) -> Result<&CurrencyConfig, Error> {
        let currency = self
            .config
            .currencies
            .get(&code.to_lowercase())
            .ok_or_else(|| Error::not_found(format!("currency {} configuration", code)))?;
        Ok(currency)
    }

    pub(crate) async fn get_currency_regulated_account(
        &self,
        code: &str,
    ) -> Result<Vec<u8>, Error> {
        let currency_config = self.get_currency(code)?;
        let currency = currency_config.get_or_register(self).await?;
        currency
            .regulated_account
            .ok_or_else(|| Error::not_found("DRM issuance account"))
    }

    pub(crate) async fn get_currency_cbdc_account(&self, code: &str) -> Result<Vec<u8>, Error> {
        let currency_config = self.get_currency(code)?;
        let currency = currency_config.get_or_register(self).await?;
        currency
            .cbdc_account
            .ok_or_else(|| Error::not_found("CBDC issuance account"))
    }
}
