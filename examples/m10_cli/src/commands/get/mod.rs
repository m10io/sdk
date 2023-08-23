use crate::context::Context;
use clap::Parser;
use m10_sdk::account::AccountId;
use m10_sdk::{Format, PrettyPrint};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use uuid::Uuid;

mod actions;
mod directory_entry;
mod images;
mod key_pair;
mod public_key;
mod transfers;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(super) struct GetStoreItemOptions<ID>
where
    ID: FromStr + Display,
    ID::Err: StdError + Send + Sync + 'static,
{
    /// The record id
    id: ID,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short = 'f', long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum GetSubCommands {
    /// Get an account record by id
    Account(GetStoreItemOptions<AccountId>),
    /// Get an account info record by id
    AccountInfo(GetStoreItemOptions<AccountId>),
    /// Get an account set record by id
    AccountSet(GetStoreItemOptions<Uuid>),
    /// Get an action by tx id
    Action(actions::GetActionOptions),
    /// Get a bank record by id
    Bank(GetStoreItemOptions<Uuid>),
    /// Get ledger account information by id
    LedgerAccount(GetStoreItemOptions<AccountId>),
    /// Get a role record by id
    Role(GetStoreItemOptions<Uuid>),
    /// Get a role binding record by id
    RoleBinding(GetStoreItemOptions<Uuid>),
    /// Get a transfer by id
    Transfer(transfers::GetTransferOptions),
    /// Get a directory entry
    DirectoryEntry(directory_entry::GetDirEntryOptions),
    /// Get a profile image by id
    Image(images::GetImageOptions),
    /// Display a public key from a key pair
    /// stored in a file or in M10_SIGNING_KEY env variable
    PublicKey(public_key::GetPubKeyOptions),
    /// Display a key pair
    /// stored in a file or in M10_SIGNING_KEY env variable
    KeyPair(key_pair::GetKeyPairOptions),
    /// Get current block height
    BlockHeight,
    /// Get offline public key
    OfflineKey,
}

impl GetSubCommands {
    pub(super) async fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        if let GetSubCommands::PublicKey(options) = self {
            return options.get(config);
        };
        let context = Context::new(config)?;
        match self {
            GetSubCommands::Account(options) => context
                .m10_client
                .get_account(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::AccountInfo(options) => context
                .m10_client
                .get_account_metadata(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::AccountSet(options) => context
                .m10_client
                .get_account_set(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::Bank(options) => context
                .m10_client
                .get_bank(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::LedgerAccount(options) => context
                .m10_client
                .get_account(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::Role(options) => context
                .m10_client
                .get_role(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::RoleBinding(options) => context
                .m10_client
                .get_role_binding(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::Transfer(options) => {
                if options.enhanced {
                    context
                        .m10_client
                        .get_enhanced_transfer(options.id)
                        .await?
                        .print(options.format)?
                } else {
                    context
                        .m10_client
                        .get_transfer(options.id)
                        .await?
                        .print(options.format)?
                }
            }
            GetSubCommands::DirectoryEntry(options) => options.get().await?,
            GetSubCommands::Image(options) => options.get(config).await?,
            GetSubCommands::Action(options) => context
                .m10_client
                .get_action(options.id)
                .await?
                .print(options.format)?,
            GetSubCommands::KeyPair(options) => options.get(config)?,
            GetSubCommands::PublicKey(_) => unreachable!("No context"),
            GetSubCommands::BlockHeight => {
                let height = context.m10_client.get_block_height().await?;
                println!("{height}");
            }
            GetSubCommands::OfflineKey => {
                let key = context.m10_client.get_offline_key().await?;
                println!("{}", base64::encode(key));
            }
        };
        Ok(())
    }
}
