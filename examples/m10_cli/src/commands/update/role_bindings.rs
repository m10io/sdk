use clap::Args;
use m10_sdk::{prost::bytes::Bytes, sdk, DocumentUpdate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::collections::role_bindings::Expression;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateRoleBindingArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[arg(short, long)]
    name: Option<String>,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
    /// Update role link
    #[arg(short, long)]
    role: Option<Uuid>,
    /// Add subjects
    #[arg(long, alias = "subjs")]
    subjects: Option<Vec<String>>,
    /// Update guard expression
    #[clap(short, long, alias = "exps")]
    expressions: Option<Expression>,
}

impl super::BuildFromArgs for UpdateRoleBindingArgs {
    type Document = sdk::RoleBinding;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(Bytes::from(owner_key));
        }
        if let Some(name) = self.name {
            builder.name(name);
        }
        if let Some(subjects) = self.subjects {
            let subjects = subjects
                .iter()
                .map(base64::decode)
                .collect::<Result<Vec<Vec<u8>>, _>>()?;
            builder.subjects(subjects);
        }
        if let Some(expressions) = self.expressions {
            builder.expressions(
                expressions
                    .0
                    .into_iter()
                    .map(|(collection, expression)| sdk::Expression {
                        collection,
                        expression,
                    })
                    .collect(),
            );
        }
        Ok(())
    }
}
