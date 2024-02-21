use std::convert::TryInto;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
pub(crate) enum Account {
    /// Show parent Id of an account
    ParentId,
    /// Show if account is an issuance account
    IsIssuance,
    /// Show if account is descendant of another account
    IsDescendantOf {
        /// Account Id
        parent: String,
    },
    /// Show if account is
    IsEqOrDescendantOf {
        /// Account Id
        parent: String,
    },
    /// Show if account is a leaf in the hierarchy
    IsLeaf,
    /// Show account level in the hierarchy
    Depth,
}

impl Account {
    pub(super) fn run(self, id: String) -> anyhow::Result<()> {
        match self {
            Account::ParentId => {
                let id = Self::try_convert(&id)?;
                if let Some(parent_id) = id.parent_id() {
                    let raw_id = parent_id.to_be_bytes();
                    println!("{}", hex::encode(raw_id));
                } else {
                    eprintln!("account is root");
                }
            }
            Account::IsIssuance => {
                let id = Self::try_convert(&id)?;
                println!("{}", id.is_issuance());
            }
            Account::IsLeaf => {
                let id = Self::try_convert(&id)?;
                println!("{}", id.is_leaf());
            }
            Account::Depth => {
                let id = Self::try_convert(&id)?;
                println!("{}", id.depth());
            }
            Account::IsDescendantOf { parent } => {
                let child = Self::try_convert(&id)?;
                let parent = Self::try_convert(&parent)?;
                println!("{}", child.is_descendant_of(parent));
            }
            Account::IsEqOrDescendantOf { parent } => {
                let child = Self::try_convert(&id)?;
                let parent = Self::try_convert(&parent)?;
                println!("{}", child.is_eq_or_descendant_of(parent));
            }
        }
        Ok(())
    }

    fn try_convert(id: &str) -> Result<m10_sdk::account::AccountId, anyhow::Error> {
        let bytes = hex::decode(id)?;
        let raw_id = u128::from_be_bytes((&bytes[0..16]).try_into()?);
        Ok(m10_sdk::account::AccountId::from_raw(raw_id)?)
    }
}
