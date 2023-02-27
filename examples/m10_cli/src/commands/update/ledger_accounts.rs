use crate::context::Context;
use clap::{ArgGroup, Parser};
use m10_sdk::account::AccountId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("instrument").requires_all(&["code", "decimals"]).multiple(true), about)]
pub(crate) struct UpdateLedgerAccountOptions {
    /// Account id
    id: AccountId,
    /// Update freeze state
    #[clap(short, long)]
    freeze: Option<bool>,
    /// Currency code
    #[clap(short, long, group = "instrument")]
    code: Option<String>,
    /// Number of relevant currency decimals
    #[clap(short, long, group = "instrument")]
    decimals: Option<u32>,
    /// Currency description
    #[clap(long, group = "instrument")]
    description: Option<String>,
    /// Holding balance limit
    holding_limit: Option<u64>,
}

impl UpdateLedgerAccountOptions {
    pub(super) async fn update(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;

        if let Some(frozen) = self.freeze {
            context
                .m10_client
                .freeze_account(self.id, frozen, config.context_id.clone())
                .await?;
        }

        if let Some(limit) = self.holding_limit {
            context
                .m10_client
                .set_account_limit(self.id, limit, config.context_id.clone())
                .await?;
        }

        if let Some(code) = self.code.clone() {
            context
                .m10_client
                .set_account_instrument(
                    self.id,
                    code,
                    self.decimals.unwrap(),
                    self.description.clone(),
                    config.context_id.clone(),
                )
                .await?;
        }
        Ok(())
    }
}
