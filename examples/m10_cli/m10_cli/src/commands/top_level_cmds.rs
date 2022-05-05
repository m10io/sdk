use super::{
    call::CallSubCommands, convert::ConvertSubCommands, create::CreateSubCommands,
    delete::DeleteSubCommands, endorse::EndorsementSubCommands, find::FindSubCommands,
    get::GetSubCommands, show::ShowSubCommands, update::UpdateSubCommands,
};
use crate::commands::observe::ObserveSubcommands;
use clap::{Parser, Subcommand};
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) enum Commands {
    /// Create various items, on and off the ledger
    #[clap(alias = "c")]
    Create(CreateOptions),
    /// Get an item by id
    #[clap(alias = "g")]
    Get(GetOptions),
    /// Find items by field(s)
    #[clap(aliases = &["l", "list", "find-by", "by"])]
    Find(FindOptions),
    /// Update item field(s)
    #[clap(alias = "u")]
    Update(UpdateOptions),
    /// Delete an item
    #[clap(alias = "d")]
    Delete(DeleteOptions),
    /// Call to ledger-api
    #[clap(alias = "r")]
    Call(CallOptions),
    /// Run a migration or batch query file
    #[clap(alias = "f")]
    File(super::batch::BatchOptions),
    /// Convert data
    #[clap(subcommand)]
    Convert(ConvertSubCommands),
    /// Display items
    #[clap(aliases = &["s", "display"])]
    Show(ShowOptions),
    /// Endorse items
    #[clap(alias = "e")]
    Endorse(EndorsementOptions),
    /// Observe changes
    #[clap(alias = "o")]
    Observe(ObserveOptions),
    /// Authenticate
    Auth(AuthOptions),
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct CallOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: CallSubCommands,
}

impl CallOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.call(config).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct CreateOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: CreateSubCommands,
}

impl CreateOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.create(config).await
    }

    async fn create_operation(
        &self,
        config: &crate::Config,
    ) -> Result<sdk::Operation, anyhow::Error> {
        self.cmd.create_operation(config).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct GetOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: GetSubCommands,
}

impl GetOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.get(config).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct FindOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: FindSubCommands,
}

impl FindOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.find(config).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: UpdateSubCommands,
}

impl UpdateOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.update(config).await
    }

    async fn update_operation(&self) -> Result<sdk::Operation, anyhow::Error> {
        self.cmd.update_operation().await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct DeleteOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: DeleteSubCommands,
}

impl DeleteOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.delete(config).await
    }

    fn delete_operation(&self) -> sdk::Operation {
        self.cmd.delete_operation()
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct ShowOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: ShowSubCommands,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short = 'f', long, default_value = "raw")]
    #[serde(default = "Format::default")]
    format: Format,
}

impl ShowOptions {
    async fn run(&self, _config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.show(self.format).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct EndorsementOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: EndorsementSubCommands,
}

impl EndorsementOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.endorse(config).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct ObserveOptions {
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: ObserveSubcommands,
}

impl ObserveOptions {
    async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        self.cmd.observe(config).await
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
/// Obtains OAuth id and access token and writes them to
/// id.token
/// access.token
pub(crate) struct AuthOptions {
    #[clap(long)]
    pub(crate) client_secret: Option<String>,
    #[clap(long)]
    pub(crate) username: Option<String>,
    #[clap(long)]
    pub(crate) password: Option<String>,
    /// Writes access token to stdout
    #[clap(long)]
    pub(crate) stdout: bool,
}

impl Commands {
    pub(crate) async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            Commands::Create(op) => op.run(config).await,
            Commands::Get(op) => op.run(config).await,
            Commands::Find(op) => op.run(config).await,
            Commands::Update(op) => op.run(config).await,
            Commands::Delete(op) => op.run(config).await,
            Commands::Call(op) => op.run(config).await,
            Commands::Convert(op) => op.convert(),
            Commands::File(op) => op.run(config).await,
            Commands::Show(op) => op.run(config).await,
            Commands::Endorse(op) => op.run(config).await,
            Commands::Observe(op) => op.run(config).await,
            Commands::Auth(op) => op.run(config).await,
        }
    }

    pub(super) async fn document_operation(
        &self,
        config: &crate::Config,
    ) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            Commands::Create(op) => op.create_operation(config).await,
            Commands::Update(op) => op.update_operation().await,
            Commands::Delete(op) => Ok(op.delete_operation()),
            _ => Err(anyhow::anyhow!("non CUD command in batch {:?}", self)),
        }
    }

    pub(super) async fn handle_batch(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            Commands::Get(op) => op.run(config).await?,
            Commands::Find(op) => op.run(config).await?,
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
                Commands::Create(op) => println!("creating: {:#?}", op),
                Commands::Update(op) => println!("updating: {:#?}", op),
                Commands::Delete(op) => println!("deleting: {:#?}", op),
                _ => {
                    return Err(anyhow::anyhow!("non CUD command in batch {:?}", self));
                }
            }
        } else {
            match self {
                Commands::Get(op) => println!("getting: {:#?}", op),
                Commands::Find(op) => println!("finding: {:#?}", op),
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub(crate) enum Format {
    Json,
    Yaml,
    Raw,
}

impl Format {
    pub(super) fn default() -> Self {
        Format::Raw
    }
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            "raw" => Ok(Format::Raw),
            _ => Err("no match"),
        }
    }
}
