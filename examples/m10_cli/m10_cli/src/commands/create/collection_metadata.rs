use crate::context::Context;
use clap::Parser;
use m10_sdk::sdk::{transaction_error::Code, Operation};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateCollectionMetadataOptions {
    /// Set name of collection
    #[clap(short, long)]
    name: String,
    /// Set descriptor name of protobuf message
    #[clap(short, long)]
    descriptor: String,
}

impl CreateCollectionMetadataOptions {
    pub(super) async fn create(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let response = context
            .submit_transaction(self.create_operation(), config.context_id.clone())
            .await?;
        if let Err(err) = response {
            if err.code() == Code::AlreadyExists {
                eprintln!("ignoring existing collection: {}", self.name);
            } else {
                anyhow::bail!("{} code={:?}", err.message, err.code);
            }
        }
        Ok(())
    }

    pub(crate) fn create_operation(&self) -> Operation {
        Operation::new_collection(self.name.clone(), self.descriptor.clone(), Vec::default())
    }
}
