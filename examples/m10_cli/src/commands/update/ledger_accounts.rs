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

        if let Some(code) = self.code {
            client
                .set_account_instrument(
                    self.id,
                    code,
                    self.decimals.unwrap(),
                    self.description,
                    context.context_id(),
                )
                .await?;
        }
        Ok(())
    }
}
