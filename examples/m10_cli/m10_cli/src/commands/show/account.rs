use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::Debug;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) enum ShowAccountCommands {
    /// Show parent Id of an account
    ParentId(AccountId),
    /// Show if account is an issuance account
    IsIssuance(AccountId),
    /// Show if account is descendant of another account
    IsDescendantOf(AccountIds),
    /// Show if account is
    IsEqOrDescendantOf(AccountIds),
    /// Show if account is a leaf in the hierarchy
    IsLeaf(AccountId),
    /// Show account level in the hierarchy
    Depth(AccountId),
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct AccountIds {
    /// Account Id potentially lower in hierarchy
    #[clap(short, long)]
    child: String,
    /// Account Id potentially higher in hierarchy
    #[clap(short, long)]
    parent: String,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct AccountId {
    /// Account Id
    id: String,
}

impl ShowAccountCommands {
    pub(super) fn show(&self) -> anyhow::Result<()> {
        match self {
            ShowAccountCommands::ParentId(option) => {
                let id = Self::try_convert(&option.id)?;
                if let Some(parent_id) = id.parent_id() {
                    let raw_id = parent_id.to_be_bytes();
                    println!("{}", hex::encode(raw_id));
                } else {
                    eprintln!("account is root");
                }
            }
            ShowAccountCommands::IsIssuance(option) => {
                let id = Self::try_convert(&option.id)?;
                println!("{}", id.is_issuance());
            }
            ShowAccountCommands::IsLeaf(option) => {
                let id = Self::try_convert(&option.id)?;
                println!("{}", id.is_leaf());
            }
            ShowAccountCommands::Depth(option) => {
                let id = Self::try_convert(&option.id)?;
                println!("{}", id.depth());
            }
            ShowAccountCommands::IsDescendantOf(option) => {
                let child = Self::try_convert(&option.child)?;
                let parent = Self::try_convert(&option.parent)?;
                println!("{}", child.is_descendant_of(parent));
            }
            ShowAccountCommands::IsEqOrDescendantOf(option) => {
                let child = Self::try_convert(&option.child)?;
                let parent = Self::try_convert(&option.parent)?;
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
