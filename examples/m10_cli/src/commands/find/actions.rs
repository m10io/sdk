use crate::collections::actions::Action;
use crate::context::Context;
use crate::utils::print_items;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindActionOptions {
    /// Action name
    #[clap(short, long, default_value_t)]
    name: String,
    /// Set minimum tx id
    #[clap(short, long, default_value_t)]
    min_tx_id: u64,
    /// Set maximum tx id
    #[clap(short = 'x', long, default_value = "u64::MAX")]
    max_tx_id: u64,
    /// Set account filter
    #[clap(short, long)]
    account: Option<String>,
    /// Set limit
    #[clap(short, long, default_value = "20")]
    limit: u64,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindActionOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let from_account_id = hex::decode(self.account.clone().unwrap_or_default())?;
        let context_id = hex::decode(config.context_id.clone())?;
        let filter = if from_account_id.is_empty() {
            sdk::list_actions_request::Filter::ContextId(context_id)
        } else {
            sdk::list_actions_request::Filter::AccountId(from_account_id)
        };
        let request = context
            .admin
            .sign_request(sdk::ListActionsRequest {
                name: self.name.clone(),
                min_tx_id: self.min_tx_id,
                max_tx_id: self.max_tx_id,
                limit: self.limit,
                filter: Some(filter),
            })
            .await?;
        let txs = context.m10_client.list_actions(request).await?;

        let items = txs
            .actions
            .into_iter()
            .map(Action::try_from)
            .collect::<Result<_, anyhow::Error>>()?;
        print_items(items, self.format)?;
        Ok(())
    }
}
