use std::fmt::Debug;

use clap::Subcommand;
use m10_sdk::{prost::Message, sdk, Format, PrettyPrint};
use serde::Serialize;

use crate::{collections::*, utils};

#[derive(Clone, Subcommand, Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Deserialize {
    /// Deserialize an account record
    #[command(alias = "am")]
    AccountMetadata,
    /// Deserialize an account set record
    #[command(alias = "as")]
    AccountSet,
    /// Deserialize a role record
    #[command(alias = "r")]
    Role,
    /// Deserialize a role bindings record
    #[command(alias = "rb")]
    RoleBinding,
}

impl Deserialize {
    pub(super) fn run(self, data: String, format: Format) -> anyhow::Result<()> {
        match self {
            Deserialize::AccountMetadata => {
                Self::print_data::<sdk::AccountMetadata, accounts::AccountMetadata>(data, format)?;
            }
            Deserialize::AccountSet => {
                Self::print_data::<sdk::AccountSet, account_sets::AccountSet>(data, format)?;
            }
            Deserialize::Role => {
                Self::print_data::<sdk::Role, roles::Role>(data, format)?;
            }
            Deserialize::RoleBinding => {
                Self::print_data::<sdk::RoleBinding, role_bindings::RoleBinding>(data, format)?;
            }
        }
        Ok(())
    }

    fn print_data<D, I>(data: String, format: Format) -> anyhow::Result<()>
    where
        D: Message + Default,
        I: TryFrom<D, Error = anyhow::Error> + Serialize,
    {
        let buf = utils::vec_from_int_array(&data)?;
        let item = D::decode(buf.as_slice())?;
        let printable = I::try_from(item)?;
        printable.print(format)?;
        Ok(())
    }
}
