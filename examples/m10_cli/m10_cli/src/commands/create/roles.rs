use crate::collections::roles::RuleOptions;
use clap::Parser;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateRoleOptions {
    /// Ignore error if item exists
    #[clap(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[clap(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set name of role
    #[clap(short, long, default_value_t)]
    #[serde(default)]
    name: String,
    /// Set owner of the role record
    #[clap(short, long)]
    owner: Option<String>,
    /// Set rule (1..N)
    #[clap(short, long, required = true, parse(try_from_str = RuleOptions::parse))]
    rule: Vec<RuleOptions>,
}

impl super::BuildFromOptions for CreateRoleOptions {
    type Document = sdk::Role;
    fn build_from_options(&self, default_owner: Vec<u8>) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self
            .owner
            .as_ref()
            .map_or::<Result<Vec<u8>, _>, _>(Ok(default_owner), base64::decode)?;
        let rules = self.rule.iter().map(|r| r.to_rbac_rule()).collect();
        Ok(sdk::Role {
            id: id.into(),
            owner: owner.into(),
            name: self.name.clone(),
            rules,
        })
    }
}
