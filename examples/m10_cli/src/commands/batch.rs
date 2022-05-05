use crate::context::Context;
use clap::{ArgGroup, Parser};
use m10_protos::sdk::transaction_error::Code;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("migration_flags"), about)]
pub(crate) struct BatchOptions {
    /// If set file can only contain CUD operations else only query operations
    #[clap(short, long, group = "migration_flags")]
    migration: bool,
    /// Set batch file location
    file: String,
    /// If set only parses the files without sending any request
    #[clap(long)]
    dry_run: bool,
}

impl BatchOptions {
    pub(super) async fn run(&self, config: &crate::Config) -> anyhow::Result<()> {
        let yaml_file = File::open(&self.file)?;
        let reader = BufReader::new(yaml_file);
        let data: Vec<super::Commands> = serde_yaml::from_reader(reader)?;
        if self.dry_run {
            for op in data {
                op.dry_run(self.migration)?;
            }
        } else if self.migration {
            let mut operations = Vec::default();
            for op in data {
                operations.push(op.document_operation(config).await?);
            }
            let mut context = Context::new(config).await?;
            let response = context
                .submit_transaction(operations, config.context_id.clone())
                .await?;
            if let Err(err) = response {
                if err.code() != Code::AlreadyExists {
                    anyhow::bail!(err);
                }
            }
        } else {
            for op in data {
                op.handle_batch(config).await?;
            }
        }
        Ok(())
    }
}
