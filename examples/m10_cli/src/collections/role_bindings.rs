use std::{collections::HashMap, convert::TryFrom, str::FromStr};

use m10_sdk::sdk;
use serde_with::{serde_as, DisplayFromStr};

use super::PrettyId;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub(crate) struct Expression(pub(crate) HashMap<String, String>);

impl FromStr for Expression {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(serde_json::from_str(s)?))
    }
}

#[serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RoleBinding {
    #[serde_as(as = "DisplayFromStr")]
    pub id: PrettyId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub created_by: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "role_id")]
    pub role: PrettyId,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<String>,
    pub is_universal: bool,
    pub created_at: String,
    pub updated_at: String,
    pub expressions: HashMap<String, String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub expires_at: String,
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}

fn format_timestamp_ms(timestamp_ms: u64) -> String {
    let secs = timestamp_ms / 1_000;
    let nanos = ((timestamp_ms % 1_000) * 1_000_000) as u32;
    let datetime = chrono::DateTime::from_timestamp(secs as i64, nanos).unwrap();
    datetime.format("%Y-%m-%dT%H:%M:%S UTC").to_string()
}

fn format_timestamp(timestamp_us: u64) -> String {
    let secs = timestamp_us / 1_000_000;
    let nanos = ((timestamp_us % 1_000_000) * 1_000) as u32;
    let datetime = chrono::DateTime::from_timestamp(secs as i64, nanos).unwrap();
    datetime.format("%Y-%m-%dT%H:%M:%S UTC").to_string()
}

impl TryFrom<sdk::RoleBinding> for RoleBinding {
    type Error = anyhow::Error;

    fn try_from(other: sdk::RoleBinding) -> Result<RoleBinding, Self::Error> {
        let sdk::RoleBinding {
            id,
            name,
            owner,
            role,
            subjects,
            expressions,
            is_universal,
            created_at,
            updated_at,
            created_by,
            description,
            expires_at,
            labels,
        } = other;

        Ok(RoleBinding {
            id: PrettyId::from(id),
            name,
            description,
            owner: base64::encode(&owner),
            created_by: base64::encode(&created_by),
            role: PrettyId::from(role),
            subjects: subjects.iter().map(base64::encode).collect(),
            is_universal,
            created_at: format_timestamp(created_at),
            updated_at: format_timestamp(updated_at),
            expressions: expressions
                .into_iter()
                .map(|expression| (expression.collection, expression.expression))
                .collect(),
            expires_at: expires_at.map(format_timestamp_ms).unwrap_or_default(),
            labels,
        })
    }
}

impl TryFrom<m10_sdk::RoleBinding> for RoleBinding {
    type Error = anyhow::Error;

    fn try_from(other: m10_sdk::RoleBinding) -> Result<RoleBinding, Self::Error> {
        Ok(RoleBinding {
            id: PrettyId::from(bytes::Bytes::from(other.id)),
            name: other.name,
            description: other.description,
            owner: base64::encode(other.owner.0),
            created_by: base64::encode(other.created_by.0),
            role: PrettyId::from(bytes::Bytes::from(other.role_id)),
            subjects: other.subjects.iter().map(base64::encode).collect(),
            is_universal: other.is_universal,
            created_at: format_timestamp(other.created_at),
            updated_at: format_timestamp(other.updated_at),
            expressions: other
                .expressions
                .into_iter()
                .map(|expression| (expression.collection, expression.expression))
                .collect(),
            labels: other.labels,
            expires_at: other
                .expires_at
                .map(format_timestamp_ms)
                .unwrap_or_default(),
        })
    }
}
