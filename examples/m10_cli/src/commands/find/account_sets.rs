use clap::{ArgGroup, Args};
use m10_sdk::{Format, NameOrOwnerFilter, PageBuilder, PrettyPrint, PublicKey};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("filter"))]
pub(crate) struct FindAccountSetArgs {
    /// Set name filter
    #[arg(short, long, group = "filter")]
    name: Option<String>,
    /// Set owner filter
    #[arg(short, long, group = "filter")]
    owner: Option<String>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindAccountSetArgs {
    pub(crate) async fn find(&self, context: &Context) -> anyhow::Result<()> {
        let filter = if let Some(name) = &self.name {
            PageBuilder::<_, NameOrOwnerFilter>::name(name)
        } else if let Some(owner) = &self.owner {
            PageBuilder::<_, NameOrOwnerFilter>::owner(PublicKey(base64::decode(owner)?))
        } else {
            return Err(anyhow::anyhow!("missing filter"));
        };
        context
            .ledger_client()
            .list_account_sets(filter)
            .await?
            .print(self.format)?;
        Ok(())
    }
}
