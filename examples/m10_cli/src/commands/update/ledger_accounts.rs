use clap::{ArgGroup, Args};
use m10_sdk::account::AccountId;
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("instrument").requires_all(&["code", "decimals"]).multiple(true))]
pub(crate) struct UpdateLedgerAccountArgs {
    /// Account id
    id: AccountId,
    /// Update freeze state
    #[arg(short, long)]
    freeze: Option<bool>,
    /// Asset code (e.g. USD or EUR)
    #[arg(long, aliases = ["currency", "symbol", "cs", "cc"], group = "instrument")]
    code: Option<String>,
    /// Number of relevant currency decimals
    #[arg(short, long, group = "instrument")]
    decimals: Option<u32>,
    /// Asset description
    #[arg(long, alias = "desc", group = "instrument")]
    description: Option<String>,
    /// Holding balance limit
    #[arg(short = 'l', long, aliases = ["limit", "hl"])]
    holding_limit: Option<u64>,
    /// Issuance limit (maximum outstanding issued balance; 0 = unlimited)
    #[arg(long, alias = "il")]
    issuance_limit: Option<u64>,
    /// Unique display code for the instrument
    #[arg(long, alias = "dc")]
    display_code: Option<String>,
}

impl UpdateLedgerAccountArgs {
    pub(super) async fn update(self, context: &Context) -> anyhow::Result<()> {
        let client = context.ledger_client();
        if let Some(frozen) = self.freeze {
            client
                .freeze_account(self.id, frozen, context.context_id())
                .await?;
        }

        if let Some(limit) = self.holding_limit {
            client
                .set_account_limit(self.id, limit, context.context_id())
                .await?;
        }

        if let Some(limit) = self.issuance_limit {
            client
                .set_issuance_limit(self.id, limit, context.context_id())
                .await?;
        }

        if self.id.depth() != 0 {
            if let Some(display_code) = self.display_code.clone() {
                client
                    .set_display_code(self.id, display_code, context.context_id())
                    .await?;
            }
        } else {
            if self.display_code.is_some() && (self.code.is_none() || self.decimals.is_none()) {
                eprintln!("warning: required code and decimals for root accounts")
            }
        }

        if let Some(code) = &self.code {
            client
                .set_account_instrument(
                    self.id,
                    code.clone(),
                    self.decimals.unwrap(),
                    self.description,
                    context.context_id(),
                    self.display_code.clone(),
                )
                .await?;
        }

        if self.freeze.is_none()
            && self.holding_limit.is_none()
            && self.issuance_limit.is_none()
            && self.code.is_none()
            && self.display_code.is_none()
        {
            eprintln!("warning: no fields specified, nothing was updated");
        }

        Ok(())
    }
}
