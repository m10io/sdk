use std::{fs::File, io::BufReader};

use clap::Subcommand;
use m10_sdk::{
    error::M10Error,
    sdk::{self, TransactionError},
    DocumentBuilder, WithContext,
};
use serde::{Deserialize, Serialize};
use tonic::Code;

use crate::context::Context;

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Run {
    /// Execute a list of commands sequentially (supports read-only Get/Find commands only).
    #[command(alias = "b")]
    Batch,
    /// Apply a set of Create/Update/Delete document operations as one atomic migration.
    #[command(alias = "m")]
    Migration,
}

impl Run {
    pub(super) async fn run(
        self,
        file: String,
        dry_run: bool,
        context: &Context,
    ) -> anyhow::Result<()> {
        match self {
            Run::Batch => Self::run_batch(file, dry_run, context).await,
            Run::Migration => Self::run_migration(file, dry_run, context).await,
        }
    }

    fn load_commands(file: &str) -> anyhow::Result<Vec<super::Commands>> {
        let yaml_file =
            File::open(file).map_err(|_| anyhow::anyhow!("file not found: {}", file))?;
        let reader = BufReader::new(yaml_file);
        serde_yml::from_reader(reader)
            .map_err(|e| anyhow::anyhow!("invalid command file {}: {}", file, e))
    }

    async fn run_migration(file: String, dry_run: bool, context: &Context) -> anyhow::Result<()> {
        let data = Self::load_commands(&file)?;
        if dry_run {
            for op in data {
                op.dry_run(true)?;
            }
        } else {
            let mut operations = Vec::with_capacity(data.len());
            for op in data {
                operations.push(op.document_operation(context).await?);
            }
            Self::submit_migration_operations(context, operations).await?;
        }
        Ok(())
    }

    async fn submit_migration_operations(
        context: &Context,
        operations: Vec<sdk::Operation>,
    ) -> anyhow::Result<()> {
        let mut pending = vec![operations];

        while let Some(batch) = pending.pop() {
            if batch.is_empty() {
                continue;
            }

            let mut builder = DocumentBuilder::default();
            for operation in batch.iter().cloned() {
                builder = builder.insert_operation(operation);
            }

            let result = m10_sdk::documents(
                context.ledger_client(),
                builder.context_id(context.context_id()),
            )
            .await;

            match result {
                Ok(_) => {}
                Err(M10Error::Transaction(TransactionError { code, .. }))
                    if code == sdk::transaction::transaction_error::Code::AlreadyExists as i32 => {}
                Err(M10Error::Status(status)) if status.code() == Code::ResourceExhausted => {
                    if batch.len() == 1 {
                        anyhow::bail!(
                            "migration request is too large even for a single operation: {}",
                            status.message()
                        );
                    }

                    let midpoint = batch.len() / 2;
                    let left = batch[..midpoint].to_vec();
                    let right = batch[midpoint..].to_vec();

                    // Preserve original execution order
                    pending.push(right);
                    pending.push(left);
                }
                Err(err) => anyhow::bail!(err),
            }
        }

        Ok(())
    }

    async fn run_batch(file: String, dry_run: bool, context: &Context) -> anyhow::Result<()> {
        let data = Self::load_commands(&file)?;
        if dry_run {
            for op in data {
                op.dry_run(false)?;
            }
        } else {
            for op in data {
                op.handle_batch(context).await?;
            }
        }
        Ok(())
    }
}
