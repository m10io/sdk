use clap::Args;
use m10_sdk::{Format, NameOrOwnerFilter, PageBuilder, PrettyPrint, PublicKey};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct FindAccountArgs {
    /// Set name filter
    #[arg(short, long, group = "filter")]
    name: Option<String>,
    /// Set owner filter
    #[arg(short, long, group = "filter")]
    owner: Option<PublicKey>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindAccountArgs {
    pub(crate) async fn find(&self, context: &Context) -> anyhow::Result<()> {
        context
            .ledger_client()
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
