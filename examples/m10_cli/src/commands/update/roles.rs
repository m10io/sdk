use crate::collections::roles::RuleOptions;
use clap::Parser;
use m10_sdk::DocumentUpdate;
use m10_sdk::{prost::bytes::Bytes, sdk};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateRoleOptions {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[clap(short, long)]
    name: Option<String>,
    /// Update owner field
    #[clap(short, long)]
    owner: Option<String>,
    /// Add rule (1..N)
    #[clap(short, long, required = true, parse(try_from_str = RuleOptions::parse))]
    rule: Vec<RuleOptions>,
}

impl super::BuildFromOptions for UpdateRoleOptions {
    type Document = sdk::Role;

    fn build_from_options(
        &self,
        builder: &mut DocumentUpdate<Self::Document>,
    ) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(Bytes::from(owner_key));
        }
        if let Some(name) = &self.name {
            builder.name(name.into());
        }
        if !self.rule.is_empty() {
            let rules = self.rule.iter().map(|r| r.to_rbac_rule()).collect();
            builder.rules(rules);
        }
        Ok(())
    }
}
