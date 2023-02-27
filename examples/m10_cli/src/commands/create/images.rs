use crate::context::Context;
use clap::{ArgGroup, Parser};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs::File, io::Read};

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("source").required(true))]
#[clap(about)]
pub(crate) struct Image {
    /// Set name of profile image
    #[clap(short, long)]
    name: String,
    /// Set image data base64 encoded
    #[clap(short, long, group = "source")]
    data: Option<String>,
    /// Load image from file
    #[clap(short, long, group = "source")]
    file: Option<String>,
}

impl Image {
    pub(super) async fn create(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config)?;

        let image = if let Some(file_name) = &self.file {
            let mut image_file = File::open(file_name)?;
            let mut image: Vec<u8> = Vec::new();
            image_file.read_to_end(&mut image)?;
            image
        } else if let Some(data) = &self.data {
            base64::decode(data)?
        } else {
            eprintln!("Neither image file or data given");
            return Err(anyhow::anyhow!("Required option missing"));
        };

        context.image_client.put_image(&self.name, image).await?;
        Ok(())
    }
}
