use crate::context::Context;

use clap::Parser;
use m10_sdk::{Format, NameFilter, PageBuilder, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindRoleBindingOptions {
    /// Set name filter
    #[clap(short, long)]
    name: String,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value_t)]
    format: Format,
}

impl FindRoleBindingOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        let builder = PageBuilder::<_, NameFilter>::name(self.name.clone());
        context
            .m10_client
            .list_role_bindings(builder)
            .await?
            .print(self.format)?;
        Ok(())
    }
}
