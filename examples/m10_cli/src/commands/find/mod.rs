use clap::Subcommand;
use m10_sdk::{Format, NameFilter, PageBuilder, PrettyPrint};
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod account_sets;
mod accounts;
mod actions;
mod directory_entry;
mod ledger_accounts;
mod transactions;
mod transfers;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Find {
    /// Find account record(s)
    #[command(alias = "a")]
    Accounts(accounts::FindAccountArgs),
    /// Find account set record(s)
    #[command(alias = "as")]
    AccountSets(account_sets::FindAccountSetArgs),
    /// Find actions
    /// (either by context or account)
    #[command(alias = "ac")]
    Actions(actions::FindActionArgs),
    /// Find banks
    #[command(alias = "b")]
    Banks {
        #[arg(short, long, default_value = "raw")]
        #[serde(default)]
        format: Format,
    },
    /// Find a directory entry
    #[command(alias = "d")]
    DirectoryEntries {
        #[command(subcommand)]
        cmd: directory_entry::DirEntry,
    },
    /// Find ledger account record(s)
    #[command(aliases = ["l", "la"])]
    LedgerAccounts(ledger_accounts::FindAccountArgs),
    /// Find role record(s)
    #[command(alias = "r")]
    Roles {
        /// Set name filter
        #[arg(short, long)]
        name: String,
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short, long, default_value = "raw")]
        #[serde(default)]
        format: Format,
    },
    /// Find role binding record(s)
    #[command(alias = "rb")]
    RoleBindings {
        /// Set name filter
        #[arg(short, long)]
        name: String,
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short, long, default_value_t)]
        format: Format,
    },
    /// Find transactions within a context
    #[command(alias = "txns")]
    Transactions(transactions::FindTransactionArgs),
    /// Find transfer(s)
    #[command(alias = "t")]
    Transfers(transfers::FindTransferArgs),
}

impl Find {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Find::Accounts(args) => {
                args.find(context).await?;
            }
            Find::AccountSets(args) => {
                args.find(context).await?;
            }
            Find::Actions(args) => args.find(context).await?,
            Find::Banks { format } => {
                context
                    .ledger_client()
                    .list_banks(PageBuilder::default())
                    .await?
                    .print(format)?;
            }
            Find::DirectoryEntries { cmd } => cmd.find(context).await?,
            Find::LedgerAccounts(args) => {
                args.find(context).await?;
            }
            Find::Roles { name, format } => {
                context
                    .ledger_client()
                    .list_roles(PageBuilder::<_, NameFilter>::name(name))
                    .await?
                    .print(format)?;
            }
            Find::RoleBindings { name, format } => {
                let builder = PageBuilder::<_, NameFilter>::name(name);
                context
                    .ledger_client()
                    .list_role_bindings(builder)
                    .await?
                    .print(format)?;
            }
            Find::Transactions(args) => args.find(context).await?,
            Find::Transfers(args) => {
                args.find(context).await?;
            }
        }
        Ok(())
    }
}
