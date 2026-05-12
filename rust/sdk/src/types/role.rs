use crate::collections::ResourceId;
use crate::error::M10Error;
use crate::types::PublicKey;
use m10_protos::sdk;
use m10_protos::sdk::Rule;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum Verb {
    Read,
    Create,
    Update,
    Delete,
    Transact,
    Initiate,
    Commit,
    Grant,
    Deny,
    Revoke,
}

impl From<sdk::rule::Verb> for Verb {
    fn from(value: sdk::rule::Verb) -> Self {
        match value {
            sdk::rule::Verb::Commit => Verb::Commit,
            sdk::rule::Verb::Create => Verb::Create,
            sdk::rule::Verb::Delete => Verb::Delete,
            sdk::rule::Verb::Initiate => Verb::Initiate,
            sdk::rule::Verb::Read => Verb::Read,
            sdk::rule::Verb::Update => Verb::Update,
            sdk::rule::Verb::Transact => Verb::Transact,
            sdk::rule::Verb::Grant => Verb::Grant,
            sdk::rule::Verb::Deny => Verb::Deny,
            sdk::rule::Verb::Revoke => Verb::Revoke,
        }
    }
}

impl From<Verb> for sdk::rule::Verb {
    fn from(verb: Verb) -> Self {
        match verb {
            Verb::Commit => sdk::rule::Verb::Commit,
            Verb::Create => sdk::rule::Verb::Create,
            Verb::Delete => sdk::rule::Verb::Delete,
            Verb::Initiate => sdk::rule::Verb::Initiate,
            Verb::Read => sdk::rule::Verb::Read,
            Verb::Update => sdk::rule::Verb::Update,
            Verb::Transact => sdk::rule::Verb::Transact,
            Verb::Grant => sdk::rule::Verb::Grant,
            Verb::Deny => sdk::rule::Verb::Deny,
            Verb::Revoke => sdk::rule::Verb::Revoke,
        }
    }
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("Role{{ id={id} owner={owner} name={name} rules={rules:?} created_at={created_at} updated_at={updated_at} created_by={created_by} description={description} labels={labels:?} }}")
)]
#[derive(Clone, Debug, Serialize)]
pub struct Role {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub name: String,
    pub rules: Vec<Rule>,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: PublicKey,
    pub description: String,
    pub labels: std::collections::HashMap<String, String>,
    pub immutable: bool,
}

impl TryFrom<sdk::Role> for Role {
    type Error = M10Error;

    fn try_from(role: sdk::Role) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(role.id.as_ref())?,
            owner: PublicKey(role.owner.to_vec()),
            name: role.name,
            rules: role.rules,
            created_at: role.created_at,
            updated_at: role.updated_at,
            created_by: PublicKey(role.created_by.to_vec()),
            description: role.description,
            labels: role.labels,
            immutable: role.immutable,
        })
    }
}
