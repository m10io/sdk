use crate::collections::ResourceId;
use crate::error::M10Error;
use crate::types::PublicKey;
use m10_protos::sdk;
use m10_protos::sdk::Expression;
use serde::Serialize;

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("RoleBinding{{ id={id} owner={owner} name={name} role_id={role_id} is_universal={is_universal} created_at={created_at} updated_at={updated_at} created_by={created_by} description={description} expires_at={expires_at:?} labels={labels:?} }}")
)]
#[derive(Clone, Debug, Serialize)]
pub struct RoleBinding {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub name: String,
    pub role_id: ResourceId,
    pub subjects: Vec<bytes::Bytes>,
    pub expressions: Vec<Expression>,
    pub is_universal: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: PublicKey,
    pub description: String,
    pub labels: std::collections::HashMap<String, String>,
    pub expires_at: Option<u64>,
}

impl TryFrom<sdk::RoleBinding> for RoleBinding {
    type Error = M10Error;

    fn try_from(role_binding: sdk::RoleBinding) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(role_binding.id.as_ref())?,
            owner: PublicKey(role_binding.owner.to_vec()),
            name: role_binding.name,
            role_id: ResourceId::try_from(role_binding.role.as_ref())?,
            subjects: role_binding.subjects,
            expressions: role_binding.expressions,
            is_universal: role_binding.is_universal,
            created_at: role_binding.created_at,
            updated_at: role_binding.updated_at,
            created_by: PublicKey(role_binding.created_by.to_vec()),
            description: role_binding.description,
            labels: role_binding.labels,
            expires_at: role_binding.expires_at,
        })
    }
}
