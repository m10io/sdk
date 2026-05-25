use clap::Args;
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

use crate::collections::{
    role_bindings::{Attribute, AttributeValue, Expression},
    PrettyId,
};
use crate::utils::{parse_key_value, validate_labels};

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
    #[arg(short = 'd', long)]
    description: Option<String>,
    /// Set expiry time in RFC3339 format (YYYY-MM-DDTHH:MM:SSZ).
    /// If set, the binding is rejected after this time
    #[arg(long)]
    #[serde(default)]
    expires_at: Option<String>,
    /// Set optional labels. (e.g. `-l label_1=value_1 -l label_2=value_2`)
    #[arg(short = 'l', long, value_parser = parse_key_value)]
    #[serde(default)]
    pub labels: Option<Vec<(String, String)>>,
    /// Set attributes (may be used in expressions)
    #[arg(long, alias = "attributes")]
    #[serde(default)]
    pub attributes: Option<Attribute>,
}

impl From<AttributeValue> for sdk::attribute::AttributeValue {
    fn from(v: AttributeValue) -> Self {
        use sdk::attribute::attribute_value::Value;
        sdk::attribute::AttributeValue {
            value: Some(match v {
                AttributeValue::Uint(n) => Value::UintValue(n),
                AttributeValue::Int(n) => Value::IntValue(n),
                AttributeValue::Float(f) => Value::FloatValue(f),
                AttributeValue::Bool(b) => Value::BoolValue(b),
                AttributeValue::String(s) => Value::StringValue(s),
            }),
        }
    }
}

impl super::BuildFromArgs for CreateRoleBindingArgs {
    type Document = sdk::RoleBinding;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.unwrap_or(default_owner.clone()).0;
        let signer_key = default_owner.0;
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

        let attributes = self.attributes.map_or(vec![], |attrs| {
            attrs
                .0
                .into_iter()
                .map(|(name, value)| sdk::Attribute {
                    name,
                    value: Some(value.into()),
                })
                .collect()
        });

        let description = self.description.unwrap_or_default();
        if description.len() > 100 {
            return Err(anyhow::anyhow!(
                "Description must be 100 characters or less"
            ));
        }

        let labels: std::collections::HashMap<String, String> =
            self.labels.unwrap_or_default().into_iter().collect();
        validate_labels(&labels)?;

        let expires_at = self
            .expires_at
            .map(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Invalid expires_at format (expected RFC3339 YYYY-MM-DDTHH:MM:SSZ): {}",
                            e
                        )
                    })
                    .map(|dt| dt.timestamp_millis() as u64)
            })
            .transpose()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        if expires_at.is_some_and(|ts| ts <= now.as_millis() as u64) {
            return Err(anyhow::anyhow!("expires_at must be in the future"));
        }
        let timestamp = now.as_micros() as u64;
        Ok(sdk::RoleBinding {
            id: id.into(),
            name: self.name,
            owner: owner.into(),
            role: self.role.into(),
            subjects,
            expressions,
            is_universal: self.is_universal,
            created_at: timestamp,
            updated_at: timestamp,
            created_by: signer_key.into(),
            description,
            labels,
            expires_at,
            attributes,
        })
    }
}
