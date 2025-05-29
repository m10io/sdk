use std::{fs::File, io::BufReader};

use clap::Subcommand;
use m10_sdk::{
    error::M10Error,
    sdk::{self, TransactionError},
    DocumentBuilder, WithContext,
};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Run {
    #[command(alias = "b")]
    Batch,
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
        let yaml_file = File::open(file)?;
        let reader = BufReader::new(yaml_file);
        Ok(serde_yml::from_reader(reader)?)
    }

    async fn run_migration(file: String, dry_run: bool, context: &Context) -> anyhow::Result<()> {
        let data = Self::load_commands(&file)?;
        if dry_run {
            for op in data {
                op.dry_run(true)?;
            }
        } else {
            let mut operations = DocumentBuilder::default();

            for op in data {
                operations = operations.insert_operation(op.document_operation(context).await?);
            }
            let result = m10_sdk::documents(
                context.ledger_client(),
                operations.context_id(context.context_id()),
            )
            .await;
            match result {
                Ok(_) => {}
                Err(M10Error::Transaction(TransactionError { code, .. }))
                    if code == sdk::transaction::transaction_error::Code::AlreadyExists as i32 => {}
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
