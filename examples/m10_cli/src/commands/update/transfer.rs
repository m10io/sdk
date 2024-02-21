use clap::Args;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateTransferArgs {
    /// Transfer id
    #[arg(short, long)]
    pub(super) id: u64,
    /// New state
    #[arg(long, value_enum, alias = "state")]
    pub(super) state: TransferState,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, clap::ValueEnum)]
pub(crate) enum TransferState {
    Accept,
    Reject,
}

impl From<TransferState> for sdk::transaction::commit_transfer::TransferState {
    fn from(state: TransferState) -> Self {
        match state {
            TransferState::Accept => sdk::transaction::commit_transfer::TransferState::Accepted,
            TransferState::Reject => sdk::transaction::commit_transfer::TransferState::Rejected,
        }
    }
}

impl UpdateTransferArgs {
    pub async fn do_update(&self, context: &Context) -> anyhow::Result<()> {
        context
            .ledger_client()
            .commit_transfer(
                self.id,
                match self.state {
                    TransferState::Accept => true,
                    TransferState::Reject => false,
                },
                context.context_id(),
            )
            .await?;
        Ok(())
    }
}
