use clap::Parser;
use m10_sdk::DocumentUpdate;
use m10_sdk::{prost::bytes::Bytes, sdk};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateRoleBindingOptions {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[clap(short, long)]
    name: Option<String>,
    /// Update owner field
    #[clap(short, long)]
    owner: Option<String>,
    /// Update role link
    #[clap(short, long)]
    role: Option<Uuid>,
    /// Add subjects
    #[clap(short, long, multiple_values = true)]
    subjects: Option<Vec<String>>,
    /// Update guard expression
    #[clap(short, long, multiple_values = true, parse(try_from_str = parse_key_vals))]
    expressions: Option<HashMap<String, String>>,
}

fn parse_key_vals(s: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    serde_json::from_str(s)
}

impl super::BuildFromOptions for UpdateRoleBindingOptions {
    type Document = sdk::RoleBinding;

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
        if let Some(subjects) = &self.subjects {
            let subjects = subjects
                .iter()
                .map(base64::decode)
                .collect::<Result<Vec<Vec<u8>>, _>>()?;
            builder.subjects(subjects);
        }
        if let Some(expressions) = &self.expressions {
            builder.expressions(
                expressions
                    .iter()
                    .map(|(collection, expression)| sdk::Expression {
                        collection: collection.clone(),
                        expression: expression.clone(),
                    })
                    .collect(),
            );
        }
        Ok(())
    }
}
