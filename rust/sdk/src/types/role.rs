use crate::collections::ResourceId;
use crate::error::M10Error;
use crate::types::PublicKey;
use m10_protos::sdk;
use m10_protos::sdk::Rule;

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("Role{{ id={id} owner={owner} name={name} rules={rules:?} }}")
)]
#[derive(Clone, Debug)]
pub struct Role {
    pub id: ResourceId,
    pub owner: PublicKey,
    pub name: String,
    pub rules: Vec<Rule>,
}

impl TryFrom<sdk::Role> for Role {
    type Error = M10Error;

    fn try_from(role: sdk::Role) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ResourceId::try_from(role.id.as_ref())?,
            owner: PublicKey(role.owner.to_vec()),
            name: role.name,
            rules: role.rules,
        })
    }
}
