use clap::Args;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

use crate::collections::{role_bindings::Expression, PrettyId};

#[serde_as]
#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateRoleBindingArgs {
    /// Ignore error if item exists
    #[arg(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[arg(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set role binding name
    #[arg(short, long, default_value_t)]
    #[serde(default)]
    name: String,
    /// Set owner of the role record
    #[arg(short, long)]
    owner: Option<PublicKey>,
    /// Link role binding to a role record
    #[arg(short, long)]
    #[serde_as(as = "DisplayFromStr")]
    role: PrettyId,
    /// Set subject (public key)
    #[arg(long, alias = "subjs")]
    #[serde(default)]
    subject: Vec<String>,
    /// Set extra guard expressions (MQL4 syntax)
    #[arg(short = 'g', long, alias = "exps")]
    expressions: Option<Expression>,
    /// Sets role binding to be used by any public key. Default: False
    #[arg(short = 'u', long, alias = "universal")]
    is_universal: bool,
}

impl super::BuildFromArgs for CreateRoleBindingArgs {
    type Document = sdk::RoleBinding;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.unwrap_or(default_owner).0;
        let subjects = self
            .subject
            .iter()
            .map(base64::decode)
            .collect::<Result<Vec<Vec<u8>>, _>>()?
            .into_iter()
            .map(bytes::Bytes::from)
            .collect();
        let expressions = self.expressions.map_or(vec![], |exps| {
            exps.0
                .into_iter()
                .map(|(collection, expression)| sdk::Expression {
                    collection,
                    expression,
                })
                .collect()
        });
        Ok(sdk::RoleBinding {
            id: id.into(),
            name: self.name,
            owner: owner.into(),
            role: self.role.into(),
            subjects,
            expressions,
            is_universal: self.is_universal,
        })
    }
}
