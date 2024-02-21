use clap::Subcommand;
use m10_sdk::{prost::Message, sdk, DocumentBuilder, DocumentUpdate, Pack, WithContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::Context;

mod account_sets;
mod accounts;
mod banks;
mod ledger_accounts;
mod role_bindings;
mod roles;
mod transfer;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Update {
    /// Update account record
    #[command(alias = "a")]
    Account(accounts::UpdateAccountArgs),
    /// Update account set record
    #[command(alias = "as")]
    AccountSet(account_sets::UpdateAccountSetArgs),
    /// Update bank record
    #[command(alias = "b")]
    Bank(banks::UpdateBankArgs),
    /// Update ledger account
    #[command(alias = "la")]
    LedgerAccount(ledger_accounts::UpdateLedgerAccountArgs),
    /// Update role record
    #[command(alias = "r")]
    Role(roles::UpdateRoleArgs),
    /// Update role binding record
    #[command(alias = "rb")]
    RoleBinding(role_bindings::UpdateRoleBindingArgs),
    /// Update transfer status
    #[command(alias = "t")]
    Transfer(transfer::UpdateTransferArgs),
}

impl Update {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Update::Account(args) => store_update(args.id, args, context).await,
            Update::AccountSet(args) => store_update(args.id, args, context).await,
            Update::Bank(args) => store_update(args.id, args, context).await,
            Update::LedgerAccount(args) => args.update(context).await,
            Update::Role(args) => store_update(args.id, args, context).await,
            Update::RoleBinding(args) => store_update(args.id, args, context).await,
            Update::Transfer(args) => args.do_update(context).await,
        }
    }

    pub(super) async fn document_operation(self) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            Update::Account(args) => update_operation(args.id, args),
            Update::AccountSet(args) => update_operation(args.id, args),
            Update::Bank(args) => update_operation(args.id, args),
            Update::Role(args) => update_operation(args.id, args),
            Update::RoleBinding(args) => update_operation(args.id, args),
            _ => Err(anyhow::anyhow!("Not supported")),
        }
    }
}

trait BuildFromArgs {
    type Document;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<()>;
}

async fn store_update<M, O>(id: Uuid, args: O, context: &Context) -> anyhow::Result<()>
where
    O: BuildFromArgs<Document = M>,
    M: Message + Pack + Default,
{
    let mut builder = DocumentUpdate::<M>::new(id);

    args.build_from_args(&mut builder)?;

    m10_sdk::documents(
        context.ledger_client(),
        DocumentBuilder::default()
            .update(&builder)
            .context_id(context.context_id()),
    )
    .await?;
    Ok(())
}

fn update_operation<M, O>(id: Uuid, args: O) -> Result<sdk::Operation, anyhow::Error>
where
    O: BuildFromArgs<Document = M>,
    M: Message + Pack + Default,
{
    let mut builder = DocumentUpdate::<M>::new(id);

    args.build_from_args(&mut builder)?;
    Ok(builder.operation())
}
