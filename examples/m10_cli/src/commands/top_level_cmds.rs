use clap::Subcommand;
use m10_sdk::{sdk, Format};
use serde::{Deserialize, Serialize};

use super::*;
use crate::context::Context;

#[derive(Clone, Subcommand, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Commands {
    /// Authenticate with keycloak. For create and find operations on directory-entry
    #[command(alias = "a")]
    Auth(auth::Auth),
    /// Convert ids
    #[command(alias = "cv")]
    Convert {
        #[command(subcommand)]
        cmd: convert::Convert,
    },
    /// Create various items, on and off the ledger
    #[command(alias = "c")]
    Create {
        #[command(subcommand)]
        #[serde(skip)] // Will be handled in the custom deserialize
        cmd: create::Create,
    },
    /// Delete an item
    #[command(alias = "d")]
    Delete {
        #[command(subcommand)]
        #[serde(flatten)]
        cmd: delete::Delete,
    },
    /// Endorse items
    #[command(alias = "e")]
    Endorse {
        #[command(subcommand)]
        cmd: endorse::Endorse,
    },
    /// Find items by field(s)
    #[command(aliases = &["f", "l", "list", "find-by", "by"])]
    Find {
        #[command(subcommand)]
        #[serde(flatten)]
        cmd: find::Find,
    },
    /// Authenticate with FIS
    #[command(alias = "fa")]
    FisAuth(fis_auth::FisAuth),
    /// Get an item by id
    #[command(alias = "g")]
    Get {
        #[command(subcommand)]
        #[serde(flatten)]
        cmd: get::Get,
    },
    /// Invoke action
    #[command(alias = "i")]
    Invoke {
        #[command(subcommand)]
        cmd: invoke::Invoke,
    },
    /// Issue a token
    #[cfg_attr(feature = "customers", command(alias = "it", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "it"))]
    Issue {
        #[command(subcommand)]
        cmd: token::Issue,
    },
    /// Observe changes
    #[command(alias = "o")]
    Observe {
        #[command(subcommand)]
        cmd: observe::Observe,
    },
    /// redeem a token
    #[cfg_attr(feature = "customers", command(alias = "rt", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "rt"))]
    Redeem {
        #[command(subcommand)]
        cmd: token::Redeem,
    },
    /// Run a migration or batch query file
    #[command(alias = "r")]
    Run {
        /// Set batch file location
        file: String,
        /// If set only parses the files without sending any request
        #[arg(long, alias = "dr", global = true)]
        dry_run: bool,
        #[command(subcommand)]
        cmd: batch::Run,
    },
    /// Display items
    #[command(aliases = &["s", "display"])]
    Show {
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short, long, default_value_t)]
        format: Format,
        #[command(subcommand)]
        cmd: show::Show,
    },
    /// Update item field(s)
    #[command(alias = "u")]
    Update {
        #[command(subcommand)]
        #[serde(flatten)]
        cmd: update::Update,
    },
    /// Verify a token
    #[cfg_attr(feature = "customers", command(alias = "vt", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "vt"))]
    Verify {
        #[command(subcommand)]
        cmd: token::Verify,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum CommandsHelper {
    Auth(auth::Auth),
    FisAuth(fis_auth::FisAuth),
    Convert {
        cmd: convert::Convert,
    },
    Create {
        create: create::Create,
    },
    Delete {
        cmd: delete::Delete,
    },
    Endorse {
        cmd: endorse::Endorse,
    },
    Find {
        cmd: find::Find,
    },
    Get {
        cmd: get::Get,
    },
    Invoke {
        cmd: invoke::Invoke,
    },
    Issue {
        cmd: token::Issue,
    },
    Observe {
        cmd: observe::Observe,
    },
    Redeem {
        cmd: token::Redeem,
    },
    Run {
        file: String,
        dry_run: bool,
        cmd: batch::Run,
    },
    Show {
        format: Format,
        cmd: show::Show,
    },
    Update {
        cmd: update::Update,
    },
    Verify {
        cmd: token::Verify,
    },
}

impl<'de> Deserialize<'de> for Commands {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = CommandsHelper::deserialize(deserializer)?;
        Ok(match helper {
            CommandsHelper::Auth(cmd) => Commands::Auth(cmd),
            CommandsHelper::FisAuth(cmd) => Commands::FisAuth(cmd),
            CommandsHelper::Convert { cmd } => Commands::Convert { cmd },
            CommandsHelper::Create { create } => Commands::Create { cmd: create },
            CommandsHelper::Delete { cmd } => Commands::Delete { cmd },
            CommandsHelper::Endorse { cmd } => Commands::Endorse { cmd },
            CommandsHelper::Find { cmd } => Commands::Find { cmd },
            CommandsHelper::Get { cmd } => Commands::Get { cmd },
            CommandsHelper::Invoke { cmd } => Commands::Invoke { cmd },
            CommandsHelper::Issue { cmd } => Commands::Issue { cmd },
            CommandsHelper::Observe { cmd } => Commands::Observe { cmd },
            CommandsHelper::Redeem { cmd } => Commands::Redeem { cmd },
            CommandsHelper::Run { file, dry_run, cmd } => Commands::Run { file, dry_run, cmd },
            CommandsHelper::Show { format, cmd } => Commands::Show { format, cmd },
            CommandsHelper::Update { cmd } => Commands::Update { cmd },
            CommandsHelper::Verify { cmd } => Commands::Verify { cmd },
        })
    }
}

