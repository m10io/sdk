use crate::collections::account_sets::AccountSet;
use crate::context::Context;
use crate::utils::print_items;
use clap::{ArgGroup, Parser};
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("filter"), about)]
pub(crate) struct FindAccountSetOptions {
    /// Set name filter
    #[clap(short, long, group = "filter")]
    name: Option<String>,
    /// Set owner filter
    #[clap(short, long, group = "filter")]
    owner: Option<String>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindAccountSetOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let request = context
            .admin
            .sign_request(sdk::ListAccountSetsRequest {
                filter: Some(self.filter_from_options()?),
                page: None,
            })
            .await?;
        let response = context.m10_client.list_account_sets(request).await?;
        let items = response
            .account_sets
            .into_iter()
            .map(AccountSet::try_from)
            .collect::<Result<_, anyhow::Error>>()?;
        print_items(items, self.format)?;
        Ok(())
    }

    fn filter_from_options(&self) -> Result<sdk::list_account_sets_request::Filter, anyhow::Error> {
        if let Some(name) = &self.name {
            return Ok(sdk::list_account_sets_request::Filter::Name(name.into()));
        }
        if let Some(owner) = &self.owner {
            return Ok(sdk::list_account_sets_request::Filter::Owner(
                base64::decode(owner)?,
            ));
        }
        Err(anyhow::anyhow!("missing filter"))
    }
}
