use crate::{collections::PrettyId, context::Context};
use bytes::Bytes;
use clap::Parser;
use m10_sdk::{prost::Message, sdk, Pack};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

#[serde_as]
#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(super) struct DeleteStoreItemOptions {
    /// Set record id
    #[serde_as(as = "DisplayFromStr")]
    id: PrettyId,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum DeleteSubCommands {
    /// Delete an account record
    Account(DeleteStoreItemOptions),
    /// Delete an account set record
    AccountSet(DeleteStoreItemOptions),
    /// Delete a role record
    Role(DeleteStoreItemOptions),
    /// Delete a role binding record
    RoleBinding(DeleteStoreItemOptions),
    /// Delete a bank record
    Bank(DeleteStoreItemOptions),
}

impl DeleteSubCommands {
    pub(super) async fn delete(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            DeleteSubCommands::Account(options) => {
                store_delete::<sdk::Account>(options.id.clone(), config).await
            }
            DeleteSubCommands::AccountSet(options) => {
                store_delete::<sdk::AccountSet>(options.id.clone(), config).await
            }
            DeleteSubCommands::Role(options) => {
                store_delete::<sdk::Role>(options.id.clone(), config).await
            }
            DeleteSubCommands::RoleBinding(options) => {
                store_delete::<sdk::RoleBinding>(options.id.clone(), config).await
            }
            DeleteSubCommands::Bank(options) => {
                store_delete::<sdk::Bank>(options.id.clone(), config).await
            }
        }
    }

    pub(super) fn delete_operation(&self) -> sdk::Operation {
        match self {
            DeleteSubCommands::Account(options) => {
                delete_operation::<sdk::Account>(options.id.clone())
            }
            DeleteSubCommands::AccountSet(options) => {
                delete_operation::<sdk::AccountSet>(options.id.clone())
            }
            DeleteSubCommands::Role(options) => delete_operation::<sdk::Role>(options.id.clone()),
            DeleteSubCommands::RoleBinding(options) => {
                delete_operation::<sdk::RoleBinding>(options.id.clone())
            }
            DeleteSubCommands::Bank(options) => delete_operation::<sdk::Bank>(options.id.clone()),
        }
    }
}

async fn store_delete<D>(id: PrettyId, config: &crate::Config) -> anyhow::Result<()>
where
    D: Message + Pack + Default + 'static,
{
    let mut context = Context::new(config).await?;
    let id_bytes: Bytes = id.clone().into();
    let response = context
        .submit_transaction(
            sdk::Operation::delete::<D>(id_bytes.to_vec()),
            config.context_id.clone(),
        )
        .await?;
    if let Err(err) = response {
        eprintln!("Err {} deleting resource: {}", err.code, err.message);
        Err(anyhow::anyhow!("failed resource deletion"))
    } else {
        eprintln!("deleted {} in {}", id, D::COLLECTION);
        Ok(())
    }
}

fn delete_operation<D>(id: PrettyId) -> sdk::Operation
where
    D: Message + Pack + Default + 'static,
{
    let id: Bytes = id.into();
    sdk::Operation::delete::<D>(id.to_vec())
}
