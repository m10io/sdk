use std::{any::Any, str::FromStr};

use clap::{Args, Subcommand};
use m10_sdk::{
    account::AccountId, directory::GetObjectUrlRequest, Format, PrettyPrint, ResourceId, Signer,
    TransferData, TransferView,
};
use serde::{Deserialize, Serialize};

use crate::{collections::roles::Role, context::Context};

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
    /// Get ledger account information by id
    #[command(alias = "la")]
    Account(GetStoreItemArgs<AccountId>),
    /// Get an account metadata record by id
    #[command(alias = "a")]
    AccountMetadata(GetStoreItemArgs<ResourceId>),
    /// Get an account set record by id
    #[command(alias = "as")]
    AccountSet(GetStoreItemArgs<ResourceId>),
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
    Bank(GetStoreItemArgs<ResourceId>),
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
    /// Get offline public key
    #[cfg_attr(feature = "customers", command(alias = "ok", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "ok"))]
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
    Role(GetStoreItemArgs<ResourceId>),
    /// Get a role binding record by id
    #[command(alias = "rb")]
    RoleBinding(GetStoreItemArgs<ResourceId>),
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
            Get::AccountMetadata(args) => context
                .ledger_client()
                .get_account_metadata(args.id.to_vec())
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
            Get::OfflineKey => {
                let key = context.ledger_client().get_offline_key().await?;
                println!("{}", base64::encode(key));
            }
            Get::PublicKey { format } => {
                let signer = context.signer();
                format.print_bytes(signer.public_key(), vec![BinFormat::Uuid])?;
            }
            Get::Role(args) => {
                let role = m10_sdk::get_role(context.ledger_client(), args.id).await?;
                TryInto::<Role>::try_into(role)?.print(args.format)?
            }
            Get::RoleBinding(args) => m10_sdk::get_role_binding(context.ledger_client(), args.id)
                .await?
                .print(args.format)?,
            Get::Transfer {
                id,
                enhanced,
                format,
            } => {
                let audit_log_result = context
                    .block_explorer_client()?
                    .get_audit_log(&id.to_string())
                    .await;

                let tx_sender = match audit_log_result {
                    Ok(log) => Some(log.public_key),
                    Err(e) => {
                        eprintln!("Failed to get audit log: {e}");
                        None
                    }
                };

                let view = if enhanced {
                    let transfer = context.ledger_client().get_enhanced_transfer(id).await?;
                    TransferView {
                        transfer: TransferData::Expanded(transfer.try_into()?),
                        tx_sender,
                    }
                } else {
                    let transfer = context.ledger_client().get_transfer(id).await?;
                    TransferView {
                        transfer: TransferData::Basic(transfer.try_into()?),
                        tx_sender,
                    }
                };

                view.print(format)?
            }
        };
        Ok(())
    }
}
