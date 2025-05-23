#![allow(dead_code)]
use eyre::{bail, OptionExt};
use m10_sdk::PublicKey;
use serde::Deserialize;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{context::Context, error::Error, rbac};

#[derive(Clone, Debug, Deserialize)]
pub struct RequiemServiceConfig {
    pub url: String,
    pub public_key: PublicKey,
}

#[derive(Debug, Clone)]
pub(crate) struct RequiemService {
    config: Option<RequiemServiceConfig>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PublicKeyResponse {
    pub public_key: PublicKey,
}

impl RequiemService {
    pub(crate) fn new_from_config(config: Option<RequiemServiceConfig>) -> Self {
        Self { config }
    }

    pub(crate) async fn get_role(
        &self,
        context: &Context,
    ) -> Result<Option<m10_sdk::ledger::Role>, Error> {
        rbac::find_role_by_name(
            &rbac::get_requiem_service_name(context.config.bank_name.clone()),
            context,
        )
        .await
    }

    pub(crate) async fn init(&self, context: &Context) -> eyre::Result<()> {
        if self.config.is_none() {
            info!("service not configured, skipping initialization",);
            return Ok(());
        }
        debug!("initializing service");
        let cfg = self
            .config
            .clone()
            .ok_or_eyre("Requiem service config not found")?;

        debug!("checking specified public key with the service public key");
        if !self.check_public_key().await? {
            bail!("Recovery service public key mismatch");
        }

        debug!("checking if service role already exists");
        if let Some(role) = self.get_role(context).await? {
            debug!(
                "service role already exists with id: {}",
                Uuid::from_slice(&role.id)?
            );
            return Ok(());
        }

        debug!("creating service role...");
        rbac::create_requiem_rbac_role(cfg.public_key.clone(), context).await?;
        debug!("service role created");
        Ok(())
    }

    async fn check_public_key(&self) -> eyre::Result<bool> {
        let cfg = self
            .config
            .clone()
            .ok_or_eyre("Requiem service config not found")?;

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/api/v1/public-key", cfg.url.clone()))
            .send()
            .await?
            .json::<PublicKeyResponse>()
            .await?;

        Ok(cfg.public_key == response.public_key)
    }
}
