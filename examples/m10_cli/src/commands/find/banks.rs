use crate::context::Context;

use clap::Parser;
use m10_sdk::{Format, PageBuilder, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindBankOptions {
    #[clap(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindBankOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        context
            .m10_client
            .list_banks(PageBuilder::default())
            .await?
            .print(self.format)?;
        Ok(())
    }
}
