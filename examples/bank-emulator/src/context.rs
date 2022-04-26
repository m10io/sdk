use m10_rds_pool::{bb8, RdsManager};
use m10_sdk::{
    client::Channel, directory::directory_service_client::DirectoryServiceClient, LedgerClient,
};

use crate::emulator::BankEmulator;
use crate::{
    config::{BankConfig, Config},
    error::Error,
    signer::ProxySigner,
};

#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) ledger: LedgerClient,
    pub(crate) signer: ProxySigner,
    pub(crate) directory: DirectoryServiceClient<Channel>,
    pub(crate) bank: BankEmulator,
    pub(crate) db_pool: bb8::Pool<RdsManager>,
    pub(crate) config: Config,
}

impl Context {
    #[allow(dead_code)]
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
                    ledger: LedgerClient::new(config.ledger_addr.connect_lazy()?),
                    signer: ProxySigner::new(&config),
                    directory: DirectoryServiceClient::new(config.directory_addr.connect_lazy()?),
                    bank: BankEmulator::new_from_config(emulator_config, bank_db_pool).await?,
                    db_pool,
                    config,
                })
            }
            _ => Err(Error::internal_msg("Unsupported bank configuration")),
        }
    }
}
