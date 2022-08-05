use crate::context::Context;
use crate::Config;
use clap::Parser;
use m10_sdk::ledger::transaction_data::Data;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateTransferOptions {
    /// Transfer id
    #[clap(short, long)]
    pub(super) id: u64,
    /// New state
    #[clap(short, long, arg_enum)]
    pub(super) state: TransferState,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, clap::ArgEnum)]
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

impl UpdateTransferOptions {
    pub async fn do_update(&self, config: &Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;

        let transfer = sdk::CommitTransfer {
            pending_tx_id: self.id,
            new_state: sdk::transaction::commit_transfer::TransferState::from(self.state) as i32,
        };
        context
            .submit_transaction(Data::CommitTransfer(transfer), config.context_id.clone())
            .await??;
        Ok(())
    }
}
