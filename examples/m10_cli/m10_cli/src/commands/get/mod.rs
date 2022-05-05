use crate::commands::top_level_cmds::Format;
use crate::{collections::*, context::Context};
use clap::Parser;
use m10_sdk::prost::Message;
use m10_sdk::{sdk, Pack, Signer};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    fmt::Debug,
    io::{self, LineWriter},
};
use uuid::Uuid;

mod actions;
mod directory_entry;
mod images;
mod key_pair;
mod ledger_accounts;
mod public_key;
mod transfers;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(super) struct GetStoreItemOptions {
    /// The record id
    id: Uuid,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short = 'f', long, default_value = "raw")]
    #[serde(default = "Format::default")]
    format: Format,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum GetSubCommands {
    /// Get an account record by id
    Account(GetStoreItemOptions),
    /// Get an account set record by id
    AccountSet(GetStoreItemOptions),
    /// Get an action by tx id
    Action(actions::GetActionOptions),
    /// Get ledger account information by id
    LedgerAccount(ledger_accounts::GetLedgerAccountOptions),
    /// Get a role record by id
    Role(GetStoreItemOptions),
    /// Get a role binding record by id
    RoleBinding(GetStoreItemOptions),
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
}

impl GetSubCommands {
    pub(super) async fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            GetSubCommands::Account(options) => {
                let mut context = Context::new(config).await?;
                let request = context
                    .admin
                    .sign_request(sdk::GetAccountRequest {
                        id: options.id.as_bytes().to_vec(),
                    })
                    .await?;
                let doc = context.m10_client.get_account(request).await?;
                print_document::<_, accounts::Account>(doc, options.format)
            }
            GetSubCommands::AccountSet(options) => {
                let mut context = Context::new(config).await?;
                let request = context
                    .admin
                    .sign_request(sdk::GetAccountSetRequest {
                        id: options.id.as_bytes().to_vec(),
                    })
                    .await?;
                let doc = context.m10_client.get_account_set(request).await?;
                print_document::<_, account_sets::AccountSet>(doc, options.format)
            }
            GetSubCommands::LedgerAccount(options) => options.get(config).await,
            GetSubCommands::Role(options) => {
                let mut context = Context::new(config).await?;
                let request = context
                    .admin
                    .sign_request(sdk::GetRoleRequest {
                        id: options.id.as_bytes().to_vec(),
                    })
                    .await?;
                let doc = context.m10_client.get_role(request).await?;
                print_document::<_, roles::Role>(doc, options.format)
            }
            GetSubCommands::RoleBinding(options) => {
                let mut context = Context::new(config).await?;
                let request = context
                    .admin
                    .sign_request(sdk::GetRoleBindingRequest {
                        id: options.id.as_bytes().to_vec(),
                    })
                    .await?;
                let doc = context.m10_client.get_role_binding(request).await?;
                print_document::<_, role_bindings::RoleBinding>(doc, options.format)
            }
            GetSubCommands::Transfer(options) => options.get(config).await,
            GetSubCommands::DirectoryEntry(options) => options.get().await,
            GetSubCommands::Image(options) => options.get(config).await,
            GetSubCommands::PublicKey(options) => options.get(config),
            GetSubCommands::KeyPair(options) => options.get(config),
            GetSubCommands::Action(options) => options.get(config).await,
        }
    }
}

fn print_item<I>(item: I, format: Format) -> anyhow::Result<()>
where
    I: Serialize,
{
    match format {
        Format::Json => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let writer = LineWriter::new(handle);
            serde_json::to_writer_pretty(writer, &item)?;
        }
        Format::Yaml => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let writer = LineWriter::new(handle);
            serde_yaml::to_writer(writer, &item)?;
        }
        Format::Raw => {
            let pretty = PrettyConfig::new()
                .with_depth_limit(4)
                .with_separate_tuple_members(true);
            let s = to_string_pretty(&item, pretty)?;
            println!("{}", s);
        }
    }
    Ok(())
}

fn print_document<D, I>(document: D, format: Format) -> anyhow::Result<()>
where
    D: Message + Pack + Default,
    I: TryFrom<D, Error = anyhow::Error> + Serialize,
{
    let printable = I::try_from(document)?;
    print_item(printable, format)?;
    Ok(())
}
