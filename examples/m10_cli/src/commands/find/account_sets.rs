use crate::context::Context;
use clap::{ArgGroup, Parser};
use m10_sdk::{Format, NameOrOwnerFilter, PageBuilder, PrettyPrint, PublicKey};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
    #[serde(default)]
    format: Format,
}

impl FindAccountSetOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        let filter = if let Some(name) = &self.name {
            PageBuilder::<_, NameOrOwnerFilter>::name(name)
        } else if let Some(owner) = &self.owner {
            PageBuilder::<_, NameOrOwnerFilter>::owner(PublicKey(base64::decode(owner)?))
        } else {
            return Err(anyhow::anyhow!("missing filter"));
        };
        context
            .m10_client
            .list_account_sets(filter)
            .await?
            .print(self.format)?;
        Ok(())
    }
}
