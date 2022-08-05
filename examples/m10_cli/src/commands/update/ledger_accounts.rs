use crate::context::Context;
use clap::{ArgGroup, Parser};
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("instrument").requires_all(&["code", "decimals"]).multiple(true), about)]
pub(crate) struct UpdateLedgerAccountOptions {
    /// Account id
    id: String,
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
        let mut context = Context::new(config).await?;

        let account_id = hex::decode(&self.id)?;
        if let Some(frozen) = self.freeze {
            let request = sdk::SetFreezeState {
                account_id: account_id.clone(),
                frozen,
            };
            context
                .submit_transaction(request, config.context_id.clone())
                .await??;
        }

        if let Some(limit) = self.holding_limit {
            let request = sdk::SetBalanceLimit {
                account_id: account_id.clone(),
                balance_limit: limit,
            };
            context
                .submit_transaction(request, config.context_id.clone())
                .await??;
        }

        if let Some(code) = self.code.clone() {
            let request = sdk::SetInstrument {
                account_id,
                code,
                decimal_places: self.decimals.unwrap(), // ensured by Clap/requires_all
                description: self.description.clone().unwrap_or_default(),
            };
            context
                .submit_transaction(request, config.context_id.clone())
                .await??;
        }
        Ok(())
    }
}
