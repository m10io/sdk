use clap::{ArgGroup, Args, ValueEnum};
use m10_sdk::{account::AccountId, sdk, Format, PrettyPrint, TransferFilter, TxnFilter};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum TransferState {
    Accepted,
    Pending,
    Expired,
    Rejected,
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("filter").required(true))]
pub(crate) struct FindTransferArgs {
    /// Set minimum tx id
    #[arg(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[arg(short = 'x', long)]
    max_tx_id: Option<u64>,
    /// Set account filter
    #[arg(short, long, group = "filter")]
    account: Option<AccountId>,
    /// Set contextID filter
    #[arg(short, long, group = "filter")]
    context_id: Option<String>,
    /// Set limit
    #[arg(short, long, default_value = "20")]
    limit: u64,
    /// Include child accounts in result
    #[arg(short, long)]
    include_child_accounts: bool,
    /// Set status filter
    #[arg(long)]
    status: Option<TransferState>,
    /// Set enhanced result
    #[arg(short, long)]
    enhanced: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindTransferArgs {
    pub(crate) async fn find(&self, context: &Context) -> anyhow::Result<()> {
        let max_tx_id = self.max_tx_id.unwrap_or(u64::MAX);

        // Build the base filter (account / context + range + limit + child accounts)
        let mut filter = if let Some(account) = self.account {
            TxnFilter::<TransferFilter>::by_account(account)
        } else if let Some(context_id) = &self.context_id {
            TxnFilter::<TransferFilter>::by_context_id(hex::decode(context_id)?)
        } else {
            anyhow::bail!("Missing account or context_id filter")
        };

        filter = filter
            .min_tx(self.min_tx_id)
            .max_tx(max_tx_id)
            .limit(self.limit)
            .include_child_accounts(self.include_child_accounts);

        // Only apply a state filter if the user explicitly provided --status.
        // If no status is provided, we leave the state unset / default so the server
        // returns transfers in all states.
        if let Some(status) = &self.status {
            let state = match status {
                TransferState::Accepted => sdk::finalized_transfer::TransferState::Accepted,
                TransferState::Pending => sdk::finalized_transfer::TransferState::Pending,
                TransferState::Rejected => sdk::finalized_transfer::TransferState::Rejected,
                TransferState::Expired => sdk::finalized_transfer::TransferState::Expired,
            };
            filter = filter.state(state);
        }

        if self.enhanced {
            context
                .ledger_client()
                .get_enhanced_transfers(filter)
                .await?
                .print(self.format)?;
        } else {
            context
                .ledger_client()
                .list_transfers(filter)
                .await?
                .print(self.format)?;
        }

        Ok(())
    }
}
