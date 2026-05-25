use clap::Subcommand;
use m10_sdk::{Format, PageBuilder, PrettyPrint, RoleBindingFilter, RoleFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::parse_key_value;
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
        #[arg(
            short,
            long,
            conflicts_with_all = ["instance", "description", "labels"],
            required_unless_present_any = ["instance", "description", "labels"]
        )]
        name: Option<String>,
        /// Set description filter
        #[arg(
            short,
            long,
            conflicts_with_all = ["instance", "name", "labels"],
            required_unless_present_any = ["instance", "name", "labels"]
        )]
        description: Option<String>,
        /// Set instance id filter (base64)
        #[arg(
            long,
            conflicts_with_all = ["name", "description", "labels"],
            required_unless_present_any = ["name", "description", "labels"]
        )]
        instance: Option<String>,
        /// Set labels filter (e.g. `-l label_1=value_1 -l label_2=value_2`)
        #[arg(
            short = 'l',
            long,
            conflicts_with_all = ["name", "description", "instance"],
            required_unless_present_any = ["name", "description", "instance"],
            value_parser = parse_key_value,
        )]
        labels: Option<Vec<(String, String)>>,
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short, long, default_value = "raw")]
        #[serde(default)]
        format: Format,
    },
    /// Find role binding record(s)
    #[command(alias = "rb")]
    RoleBindings {
        /// Set name filter
        #[arg(
            short,
            long,
            conflicts_with_all = ["subject", "description", "labels"],
            required_unless_present_any = ["subject", "description", "labels"]
        )]
        name: Option<String>,
        /// Set description filter
        #[arg(
            short,
            long,
            conflicts_with_all = ["subject", "name", "labels"],
            required_unless_present_any = ["subject", "name", "labels"]
        )]
        description: Option<String>,
        /// Set subject filter (public key, base64)
        #[arg(
            long,
            conflicts_with_all = ["name", "description", "labels"],
            required_unless_present_any = ["name", "description", "labels"]
        )]
        subject: Option<String>,
        /// Set labels filter (e.g. `-l label_1=value_1 -l label_2=value_2`)
        #[arg(
            short = 'l',
            long,
            conflicts_with_all = ["name", "description", "subject"],
            required_unless_present_any = ["name", "description", "subject"],
            value_parser = parse_key_value,
        )]
        labels: Option<Vec<(String, String)>>,
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
            Find::Roles {
                name,
                description,
                instance,
                labels,
                format,
            } => {
                let builder = if let Some(name) = name {
                    PageBuilder::<_, RoleFilter>::name(name)
                } else if let Some(description) = description {
                    PageBuilder::<_, RoleFilter>::description(description)
                } else if let Some(instance) = instance {
                    let instance_id = parse_instance_id(&instance)?;
                    PageBuilder::<_, RoleFilter>::instance_id(instance_id)
                } else if let Some(labels) = labels {
                    let labels: std::collections::HashMap<String, String> =
                        labels.into_iter().collect();
                    PageBuilder::<_, RoleFilter>::labels(labels)
                } else {
                    anyhow::bail!(
                        "either --name, --description, --labels or --instance is required"
                    );
                };
                let roles = context.ledger_client().list_roles(builder).await?;
                let display_roles: Vec<Role> = roles
                    .into_iter()
                    .filter_map(|role| TryInto::<Role>::try_into(role).ok())
                    .collect();
                display_roles.print(format)?;
            }
            Find::RoleBindings {
                name,
                description,
                subject,
                labels,
                format,
            } => {
                let builder = if let Some(name) = name {
                    PageBuilder::<_, RoleBindingFilter>::name(name)
                } else if let Some(description) = description {
                    PageBuilder::<_, RoleBindingFilter>::description(description)
                } else if let Some(subject) = subject {
                    let subject = base64::decode(subject)?;
                    PageBuilder::<_, RoleBindingFilter>::subject(subject)
                } else if let Some(labels) = labels {
                    let labels: std::collections::HashMap<String, String> =
                        labels.into_iter().collect();
                    PageBuilder::<_, RoleBindingFilter>::labels(labels)
                } else {
                    anyhow::bail!(
                        "either --name, --description, --labels or --subject is required"
                    );
                };
                let role_bindings = context.ledger_client().list_role_bindings(builder).await?;

                let display_role_bindings: Vec<crate::collections::role_bindings::RoleBinding> =
                    role_bindings
                        .into_iter()
                        .filter_map(|rb| TryInto::try_into(rb).ok())
                        .collect();

                display_role_bindings.print(format)?;
            }
            Find::Transactions(args) => args.find(context).await?,
            Find::Transfers(args) => {
                args.find(context).await?;
            }
        }
        Ok(())
    }
}

fn parse_instance_id(value: &str) -> anyhow::Result<Vec<u8>> {
    let trimmed = value.trim();
    if let Ok(uuid) = Uuid::parse_str(trimmed) {
        return Ok(uuid.as_bytes().to_vec());
    }

    let hex_candidate = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    if is_hex(hex_candidate) && hex_candidate.len().is_multiple_of(2) {
        return Ok(hex::decode(hex_candidate)?);
    }

    base64::decode(trimmed).map_err(|err| {
        anyhow::anyhow!(
            "invalid instance id '{}': expected uuid, hex, or base64 ({})",
            value,
            err
        )
    })
}

fn is_hex(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|b| b.is_ascii_hexdigit())
}
