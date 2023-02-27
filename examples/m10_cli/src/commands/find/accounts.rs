use crate::context::Context;
use clap::Parser;
use m10_sdk::{Format, NameOrOwnerFilter, PageBuilder, PrettyPrint, PublicKey};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindAccountOptions {
    /// Set name filter
    #[clap(short, long, group = "filter")]
    name: Option<String>,
    /// Set owner filter
    #[clap(short, long, group = "filter")]
    owner: Option<PublicKey>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindAccountOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        context
            .m10_client
            .list_accounts(self.filter_from_options()?)
            .await?
            .print(self.format)?;
        Ok(())
    }

    fn filter_from_options(&self) -> anyhow::Result<PageBuilder<Vec<u8>, NameOrOwnerFilter>> {
        if let Some(name) = &self.name {
            Ok(PageBuilder::filter(NameOrOwnerFilter::Name(
                name.to_string(),
            )))
        } else if let Some(owner) = &self.owner {
            Ok(PageBuilder::filter(NameOrOwnerFilter::Owner(owner.clone())))
        } else {
            Err(anyhow::anyhow!("missing filter"))
        }
    }
}
