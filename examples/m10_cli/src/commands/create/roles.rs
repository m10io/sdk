use clap::Args;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::collections::roles::RuleArgs;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateRoleArgs {
    /// Ignore error if item exists
    #[arg(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set record uuid
    #[arg(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set name of role
    #[arg(short, long, default_value_t)]
    #[serde(default)]
    name: String,
    /// Set owner of the role record
    #[arg(short, long)]
    owner: Option<PublicKey>,
    /// Set rule.
    #[arg(
        short,
        long,
        required = true,
        long_help = "Rules include --collections (-c), --verbs (-v) and optionally, --instances (-i). Default collections include ledger-accounts (aka “account”), account-metadata, roles and role-bindings. Available verbs include Read, Create, Update, Delete, Transact, Initiate, and Commit. Instances take the argument of account-metadata ID in uuid format. An option key has one argument only. E.g.  *-r 'rule -c roles -v Read -v Update -v Delete'*"
    )]
    rule: Vec<RuleArgs>,
}

impl super::BuildFromArgs for CreateRoleArgs {
    type Document = sdk::Role;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.unwrap_or(default_owner).0;
        let rules = self.rule.iter().map(|r| r.to_rbac_rule()).collect();
        Ok(sdk::Role {
            id: id.into(),
            owner: owner.into(),
            name: self.name,
            rules,
        })
    }
}
