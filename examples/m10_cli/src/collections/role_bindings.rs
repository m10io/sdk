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
    #[serde_as(as = "DisplayFromStr")]
    pub role: PrettyId,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<String>,
    pub expressions: HashMap<String, String>,
    pub is_universal: bool,
}

impl TryFrom<sdk::RoleBinding> for RoleBinding {
    type Error = anyhow::Error;

    fn try_from(other: sdk::RoleBinding) -> Result<RoleBinding, Self::Error> {
        let sdk::RoleBinding {
            id,
            name,
            role,
            subjects,
            expressions,
            is_universal,
            ..
        } = other;

        Ok(RoleBinding {
            id: PrettyId::from(id),
            name,
            role: PrettyId::from(role),
            subjects: subjects.iter().map(base64::encode).collect(),
            expressions: expressions
                .into_iter()
                .map(|expression| (expression.collection, expression.expression))
                .collect(),
            is_universal,
        })
    }
}
