use crate::context::Context;
use clap::Subcommand;
use m10_sdk::{prost::Message, sdk, Collection, Pack, Signer};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

mod account_sets;
mod accounts;
mod collection_metadata;
mod directory_entry;
mod images;
mod key_pair;
mod ledger_accounts;
mod role_bindings;
mod roles;
mod transfer;
mod uuid_options;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum CreateSubCommands {
    /// Create a new collection on the ledger
    CollectionMetadata(collection_metadata::CreateCollectionMetadataOptions),
    /// Create a new account set record on the ledger
    AccountSet(account_sets::CreateAccountSetOptions),
    /// Create a new account metadata record on the ledger
    Account(accounts::CreateAccountOptions),
    /// Create a directory entry
    DirectoryEntry(directory_entry::CreateDirEntryOptions),
    /// Create a new account on the ledger
    LedgerAccount(ledger_accounts::CreateLedgerAccountOptions),
    /// Create a new RBAC role record on the ledger
    Role(roles::CreateRoleOptions),
    /// Create a new RBAC role binding record on the ledger
    RoleBinding(role_bindings::CreateRoleBindingOptions),
    /// Create a new transfer
    Transfer(transfer::Transfer),
    /// Create a new profile image
    Image(images::Image),
    /// Generate new uuid(s)
    Uuid(uuid_options::UuidOptions),
    /// Create a new keypair and print the public key
    KeyPair(key_pair::CreateKeyPairOptions),
}

impl CreateSubCommands {
    pub(crate) async fn create(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            CreateSubCommands::CollectionMetadata(options) => options.create(config).await,
            CreateSubCommands::Account(options) => {
                store_create::<_, sdk::Account>(options, config).await
            }
            CreateSubCommands::AccountSet(options) => {
                store_create::<_, sdk::AccountSet>(options, config).await
            }
            CreateSubCommands::DirectoryEntry(options) => options.create().await,
            CreateSubCommands::LedgerAccount(options) => options.create(config).await,
            CreateSubCommands::Role(options) => store_create::<_, sdk::Role>(options, config).await,
            CreateSubCommands::RoleBinding(options) => {
                store_create::<_, sdk::RoleBinding>(options, config).await
            }
            CreateSubCommands::Transfer(options) => options.create(config).await,
            CreateSubCommands::Image(options) => options.create(config).await,
            CreateSubCommands::Uuid(options) => {
                options.create();
                Ok(())
            }
            CreateSubCommands::KeyPair(options) => options.create(),
        }
    }

    pub(crate) async fn create_operation(
        &self,
        config: &crate::Config,
    ) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            CreateSubCommands::CollectionMetadata(options) => Ok(options.create_operation()),
            CreateSubCommands::Account(options) => {
                create_operation::<_, sdk::Account>(options, config).await
            }
            CreateSubCommands::AccountSet(options) => {
                create_operation::<_, sdk::AccountSet>(options, config).await
            }
            CreateSubCommands::Role(options) => {
                create_operation::<_, sdk::Role>(options, config).await
            }
            CreateSubCommands::RoleBinding(options) => {
                create_operation::<_, sdk::RoleBinding>(options, config).await
            }
            _ => Err(anyhow::anyhow!("Not supported")),
        }
    }
}

trait BuildFromOptions {
    type Document: Pack;
    fn build_from_options(&self, default_owner: Vec<u8>) -> Result<Self::Document, anyhow::Error>;
}

async fn store_create<O, D>(options: &O, config: &crate::Config) -> anyhow::Result<()>
where
    O: BuildFromOptions<Document = D> + Debug,
    D: Message + Pack + 'static,
{
    let mut context = Context::new(config).await?;
    let default_owner = context.admin.public_key().to_vec();
    let document = options.build_from_options(default_owner)?;
    let id = document.id().to_vec();
    let response = context
        .submit_transaction(sdk::Operation::insert(document), config.context_id.clone())
        .await?;
    if let Err(err) = response {
        if err.code == sdk::transaction_error::Code::AlreadyExists as i32 {
            eprintln!("Item exists already");
            Ok(())
        } else {
            eprintln!("Err {} creating resource: {}", err.code, err.message);
            Err(anyhow::anyhow!("failed resource creation"))
        }
    } else {
        eprintln!("Created {} resource:", D::COLLECTION);
        if D::COLLECTION == Collection::Accounts {
            println!("{}", hex::encode(&id));
        } else {
            println!("{}", Uuid::from_slice(&id).unwrap());
        }
        Ok(())
    }
}

async fn create_operation<O, D>(
    options: &O,
    config: &crate::Config,
) -> Result<sdk::Operation, anyhow::Error>
where
    O: BuildFromOptions<Document = D>,
    D: Message + Pack + 'static,
{
    let context = Context::new(config).await?;
    let default_owner = context.admin.public_key().to_vec();
    let document = options.build_from_options(default_owner)?;
    Ok(sdk::Operation::insert(document))
}
