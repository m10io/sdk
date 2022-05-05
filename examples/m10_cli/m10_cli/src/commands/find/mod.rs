use crate::commands::top_level_cmds::Format;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod account_sets;
mod accounts;
mod actions;
mod directory_entry;
mod role_bindings;
mod roles;
mod transactions;
mod transfers;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum FindSubCommands {
    /// Find account record(s)
    Account(accounts::FindAccountOptions),
    /// Find account set record(s)
    AccountSet(account_sets::FindAccountSetOptions),
    /// Find actions
    /// (either by context or account)
    Action(actions::FindActionOptions),
    /// Find role record(s)
    Role(roles::FindRoleOptions),
    /// Find role binding record(s)
    RoleBinding(role_bindings::FindRoleBindingOptions),
    /// Find transfer(s)
    Transfer(transfers::FindTransferOptions),
    /// Find a directory entry
    DirectoryEntry(directory_entry::FindDirEntryOptions),
    /// Find transactions within a context
    Transactions(transactions::FindTransactionOptions),
}

impl FindSubCommands {
    pub(super) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            FindSubCommands::Account(options) => {
                options.find(config).await?;
            }
            FindSubCommands::AccountSet(options) => {
                options.find(config).await?;
            }
            FindSubCommands::Role(options) => {
                options.find(config).await?;
            }
            FindSubCommands::RoleBinding(options) => {
                options.find(config).await?;
            }
            FindSubCommands::Transfer(options) => {
                options.find(config).await?;
            }
            FindSubCommands::DirectoryEntry(options) => options.find().await?,
            FindSubCommands::Action(options) => options.find(config).await?,
            FindSubCommands::Transactions(options) => options.find(config).await?,
        }
        Ok(())
    }
}
