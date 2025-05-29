use std::{convert::TryFrom, str::FromStr};

use bytes::Bytes;
use clap::Parser;
use m10_sdk::sdk::{self, Value};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

use super::PrettyId;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct RuleArgs {
    #[clap(short, long)]
    instances: Option<Vec<Uuid>>,
    #[clap(short = 'x', long = "excluded-instances")]
    excluded_instances: Option<Vec<Uuid>>,
    #[clap(short, long)]
    collection: String,
    #[clap(short, long)]
    verbs: Vec<Verb>,
}

impl RuleArgs {
    pub(crate) fn to_rbac_rule(&self) -> sdk::Rule {
        let instance_keys = self.instances.as_ref().map_or(vec![], |i| {
            i.iter()
                .map(|i| Bytes::copy_from_slice(i.as_bytes()).into())
                .collect()
        });

        let excluded_instance_keys = self.excluded_instances.as_ref().map_or(vec![], |i| {
            i.iter()
                .map(|i| Bytes::copy_from_slice(i.as_bytes()).into())
                .collect()
        });

        let collection = self.collection.to_owned();
        let verbs = self.verbs.iter().map(|v| *v as i32).collect::<Vec<i32>>();

        sdk::Rule {
            collection,
            instance_keys,
            excluded_instance_keys,
            verbs,
        }
    }
}

impl FromStr for RuleArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_ascii_whitespace();
        let rule = RuleArgs::try_parse_from(args)?;
        Ok(rule)
    }
}

// Note: Arcadius types are not implementing Serialize/Deserialize.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Verb {
    Read = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
    Transact = 4,
    Initiate = 5,
    Commit = 6,
}

impl TryFrom<i32> for Verb {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Verb::Read),
            1 => Ok(Verb::Create),
            2 => Ok(Verb::Update),
            3 => Ok(Verb::Delete),
            4 => Ok(Verb::Transact),
            5 => Ok(Verb::Initiate),
            6 => Ok(Verb::Commit),
            _ => Err(anyhow::anyhow!("Unknown verb: {}", value)),
        }
    }
}

impl FromStr for Verb {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Read" => Ok(Verb::Read),
            "Create" => Ok(Verb::Create),
            "Update" => Ok(Verb::Update),
            "Delete" => Ok(Verb::Delete),
            "Transact" => Ok(Verb::Transact),
            "Initiate" => Ok(Verb::Initiate),
            "Commit" => Ok(Verb::Commit),
            _ => Err("no match"),
        }
    }
}

#[derive(Serialize, Deserialize, parse_display::Display, Debug, Clone, Default)]
#[display("Rule{{ collection={collection} instance_keys={instance_keys:?} excluded_instance_keys={excluded_instance_keys:?} verbs={verbs:?} }}")]
pub struct Rule {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub collection: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_keys: Vec<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_instance_keys: Vec<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
}

impl TryFrom<m10_sdk::ledger::Rule> for Rule {
    type Error = anyhow::Error;

    fn try_from(other: m10_sdk::ledger::Rule) -> Result<Rule, Self::Error> {
        let m10_sdk::ledger::Rule {
            collection,
            instance_keys,
            excluded_instance_keys,
            verbs,
        } = other;

        let converted_verbs = verbs
            .iter()
            .map(|&t| {
                Verb::try_from(t)
                    .map(|v| format!("{:?}", v))
                    .map_err(|e| anyhow::anyhow!("Failed to convert verb: {}", e))
            })
            .collect::<Result<_, anyhow::Error>>()?;

        Ok(Rule {
            collection,
            instance_keys,
            excluded_instance_keys,
            verbs: converted_verbs,
        })
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Role {
    #[serde_as(as = "DisplayFromStr")]
    pub id: PrettyId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<Rule>,
}

impl TryFrom<m10_sdk::ledger::Role> for Role {
    type Error = anyhow::Error;

    fn try_from(other: m10_sdk::ledger::Role) -> Result<Role, Self::Error> {
        let m10_sdk::ledger::Role {
            id,
            owner,
            name,
            mut rules,
            ..
        } = other;
        Ok(Role {
            id: PrettyId::from(id),
            owner: base64::encode(owner),
            name,
            rules: rules
                .drain(..)
                .map(Rule::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<m10_sdk::Role> for Role {
    type Error = anyhow::Error;

    fn try_from(other: m10_sdk::Role) -> Result<Role, Self::Error> {
        let m10_sdk::Role {
            id,
            owner,
            name,
            mut rules,
            ..
        } = other;
        Ok(Role {
            id: PrettyId::from(Bytes::from(id)),
            owner: base64::encode(owner.to_vec()),
            name,
            rules: rules
                .drain(..)
                .map(Rule::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}
