use crate::context::Context;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetActionOptions {
    /// The transaction id
    id: u64,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl GetActionOptions {
    pub(super) async fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let request = context
            .admin
            .sign_request(sdk::GetActionRequest { tx_id: self.id })
            .await?;
        let tx = context.m10_client.get_action(request).await?;
        super::print_item(
            crate::collections::actions::Action::try_from(tx)?,
            self.format,
        )
    }
}
