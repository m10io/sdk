use crate::context::Context;
use clap::Parser;
use m10_sdk::DocumentUpdate;
use m10_sdk::{prost::Message, sdk, Pack};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

mod account_sets;
mod accounts;
mod ledger_accounts;
mod role_bindings;
mod roles;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(super) enum UpdateSubCommands {
    /// Update account record
    Account(accounts::UpdateAccountOptions),
    /// Update account set record
    AccountSet(account_sets::UpdateAccountSetOptions),
    /// Update ledger account
    LedgerAccount(ledger_accounts::UpdateLedgerAccountOptions),
    /// Update role record
    Role(roles::UpdateRoleOptions),
    /// Update role binding record
    RoleBinding(role_bindings::UpdateRoleBindingOptions),
}

impl UpdateSubCommands {
    pub(super) async fn update(&self, config: &crate::Config) -> anyhow::Result<()> {
        match self {
            UpdateSubCommands::Account(options) => store_update(options.id, options, config).await,
            UpdateSubCommands::AccountSet(options) => {
                store_update(options.id, options, config).await
            }
            UpdateSubCommands::LedgerAccount(options) => options.update(config).await,
            UpdateSubCommands::Role(options) => store_update(options.id, options, config).await,
            UpdateSubCommands::RoleBinding(options) => {
                store_update(options.id, options, config).await
            }
        }
    }

    pub(super) async fn update_operation(&self) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            UpdateSubCommands::Account(options) => update_operation(options.id, options),
            UpdateSubCommands::AccountSet(options) => update_operation(options.id, options),
            UpdateSubCommands::Role(options) => update_operation(options.id, options),
            UpdateSubCommands::RoleBinding(options) => update_operation(options.id, options),
            _ => Err(anyhow::anyhow!("Not supported")),
        }
    }
}

trait BuildFromOptions {
    type Document;

    fn build_from_options(
        &self,
        builder: &mut DocumentUpdate<Self::Document>,
    ) -> anyhow::Result<()>;
}

async fn store_update<M, O>(id: Uuid, options: &O, config: &crate::Config) -> anyhow::Result<()>
where
    O: BuildFromOptions<Document = M>,
    M: Message + Pack + Default,
{
    let mut context = Context::new(config).await?;
    let mut builder = DocumentUpdate::<M>::new(id);

    options.build_from_options(&mut builder)?;
    context
        .submit_transaction(builder.operation(), config.context_id.clone())
        .await??;

    Ok(())
}

fn update_operation<M, O>(id: Uuid, options: &O) -> Result<sdk::Operation, anyhow::Error>
where
    O: BuildFromOptions<Document = M>,
    M: Message + Pack + Default,
{
    let mut builder = DocumentUpdate::<M>::new(id);

    options.build_from_options(&mut builder)?;
    Ok(builder.operation())
}