impl Commands {
    pub(crate) async fn run(self, context: &Context) -> anyhow::Result<()> {
        let result = match self {
            Commands::Auth(cmd) => cmd.run(context).await,
            Commands::FisAuth(cmd) => cmd.run().await,
            Commands::Convert { cmd } => cmd.convert(),
            Commands::Create { cmd } => cmd.run(context).await,
            Commands::Delete { cmd } => cmd.run(context).await,
            Commands::Endorse { cmd } => cmd.run(context).await,
            Commands::Find { cmd } => cmd.run(context).await,
            Commands::Get { cmd } => cmd.run(context).await,
            Commands::Invoke { cmd } => cmd.run(context).await,
            Commands::Issue { cmd } => cmd.run(context).await,
            Commands::Observe { cmd } => cmd.run(context).await,
            Commands::Redeem { cmd } => cmd.run(context).await,
            Commands::Run { cmd, file, dry_run } => cmd.run(file, dry_run, context).await,
            Commands::Show { format, cmd } => cmd.run(format).await,
            Commands::Update { cmd } => cmd.run(context).await,
            Commands::Verify { cmd } => cmd.run(),
        };

        match &result {
            Err(err) => log::debug!("Command failed: {:?}", err),
            _ => {}
        }

        result
    }

    pub(super) async fn document_operation(
        self,
        context: &Context,
    ) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            Commands::Create { cmd } => cmd.document_operation(context).await,
            Commands::Update { cmd } => cmd.document_operation().await,
            Commands::Delete { cmd } => Ok(cmd.operation()),
            _ => Err(anyhow::anyhow!("non CUD command in batch {:?}", self)),
        }
    }

    pub(super) async fn handle_batch(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Commands::Get { cmd } => cmd.run(context).await?,
            Commands::Find { cmd } => cmd.run(context).await?,
            _ => {
                return Err(anyhow::anyhow!(
                    "non Get/Find/Query command in batch {:?}",
                    self
                ));
            }
        }
        Ok(())
    }

    pub(super) fn dry_run(&self, migration: bool) -> anyhow::Result<()> {
        if migration {
            match self {
                Commands::Create { cmd } => println!("creating: {:#?}", cmd),
                Commands::Update { cmd } => println!("updating: {:#?}", cmd),
                Commands::Delete { cmd } => println!("deleting: {:#?}", cmd),
                _ => {
                    return Err(anyhow::anyhow!("non CUD command in batch {:?}", self));
                }
            }
        } else {
            match self {
                Commands::Get { cmd } => println!("getting: {:#?}", cmd),
                Commands::Find { cmd } => println!("finding: {:#?}", cmd),
                _ => {
                    return Err(anyhow::anyhow!(
                        "non Get/Find/Query command in batch {:?}",
                        self
                    ));
                }
            }
        }
        Ok(())
    }
}
