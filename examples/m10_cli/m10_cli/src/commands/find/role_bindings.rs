use crate::collections::role_bindings::RoleBinding;
use crate::context::Context;
use crate::utils::print_items;
use clap::{ArgGroup, Parser};
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("filter"), about)]
pub(crate) struct FindRoleBindingOptions {
    /// Set name filter
    #[clap(short, long, group = "filter")]
    name: Option<String>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindRoleBindingOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let request = sdk::ListRoleBindingsRequest {
            filter: Some(self.filter_from_options()?),
            page: None,
        };
        let request = context.admin.sign_request(request).await?;
        let response = context.m10_client.list_role_bindings(request).await?;
        let items = response
            .role_bindings
            .into_iter()
            .map(RoleBinding::try_from)
            .collect::<Result<_, anyhow::Error>>()?;
        print_items(items, self.format)?;
        Ok(())
    }

    fn filter_from_options(
        &self,
    ) -> Result<sdk::list_role_bindings_request::Filter, anyhow::Error> {
        if let Some(name) = &self.name {
            return Ok(sdk::list_role_bindings_request::Filter::Name(name.into()));
        }
        Err(anyhow::anyhow!("missing filter"))
    }
}
