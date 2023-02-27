use crate::context::Context;

use clap::Parser;
use m10_sdk::{Format, NameFilter, PageBuilder, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindRoleOptions {
    /// Set name filter
    #[clap(short, long)]
    name: String,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindRoleOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        context
            .m10_client
            .list_roles(PageBuilder::<_, NameFilter>::name(self.name.clone()))
            .await?
            .print(self.format)?;
        Ok(())
    }
}
