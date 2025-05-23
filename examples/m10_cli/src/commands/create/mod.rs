use std::{fmt::Debug, fs::File, io::Read};

use clap::{Subcommand, ValueEnum};
use m10_sdk::{
    account::AccountId, error::M10Error, prost::Message, sdk, DocumentBuilder, Pack, PublicKey,
    Signer, WithContext,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    context::{Context, Provider},
    utils,
};

use super::convert::BinFormat;

mod account_sets;
mod accounts;
pub(crate) mod banks;
mod directory_entry;
mod ledger_accounts;
mod role_bindings;
mod roles;
mod transfer;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum)]
pub(crate) enum Algorithm {
    P256,
    Ed25519,
}

impl From<Algorithm> for sdk::signature::Algorithm {
    fn from(value: Algorithm) -> Self {
        match value {
            Algorithm::Ed25519 => sdk::signature::Algorithm::Ed25519,
            Algorithm::P256 => sdk::signature::Algorithm::P256Sha256Asn1,
        }
    }
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Create {
    /// Create a new account on the ledger
    #[command(alias = "la")]
    Account(ledger_accounts::CreateLedgerAccountArgs),
    /// Create a new account metadata record on the ledger
    #[command(alias = "a")]
    AccountMetadata(accounts::CreateAccountMetadataArgs),
    /// Create a new AccountSet record on the ledger
    #[command(alias = "as")]
    AccountSet(account_sets::CreateAccountSetArgs),
    /// Create a new bank metadata record on the ledger
    #[command(alias = "b")]
    Bank(banks::CreateBankArgs),
    /// Create a new collection on the ledger
    #[cfg_attr(feature = "customers", command(alias = "c", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "c"))]
    CollectionMetadata {
        /// Set name of collection
        #[arg(short, long)]
        name: String,
        /// Set descriptor name of protobuf message
        #[arg(short, long)]
        descriptor: String,
    },
    /// Create a directory entry (first requires keycloak auth. See `m10 auth`)
    #[command(alias = "d")]
    DirectoryEntry {
        #[command(subcommand)]
        #[serde(flatten)]
        cmd: directory_entry::CreateDirEntry,
    },
    /// Create a new profile image
    #[command(alias = "i")]
    Image {
        /// Set name of profile image
        #[arg(short, long)]
        name: String,
        /// Set image data base64 encoded
        #[arg(short, long, group = "source")]
        data: Option<String>,
        /// Load image from file
        #[arg(short, long, group = "source")]
        file: Option<String>,
    },
    /// Create a new keypair and print the public key
    #[command(aliases = ["k", "kp"])]
    KeyPair {
        /// File name to store keypair
        #[arg(long, aliases = ["path", "fp"])]
        file_path: Option<String>,
        /// Set format of output
        #[arg(short = 'f', long, default_value = "base64")]
        format: BinFormat,
        /// Set signing algorithm
        #[arg(short, long, value_enum)]
        algorithm: Algorithm,
    },
    /// Create a offline token from an account
    #[cfg_attr(feature = "customers", command(alias = "ot", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "ot"))]
    OfflineToken {
        #[arg(short, long)]
        account: AccountId,
        #[arg(short, long)]
        value: u64,
    },
    /// Create a new RBAC role record on the ledger
    #[command(alias = "r")]
    Role(roles::CreateRoleArgs),
    /// Create a new RBAC role binding record on the ledger
    #[command(alias = "rb")]
    RoleBinding(role_bindings::CreateRoleBindingArgs),
    /// Create a new transfer
    #[command(alias = "t")]
    Transfer(transfer::CreateTransferArgs),
    /// Generate new uuid(s)
    #[command(alias = "u")]
    Uuid {
        /// Set amount of uuids to be generated
        #[arg(short = 'm', long)]
        multiple: Option<usize>,
    },
}

impl Create {
    pub(crate) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Create::Account(args) => args.create(context).await,
            Create::AccountMetadata(args) => {
                store_create::<_, sdk::AccountMetadata>(args, context).await
            }
            Create::AccountSet(args) => store_create::<_, sdk::AccountSet>(args, context).await,
            Create::Bank(args) => store_create::<_, sdk::Bank>(args, context).await,
            Create::CollectionMetadata { name, descriptor } => {
                create_collection(name, descriptor, context).await
            }
            Create::DirectoryEntry { cmd } => cmd.create(context).await,
            Create::Image { name, data, file } => create_image(name, data, file, context).await,
            Create::KeyPair {
                file_path,
                format,
                algorithm,
            } => {
                if context.provider() == Provider::Vault {
                    let pubkey = context.signer().public_key();
                    format.print_bytes(pubkey, vec![BinFormat::Uuid])?;
                    Ok(())
                } else {
                    create_key_pair(file_path, format, algorithm.into())
                }
            }
            Create::OfflineToken { account, value } => {
                create_offline_token(account, value, context).await
            }
            Create::Role(args) => store_create::<_, sdk::Role>(args, context).await,
            Create::RoleBinding(args) => store_create::<_, sdk::RoleBinding>(args, context).await,
            Create::Transfer(args) => args.create(context).await,
            Create::Uuid { multiple } => {
                generate_uuid(multiple);
                Ok(())
            }
        }
    }

    pub(crate) async fn document_operation(
        self,
        context: &Context,
    ) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            Create::CollectionMetadata { name, descriptor } => {
                Ok(sdk::Operation::new_collection(name, descriptor, vec![]))
            }
            Create::AccountMetadata(args) => {
                create_operation::<_, sdk::AccountMetadata>(args, context).await
            }
            Create::AccountSet(args) => create_operation::<_, sdk::AccountSet>(args, context).await,
            Create::Role(args) => create_operation::<_, sdk::Role>(args, context).await,
            Create::RoleBinding(args) => {
                create_operation::<_, sdk::RoleBinding>(args, context).await
            }
            _ => Err(anyhow::anyhow!("Not supported")),
        }
    }
}

