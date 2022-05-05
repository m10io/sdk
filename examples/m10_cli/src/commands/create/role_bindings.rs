use clap::Parser;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{collections::HashMap, fmt::Debug};
use uuid::Uuid;

use crate::collections::PrettyId;

#[serde_as]
#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateRoleBindingOptions {
    /// Ignore error if item exists
    #[clap(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[clap(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set role binding name
    #[clap(short, long, default_value_t)]
    #[serde(default)]
    name: String,
    /// Set owner of the role record
    #[clap(short, long)]
    owner: Option<String>,
    /// Link role binding to a role record
    #[clap(short, long)]
    #[serde_as(as = "DisplayFromStr")]
    role: PrettyId,
    /// Set subjects
    #[clap(short, long, multiple_values = true)]
    #[serde(default)]
    subjects: Vec<String>,
    /// Set extra guard expressions
    #[clap(
        short = 'g',
        long,
        parse(try_from_str = parse_key_vals)
    )]
    expressions: Option<HashMap<String, String>>,
    /// Set role binding to 'universal'
    #[clap(short = 'u', long)]
    is_universal: bool,
}

fn parse_key_vals(s: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    serde_json::from_str(s)
}

impl super::BuildFromOptions for CreateRoleBindingOptions {
    type Document = sdk::RoleBinding;
    fn build_from_options(&self, default_owner: Vec<u8>) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self
            .owner
            .as_ref()
            .map_or::<Result<Vec<u8>, _>, _>(Ok(default_owner), base64::decode)?;
        let subjects = self
            .subjects
            .iter()
            .map(base64::decode)
            .collect::<Result<Vec<Vec<u8>>, _>>()?
            .into_iter()
            .map(bytes::Bytes::from)
            .collect();
        let expressions = self.expressions.as_ref().map_or(vec![], |exps| {
            exps.clone()
                .into_iter()
                .map(|(collection, expression)| sdk::Expression {
                    collection,
                    expression,
                })
                .collect()
        });
        Ok(sdk::RoleBinding {
            id: id.into(),
            name: self.name.clone(),
            owner: owner.into(),
            role: self.role.clone().into(),
            subjects,
            expressions,
            is_universal: self.is_universal,
        })
    }
}
