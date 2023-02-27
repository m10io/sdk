use crate::context::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetImageOptions {
    /// Image name
    name: String,
}

impl GetImageOptions {
    pub(super) async fn get(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config)?;
        let image = context.image_client.get_image(&self.name).await?;
        if image.is_empty() {
            eprintln!("Image {} not found", self.name);
        } else {
            println!("{}", base64::encode(&image));
        }
        Ok(())
    }
}
