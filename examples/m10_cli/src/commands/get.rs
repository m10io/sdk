use std::{any::Any, str::FromStr};

use clap::{Args, Subcommand};
use m10_sdk::{
    account::AccountId, directory::GetObjectUrlRequest, Format, MetadataExt, PrettyPrint,
    ResourceId, Signer, TransferData, TransferView,
};
use serde::{Deserialize, Serialize};

use crate::{collections::role_bindings::RoleBinding, collections::roles::Role, context::Context};

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
                let image = context
                    .image_client()?
                    .get_image(&name)
                    .await
                    .map_err(|_| anyhow::anyhow!("image not found"))?;
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
                let signer = context.signer()?;
                format.print_bytes(signer.public_key(), vec![BinFormat::Uuid])?;
            }
            Get::Role(args) => {
                let role = m10_sdk::get_role(context.ledger_client(), args.id).await?;
                TryInto::<Role>::try_into(role)?.print(args.format)?
            }
            Get::RoleBinding(args) => {
                let role_binding =
                    m10_sdk::get_role_binding(context.ledger_client(), args.id).await?;
                TryInto::<RoleBinding>::try_into(role_binding)?.print(args.format)?
            }
            Get::Transfer {
                id,
                enhanced,
                format,
            } => {
                let txn_result = context
                    .block_explorer_client()?
                    .get_transaction_v2(&id.to_string())
                    .await;

                let tx_signer = match txn_result {
                    Ok(txn) => txn
                        .data
                        .first()
                        .map(|d| d.attributes.signer_public_key.clone()),
                    Err(e) => {
                        eprintln!("Failed to get transaction from block explorer: {e}");
                        None
                    }
                };

                let view = if enhanced {
                    let transfer = context.ledger_client().get_enhanced_transfer(id).await?;
                    TransferView {
                        transfer: TransferData::Expanded(transfer),
                        tx_signer,
                    }
                } else {
                    let transfer = context.ledger_client().get_transfer(id).await?;
                    TransferView {
                        transfer: TransferData::Basic(transfer),
                        tx_signer,
                    }
                };

                if format == Format::Raw {
                    // Convert to a generic JSON value to manipulate it before printing
                    // Use a custom struct to enforce field order for raw output.
                    #[derive(serde::Serialize)]
                    struct CustomTransferStep<'a> {
                        from: &'a AccountId,
                        to: &'a AccountId,
                        amount: u64,
                        metadata: Vec<String>,
                    }

                    #[derive(serde::Serialize)]
                    struct CustomTransferView<'a> {
                        tx_id: u64,
                        context_id: String,
                        timestamp: String,
                        steps: Vec<CustomTransferStep<'a>>,
                        success: bool,
                        status: m10_sdk::TransferStatus,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        tx_signer: &'a Option<String>,
                    }

                    let (transfer, steps) = match &view.transfer {
                        TransferData::Basic(t) => (t, &t.steps),
                        TransferData::Expanded(_) => {
                            // Expanded view is complex; fall back to default serialization for now.
                            view.print(format)?;
                            return Ok(());
                        }
                    };

                    let timestamp: chrono::DateTime<chrono::Utc> = transfer.timestamp.into();

                    let custom_view = CustomTransferView {
                        tx_id: transfer.tx_id,
                        context_id: hex::encode(&transfer.context_id),
                        timestamp: timestamp.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
                        steps: steps
                            .iter()
                            .map(|s| CustomTransferStep {
                                from: &s.from,
                                to: &s.to,
                                amount: s.amount,
                                metadata: s.metadata.iter().map(format_metadata).collect(),
                            })
                            .collect(),
                        success: transfer.success,
                        status: transfer.status,
                        tx_signer: &view.tx_signer,
                    };

                    let json_string = serde_json::to_string_pretty(&custom_view)?;
                    println!("{}", json_string);
                } else {
                    // Use the standard print for JSON/YAML formats
                    view.print(format)?;
                }
            }
        };
        Ok(())
    }
}

fn format_metadata(data: &m10_sdk::prost::Any) -> String {
    if let Some(m10_sdk::sdk::metadata::Memo { plaintext }) = data.protobuf() {
        format!("Memo: {plaintext}")
    } else if let Some(m10_sdk::sdk::metadata::Fee {}) = data.protobuf() {
        "Fee".to_string()
    } else if let Some(m10_sdk::sdk::metadata::Withdraw { bank_account_id }) = data.protobuf() {
        format!("Withdraw {bank_account_id}")
    } else if let Some(m10_sdk::sdk::metadata::Deposit { bank_account_id }) = data.protobuf() {
        format!("Deposit {bank_account_id}")
    } else if let Some(m10_sdk::sdk::metadata::Attachment { object_id, .. }) = data.protobuf() {
        format!("Attachment: {object_id}")
    } else if let Some(m10_sdk::sdk::metadata::Contract { endorsements, .. }) = data.protobuf() {
        format!("Contract with {} endorsments", endorsements.len())
    } else if let Some(m10_sdk::sdk::metadata::SelfTransfer {
        from_account_name,
        to_account_name,
    }) = data.protobuf()
    {
        format!("Self transfer from {from_account_name} to {to_account_name}")
    } else if let Some(m10_sdk::sdk::metadata::RebalanceTransfer {}) = data.protobuf() {
        "Rebalance transfer".to_string()
    } else {
        "Unknown".to_string()
    }
}