trait BuildFromArgs {
    type Document: Pack;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error>;
}

async fn store_create<O, D>(args: O, context: &Context) -> anyhow::Result<()>
where
    O: BuildFromArgs<Document = D> + Debug,
    D: Message + Pack + 'static,
{
    let default_owner = context.signer().public_key().to_vec();

    let document = args.build_from_options(PublicKey(default_owner))?;
    let id = document.id().to_vec();
    let response = m10_sdk::documents(
        context.ledger_client(),
        DocumentBuilder::default()
            .insert(document)
            .context_id(context.context_id()),
    )
    .await;
    match response {
        Ok(_) => {
            eprintln!("Created {} resource:", D::COLLECTION);
            println!("{}", hex::encode(id));
            Ok(())
        }
        Err(M10Error::Transaction(err))
            if err.code() == sdk::transaction_error::Code::AlreadyExists =>
        {
            eprintln!("Item exists already");
            Ok(())
        }
        Err(M10Error::Transaction(err)) => {
            eprintln!("Err {} creating resource: {}", err.code, err.message);
            Err(anyhow::anyhow!("failed resource creation"))
        }
        Err(err) => {
            eprintln!("Err {}", err);
            Err(anyhow::anyhow!("failed resource creation"))
        }
    }
}

async fn create_operation<O, D>(args: O, context: &Context) -> Result<sdk::Operation, anyhow::Error>
where
    O: BuildFromArgs<Document = D>,
    D: Message + Pack + 'static,
{
    let default_owner = context.signer().public_key().to_vec();
    let document = args.build_from_options(PublicKey(default_owner))?;
    Ok(sdk::Operation::insert(document))
}

pub(super) async fn create_collection(
    name: String,
    descriptor: String,
    context: &Context,
) -> anyhow::Result<()> {
    let response = m10_sdk::documents(
        context.ledger_client(),
        DocumentBuilder::default()
            .insert_operation(sdk::Operation::new_collection(
                name.clone(),
                descriptor,
                vec![],
            ))
            .context_id(context.context_id()),
    )
    .await;
    match response {
        Ok(_) => {}
        Err(M10Error::Transaction(err))
            if err.code() == sdk::transaction_error::Code::AlreadyExists =>
        {
            eprintln!("ignoring existing collection: {}", name);
        }
        Err(err) => {
            anyhow::bail!("{}", err);
        }
    }
    Ok(())
}

async fn create_image(
    name: String,
    data: Option<String>,
    file: Option<String>,
    context: &Context,
) -> anyhow::Result<()> {
    let image = if let Some(file_name) = file {
        let mut image_file = File::open(file_name)?;
        let mut image: Vec<u8> = Vec::new();
        image_file.read_to_end(&mut image)?;
        image
    } else if let Some(data) = data {
        base64::decode(data)?
    } else {
        eprintln!("Neither image file or data given");
        return Err(anyhow::anyhow!("Required option missing"));
    };

    context.image_client()?.put_image(&name, image).await?;
    Ok(())
}

fn create_key_pair(
    key_file: Option<String>,
    format: BinFormat,
    algorithm: sdk::signature::Algorithm,
) -> anyhow::Result<()> {
    let buf = if let Some(key_file) = &key_file {
        let k = utils::create_key_pair(key_file, algorithm)?;
        eprintln!("public key is:");
        k
    } else {
        let k = utils::create_exportable_key_pair(algorithm)?;
        eprintln!("key pair is:");
        k
    };
    format.print_bytes(&buf, vec![BinFormat::Uuid])?;
    Ok(())
}

async fn create_offline_token(
    account: AccountId,
    value: u64,
    context: &Context,
) -> anyhow::Result<()> {
    let (tx_id, token) = context
        .ledger_client()
        .create_token(account, value, None, context.context_id())
        .await?;
    println!("txn-id: {tx_id}");
    let token = token.ok_or_else(|| anyhow::anyhow!("no token returned"))?;
    let mut token_buf = vec![];
    token.encode(&mut token_buf)?;
    let path = format!("./ot-{}.tok", hex::encode(tx_id.to_be_bytes().as_slice()));
    std::fs::write(path, token_buf)?;
    println!("{token:?}");
    Ok(())
}

fn generate_uuid(multiple: Option<usize>) {
    let mul = multiple.unwrap_or(1);
    for _ in 0..mul {
        let uuid = Uuid::new_v4();
        println!("{}", uuid);
    }
}
