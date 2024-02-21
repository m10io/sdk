use bytes::Bytes;
use clap::{Args, Subcommand};
use m10_sdk::{prost::Message, sdk, DocumentBuilder, Pack, WithContext};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{collections::PrettyId, context::Context};

#[serde_as]
#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct DeleteStoreItemArgs {
    /// Set record id
    #[serde_as(as = "DisplayFromStr")]
    id: PrettyId,
}

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Delete {
    /// Delete an account record
    #[command(alias = "a")]
    Account(DeleteStoreItemArgs),
    /// Delete an account set record
    #[command(alias = "as")]
    AccountSet(DeleteStoreItemArgs),
    /// Delete a role record
    #[command(alias = "r")]
    Role(DeleteStoreItemArgs),
    /// Delete a role binding record
    #[command(alias = "rb")]
    RoleBinding(DeleteStoreItemArgs),
    /// Delete a bank record
    #[command(alias = "b")]
    Bank(DeleteStoreItemArgs),
}

impl Delete {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Delete::Account(args) => store_delete::<sdk::AccountMetadata>(args.id, context).await,
            Delete::AccountSet(args) => store_delete::<sdk::AccountSet>(args.id, context).await,
            Delete::Role(args) => store_delete::<sdk::Role>(args.id, context).await,
            Delete::RoleBinding(args) => store_delete::<sdk::RoleBinding>(args.id, context).await,
            Delete::Bank(args) => store_delete::<sdk::Bank>(args.id, context).await,
        }
    }

    pub(super) fn operation(&self) -> sdk::Operation {
        match self {
            Delete::Account(args) => delete_operation::<sdk::AccountMetadata>(args.id.clone()),
            Delete::AccountSet(args) => delete_operation::<sdk::AccountSet>(args.id.clone()),
            Delete::Role(args) => delete_operation::<sdk::Role>(args.id.clone()),
            Delete::RoleBinding(args) => delete_operation::<sdk::RoleBinding>(args.id.clone()),
            Delete::Bank(args) => delete_operation::<sdk::Bank>(args.id.clone()),
        }
    }
}

async fn store_delete<D>(id: PrettyId, context: &Context) -> anyhow::Result<()>
where
    D: Message + Pack + Default + 'static,
{
    m10_sdk::documents(
        context.ledger_client(),
        DocumentBuilder::default()
            .delete_custom(D::COLLECTION, id.to_vec())
            .context_id(context.context_id()),
    )
    .await?;
    eprintln!("deleted {} in {}", id, D::COLLECTION);
    Ok(())
}

fn delete_operation<D>(id: PrettyId) -> sdk::Operation
where
    D: Message + Pack + Default + 'static,
{
    let id: Bytes = id.into();
    sdk::Operation::delete::<D>(id.to_vec())
}
