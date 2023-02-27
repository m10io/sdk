use crate::context::Context;
use crate::Config;
use clap::Parser;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateTransferOptions {
    /// Transfer id
    #[clap(short, long, action)]
    pub(super) id: u64,
    /// New state
    #[clap(short, long, value_enum, action)]
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

impl UpdateTransferOptions {
    pub async fn do_update(&self, config: &Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        context
            .m10_client
            .commit_transfer(
                self.id,
                match self.state {
                    TransferState::Accept => true,
                    TransferState::Reject => false,
                },
                config.context_id.clone(),
            )
            .await?;
        Ok(())
    }
}
