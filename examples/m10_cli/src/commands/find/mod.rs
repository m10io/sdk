use std::collections::HashSet;

use clap::Subcommand;
use m10_sdk::{Format, NameFilter, PageBuilder, PrettyPrint};
use serde::{Deserialize, Serialize};

use crate::{collections::roles::Role, context::Context};

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
    /// Find ledger account record(s)
    #[command(aliases = ["l", "la"])]
    Accounts(ledger_accounts::FindAccountArgs),
    /// Find account metadata record(s)
    #[command(alias = "a")]
    AccountMetadata(accounts::FindAccountArgs),
    /// Find account set record(s)
    #[command(alias = "as")]
    AccountSets(account_sets::FindAccountSetArgs),
    /// Find actions
    /// (either by context or ledger account)
    #[command(
        alias = "ac",
        help_template = "\
    {before-help}{name} {version}
    {about-with-newline}
    {usage-heading}
        \x1b[1mm10 find actions\x1b[0m [OPTIONS] \
                    \x1b[1m--account\x1b[0m <ACCOUNT>
    {all-args}{after-help}"
    )]
    Actions(actions::FindActionArgs),
    /// List balances based on a list of accounts ids
    /// piped to stdin in the given format
    #[command(aliases = ["ab", "bal"])]
    Balances {
        #[arg(short, long, default_value = "csv")]
        #[serde(default)]
        format: Format,
    },
    /// Find banks
    #[command(alias = "b")]
    Banks {
        #[arg(short, long, default_value = "raw")]
        #[serde(default)]
        format: Format,
    },
    /// Find a directory entry (first requires keycloak auth. See `m10 auth`)
    #[command(alias = "d")]
    DirectoryEntries {
        #[command(subcommand)]
        cmd: directory_entry::DirEntry,
    },
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
    #[command(
        alias = "t",
        help_template = "\
    {before-help}{name} {version}
    {about-with-newline}
    {usage-heading}
        \x1b[1mm10 find transfers\x1b[0m [OPTIONS] \
                    \x1b[1m--account\x1b[0m <ACCOUNT> \
                    \x1b[1m| --context-id\x1b[0m <CONTEXT_ID>
                    
    {all-args}{after-help}"
    )]
    Transfers(transfers::FindTransferArgs),
}

impl Find {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Find::Accounts(args) => {
                args.find(context).await?;
            }
            Find::AccountMetadata(args) => {
                args.find(context).await?;
            }
            Find::AccountSets(args) => {
                args.find(context).await?;
            }
            Find::Actions(args) => args.find(context).await?,
            Find::Balances { format } => accounts::list_balances(format, context).await?,
            Find::Banks { format } => {
                context
                    .ledger_client()
                    .list_banks(PageBuilder::default())
                    .await?
                    .print(format)?;
            }
            Find::DirectoryEntries { cmd } => cmd.find(context).await?,
            Find::Roles { name, format } => {
                let roles = context
                    .ledger_client()
                    .list_roles(PageBuilder::<_, NameFilter>::name(name))
                    .await?;

                // TODO check why the query server returns duplicates
                let mut seen = HashSet::new();
                let unique_roles: Vec<Role> = roles
                    .into_iter()
                    .filter_map(|role| {
                        if seen.insert(role.id.clone().to_string()) {
                            TryInto::<Role>::try_into(role).ok()
                        } else {
                            None
                        }
                    })
                    .collect();

                unique_roles.print(format)?;
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
