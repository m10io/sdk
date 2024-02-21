use std::{any::Any, str::FromStr};

use clap::{Args, Subcommand};
use m10_sdk::{account::AccountId, directory::GetObjectUrlRequest, Format, PrettyPrint, Signer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::Context;

use super::convert::BinFormat;

#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct GetStoreItemArgs<ID>
where
    ID: FromStr + Any + Clone + Send + Sync + 'static,
    <ID as FromStr>::Err: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    /// The record id
    id: ID,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, alias = "fmt", default_value = "raw")]
    #[serde(default)]
    format: Format,
}

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Get {
    /// Get an account record by id
    #[command(alias = "a")]
    Account(GetStoreItemArgs<AccountId>),
    /// Get an account info record by id
    #[command(alias = "ai")]
    AccountInfo(GetStoreItemArgs<AccountId>),
    /// Get an account set record by id
    #[command(alias = "as")]
    AccountSet(GetStoreItemArgs<Uuid>),
    /// Get an action by tx id
    #[command(alias = "ac")]
    Action {
        /// The transaction id
        id: u64,
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short, long, default_value = "raw")]
        #[serde(default)]
        format: Format,
    },
    /// Get a bank record by id
    #[command(alias = "b")]
    Bank(GetStoreItemArgs<Uuid>),
    /// Get current block height
    #[command(alias = "bh")]
    BlockHeight,
    /// Get a directory entry
    #[command(alias = "d")]
    DirectoryEntry {
        #[arg(short, long)]
        object_id: String,
    },
    /// Get a profile image by id
    #[command(alias = "i")]
    Image {
        /// Image name
        name: String,
    },
    /// Display a key pair
    /// stored in a file or in M10_SIGNING_KEY env variable
    #[command(alias = "sk")]
    KeyPair {
        #[arg(short, long, default_value = "base64")]
        format: crate::commands::convert::BinFormat,
    },
    /// Get ledger account information by id
    #[command(alias = "la")]
    LedgerAccount(GetStoreItemArgs<AccountId>),
    /// Get offline public key
    #[command(alias = "ok")]
    OfflineKey,
    /// Display a public key from a key pair
    /// stored in a file or in M10_SIGNING_KEY env variable
    #[command(alias = "pk")]
    PublicKey {
        #[arg(short, long, default_value = "base64")]
        format: crate::commands::convert::BinFormat,
    },
    /// Get a role record by id
    #[command(alias = "r")]
    Role(GetStoreItemArgs<Uuid>),
    /// Get a role binding record by id
    #[command(alias = "rb")]
    RoleBinding(GetStoreItemArgs<Uuid>),
    /// Get a transfer by id
    #[command(alias = "t")]
    Transfer {
        /// The transaction id
        id: u64,
        /// Set enhanced result
        #[arg(short, long)]
        enhanced: bool,
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short, long, default_value = "raw")]
        #[serde(default)]
        format: Format,
    },
}

impl Get {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Get::Account(args) => context
                .ledger_client()
                .get_account(args.id)
                .await?
                .print(args.format)?,
            Get::AccountInfo(args) => context
                .ledger_client()
                .get_account_metadata(args.id)
                .await?
                .print(args.format)?,
            Get::AccountSet(args) => m10_sdk::get_account_set(context.ledger_client(), args.id)
                .await?
                .print(args.format)?,
            Get::Action { id, format } => context
                .ledger_client()
                .get_action(id)
                .await?
                .print(format)?,
            Get::Bank(args) => m10_sdk::get_bank(context.ledger_client(), args.id)
                .await?
                .print(args.format)?,
            Get::BlockHeight => {
                let height = context.ledger_client().get_block_height().await?;
                println!("{height}");
            }
            Get::DirectoryEntry { object_id } => {
                let mut client = context.directory_client()?;
                let request = GetObjectUrlRequest {
                    object_id: object_id.clone(),
                };
                let response = client.get_object_url(request).await?.into_inner();
                println!("{:#?}", response);
            }
            Get::Image { name } => {
                let image = context.image_client()?.get_image(&name).await?;
                if image.is_empty() {
                    eprintln!("Image {} not found", name);
                } else {
                    println!("{}", base64::encode(&image));
                }
            }
            Get::KeyPair { format } => {
                let key = context.raw_key()?;
                format.print_bytes(&key, vec![BinFormat::Uuid])?;
            }
            Get::LedgerAccount(args) => context
                .ledger_client()
                .get_account(args.id)
                .await?
                .print(args.format)?,
            Get::OfflineKey => {
                let key = context.ledger_client().get_offline_key().await?;
                println!("{}", base64::encode(key));
            }
            Get::PublicKey { format } => {
                let key = context.signing_key()?;
                format.print_bytes(key.public_key(), vec![BinFormat::Uuid])?;
            }
            Get::Role(args) => m10_sdk::get_role(context.ledger_client(), args.id)
                .await?
                .print(args.format)?,
            Get::RoleBinding(args) => m10_sdk::get_role_binding(context.ledger_client(), args.id)
                .await?
                .print(args.format)?,
            Get::Transfer {
                id,
                enhanced,
                format,
            } => {
                if enhanced {
                    context
                        .ledger_client()
                        .get_enhanced_transfer(id)
                        .await?
                        .print(format)?
                } else {
                    context
                        .ledger_client()
                        .get_transfer(id)
                        .await?
                        .print(format)?
                }
            }
        };
        Ok(())
    }
}
