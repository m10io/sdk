use clap::Args;
use m10_sdk::{prost::bytes::Bytes, sdk, DocumentUpdate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::collections::roles::RuleArgs;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateRoleArgs {
    /// Record id
    pub(super) id: Uuid,
    /// Update name field
    #[arg(short, long)]
    name: Option<String>,
    /// Update owner field
    #[arg(short, long)]
    owner: Option<String>,
    /// Set rule
    #[arg(
        short,
        long,
        required = true,
        long_help = "IMPORTANT: When updating one or more rules for a role, ALL rules and their verbs must be entered, even those that don't change. Rules include --collections (-c), --verbs (-v) and optionally, --instances (-i). Default collections include ledger-accounts (aka “account”), account-metadata, roles and role-bindings. Available verbs include Read, Create, Update, Delete, Transact, Initiate, and Commit. Instances take the argument of account-metadata ID in uuid format. An option key has one argument only. E.g.  *-r 'rule -c roles -v Read -v Update -v Delete'*"
    )]
    rule: Vec<RuleArgs>,
}

impl super::BuildFromArgs for UpdateRoleArgs {
    type Document = sdk::Role;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(Bytes::from(owner_key));
        }
        if let Some(name) = self.name {
            builder.name(name);
        }
        if !self.rule.is_empty() {
            let rules = self.rule.iter().map(|r| r.to_rbac_rule()).collect();
            builder.rules(rules);
        }
        builder.merge_repeated(true);
        Ok(())
    }
}
