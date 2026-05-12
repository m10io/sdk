use crate::context::Context;
use clap::Subcommand;
use m10_sdk::{prost::Message, sdk, DocumentBuilder, DocumentUpdate, Pack, WithContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    /// Update account on the ledger
    #[command(alias = "la")]
    Account(ledger_accounts::UpdateLedgerAccountArgs),
    /// Update account metadata record
    #[command(alias = "a")]
    AccountMetadata(accounts::UpdateAccountArgs),
    /// Update AccountSet record
    #[command(alias = "as")]
    AccountSet(account_sets::UpdateAccountSetArgs),
    /// Update bank record
    #[command(alias = "b")]
    Bank(banks::UpdateBankArgs),
    /// Update role record
    #[command(alias = "r")]
    Role(roles::UpdateRoleArgs),
    /// Update role metadata (name, description, owner)
    #[command(alias = "rm")]
    RoleMetadata(roles::UpdateRoleMetadataArgs),
    /// Update role rules
    #[command(alias = "rr")]
    RoleRules(roles::UpdateRoleRulesArgs),
    /// Update role binding record
    #[command(alias = "rb")]
    RoleBinding(role_bindings::UpdateRoleBindingArgs),
    /// Update role binding subjects
    #[command(subcommand)]
    RoleBindingSubjects(role_bindings::RoleBindingSubjects),
    /// Update transfer status
    #[command(alias = "t")]
    Transfer(transfer::UpdateTransferArgs),
}

impl Update {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Update::Account(args) => args.update(context).await,
            Update::AccountMetadata(args) => store_update(args.id, args, context).await,
            Update::AccountSet(args) => store_update(args.id, args, context).await,
            Update::Bank(args) => store_update(args.id, args, context).await,
            Update::Role(args) => store_update(args.id, args, context).await,
            Update::RoleMetadata(args) => store_update(args.id, args, context).await,
            Update::RoleRules(args) => args.update(context).await,
            Update::RoleBinding(args) => store_update(args.id, args, context).await,
            Update::RoleBindingSubjects(args) => args.update(context).await,
            Update::Transfer(args) => args.do_update(context).await,
        }
    }

    pub(super) async fn document_operation(self) -> Result<sdk::Operation, anyhow::Error> {
        match self {
            Update::AccountMetadata(args) => update_operation(args.id, args),
            Update::AccountSet(args) => update_operation(args.id, args),
            Update::Bank(args) => update_operation(args.id, args),
            Update::Role(args) => update_operation(args.id, args),
            Update::RoleMetadata(args) => update_operation(args.id, args),
            Update::RoleBinding(args) => update_operation(args.id, args),
            _ => Err(anyhow::anyhow!("Not supported")),
        }
    }
}

trait BuildFromArgs {
    type Document;

    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<bool>;
}

async fn store_update<M, O>(id: Uuid, args: O, context: &Context) -> anyhow::Result<()>
where
    O: BuildFromArgs<Document = M>,
    M: Message + Pack + Default,
{
    let mut builder = DocumentUpdate::<M>::new(id);

    let changed = args.build_from_args(&mut builder)?;

    m10_sdk::documents(
        context.ledger_client(),
        DocumentBuilder::default()
            .update(&builder)
            .context_id(context.context_id()),
    )
    .await?;

    if changed {
        println!("document was successfully updated");
    } else {
        println!("warning: no fields specified, nothing was updated");
    }

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
