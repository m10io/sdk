use std::{collections::HashMap, convert::TryFrom, str::FromStr};

use bytes::Bytes;
use clap::Parser;
use m10_sdk::sdk::{self, rule::Ty};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

use super::PrettyId;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct RuleArgs {
    #[clap(short = 'c', long = "collection")]
    pub(crate) collection: String,
    #[clap(short = 'v', long = "verbs")]
    pub(crate) verbs: Vec<Verb>,
    #[clap(long = "when")]
    pub(crate) when: Option<String>,
    #[clap(long = "types", value_parser = parse_type_entry)]
    pub(crate) types: Option<Vec<(String, Ty)>>,
    #[clap(short = 'i', long = "instances")]
    pub(crate) instances: Option<Vec<Uuid>>,
}

fn parse_type_entry(s: &str) -> Result<(String, Ty), anyhow::Error> {
    let (k, v) = s
        .split_once('=')
        .ok_or_else(|| anyhow::anyhow!("expected key=value, got '{s}'"))?;
    let ty = Ty::from_str_name(v).ok_or_else(|| anyhow::anyhow!("unknown type: '{v}'"))?;
    Ok((k.to_string(), ty))
}

impl RuleArgs {
    pub(crate) fn to_rbac_rule(&self) -> sdk::Rule {
        let instance_keys = self.instances.as_ref().map_or(vec![], |i| {
            i.iter()
                .map(|i| Bytes::copy_from_slice(i.as_bytes()).into())
                .collect()
        });

        let collection = self.collection.to_owned();
        let verbs = self.verbs.iter().map(|v| *v as i32).collect::<Vec<i32>>();
        let when = self.when.clone();
        let types = if let Some(types) = &self.types {
            types
                .iter()
                .map(|(name, ty)| (name.clone(), *ty as i32))
                .collect::<HashMap<String, i32>>()
        } else {
            HashMap::new()
        };

        sdk::Rule {
            collection,
            instance_keys,
            verbs,
            when,
            types,
        }
    }
}

impl FromStr for RuleArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = shlex::split(s).ok_or_else(|| anyhow::anyhow!("failed to parse rule args"))?;
        let rule = RuleArgs::try_parse_from(args)?;
        Ok(rule)
    }
}

// Note: Arcadius types are not implementing Serialize/Deserialize.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) enum Verb {
    Read = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
    Transact = 4,
    Initiate = 5,
    Commit = 6,
    Grant = 7,
    Deny = 8,
    Revoke = 9,
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
            7 => Ok(Verb::Grant),
            8 => Ok(Verb::Deny),
            9 => Ok(Verb::Revoke),
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
            "Grant" => Ok(Verb::Grant),
            "Deny" => Ok(Verb::Deny),
            "Revoke" => Ok(Verb::Revoke),
            _ => Err("no match"),
        }
    }
}

#[derive(Serialize, Deserialize, parse_display::Display, Debug, Clone, Default)]
#[display("Rule{{ collection={collection} instance_keys={instance_keys:?} verbs={verbs:?} when={when} types={types:?}}}")]
pub struct Rule {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub collection: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_keys: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub when: String,
    #[serde(default)]
    pub types: HashMap<String, Ty>,
}

impl TryFrom<m10_sdk::ledger::Rule> for Rule {
    type Error = anyhow::Error;

    fn try_from(other: m10_sdk::ledger::Rule) -> Result<Rule, Self::Error> {
        let m10_sdk::ledger::Rule {
            collection,
            instance_keys,
            verbs,
            when,
            types,
        } = other;

        let converted_verbs = verbs
            .iter()
            .map(|&t| {
                Verb::try_from(t)
                    .map(|v| format!("{:?}", v))
                    .map_err(|e| anyhow::anyhow!("Failed to convert verb: {}", e))
            })
            .collect::<Result<_, anyhow::Error>>()?;

        let converted_instance_keys = instance_keys
            .into_iter()
            .map(|v| {
                if let Some(m10_sdk::sdk::value::Value::BytesValue(bytes)) = v.value {
                    hex::encode(bytes)
                } else {
                    "<non-bytes-instance>".to_string()
                }
            })
            .collect::<Vec<_>>();

        let converted_when = when.unwrap_or_default();

        let converted_types = types
            .iter()
            .map(|(name, ty)| {
                Ok((
                    name.clone(),
                    Ty::try_from(*ty).map_err(|e| anyhow::anyhow!("unexpected Ty: {e}"))?,
                ))
            })
            .collect::<Result<HashMap<String, Ty>, anyhow::Error>>()?;

        Ok(Rule {
            collection,
            instance_keys: converted_instance_keys,
            verbs: converted_verbs,
            when: converted_when,
            types: converted_types,
        })
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Role {
    #[serde_as(as = "DisplayFromStr")]
    pub id: PrettyId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub created_by: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<Rule>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub immutable: bool,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

fn format_timestamp(timestamp_us: u64) -> String {
    let secs = timestamp_us / 1_000_000;
    let nanos = ((timestamp_us % 1_000_000) * 1_000) as u32;
    let datetime = chrono::DateTime::from_timestamp(secs as i64, nanos).unwrap();
    datetime.format("%Y-%m-%dT%H:%M:%S UTC").to_string()
}

impl TryFrom<m10_sdk::ledger::Role> for Role {
    type Error = anyhow::Error;

    fn try_from(other: m10_sdk::ledger::Role) -> Result<Role, Self::Error> {
        let m10_sdk::ledger::Role {
            id,
            owner,
            name,
            mut rules,
            created_at,
            updated_at,
            created_by,
            description,
            immutable,
            labels,
        } = other;
        Ok(Role {
            id: PrettyId::from(id),
            name,
            description,
            owner: base64::encode(owner),
            created_by: base64::encode(created_by),
            rules: rules
                .drain(..)
                .map(Rule::try_from)
                .collect::<Result<_, _>>()?,
            created_at: format_timestamp(created_at),
            updated_at: format_timestamp(updated_at),
            labels,
            immutable,
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
            created_at,
            updated_at,
            created_by,
            description,
            immutable,
            labels,
        } = other;
        Ok(Role {
            id: PrettyId::from(Bytes::from(id)),
            name,
            description,
            owner: base64::encode(owner.to_vec()),
            created_by: base64::encode(created_by.to_vec()),
            rules: rules
                .drain(..)
                .map(Rule::try_from)
                .collect::<Result<_, _>>()?,
            created_at: format_timestamp(created_at),
            updated_at: format_timestamp(updated_at),
            labels,
            immutable,
        })
    }
}
