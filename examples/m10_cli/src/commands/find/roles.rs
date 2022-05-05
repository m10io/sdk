use crate::collections::roles::Role;
use crate::context::Context;
use crate::utils::print_items;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindRoleOptions {
    /// Set name filter
    #[clap(short, long)]
    name: Option<String>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindRoleOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let request = sdk::ListRolesRequest {
            filter: Some(self.filter_from_options()?),
            page: None,
        };
        let request = context.admin.sign_request(request).await?;
        let response = context.m10_client.list_roles(request).await?;
        let items = response
            .roles
            .into_iter()
            .map(Role::try_from)
            .collect::<Result<_, anyhow::Error>>()?;
        print_items(items, self.format)?;
        Ok(())
    }

    fn filter_from_options(&self) -> Result<sdk::list_roles_request::Filter, anyhow::Error> {
        if let Some(name) = &self.name {
            return Ok(sdk::list_roles_request::Filter::Name(name.into()));
        }
        Err(anyhow::anyhow!("missing filter"))
    }
}
