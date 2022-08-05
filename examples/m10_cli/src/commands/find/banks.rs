use crate::collections::banks::Bank;
use crate::context::Context;
use crate::utils::print_items;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindBankOptions {
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindBankOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config).await?;
        let request = context
            .admin
            .sign_request(sdk::ListBanksRequest { page: None })
            .await?;
        let response = context.m10_client.list_banks(request).await?;
        let items = response
            .banks
            .into_iter()
            .map(Bank::try_from)
            .collect::<Result<_, anyhow::Error>>()?;
        print_items(items, self.format)?;
        Ok(())
    }
}
