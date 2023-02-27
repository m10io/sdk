use crate::context::Context;
use clap::Parser;
use m10_sdk::{
    error::M10Error,
    sdk::{transaction_error::Code, Operation},
    DocumentBuilder, WithContext,
};
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
        let context = Context::new(config)?;
        let response = context
            .m10_client
            .documents(
                DocumentBuilder::default()
                    .insert_operation(self.create_operation())
                    .context_id(config.context_id.clone()),
            )
            .await;
        match response {
            Ok(_) => {}
            Err(M10Error::Transaction(err)) if err.code() == Code::AlreadyExists => {
                eprintln!("ignoring existing collection: {}", self.name);
            }
            Err(err) => {
                anyhow::bail!("{}", err);
            }
        }
        Ok(())
    }

    pub(crate) fn create_operation(&self) -> Operation {
        Operation::new_collection(self.name.clone(), self.descriptor.clone(), Vec::default())
    }
}
