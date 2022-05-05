use crate::context::Context;
use anyhow::Context as AnyhowContext;
use clap::Parser;
use m10_protos::sdk::transaction_error::Code;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct ActionOptions {
    /// Name of the registered action
    #[clap(short, long)]
    name: String,
    /// Account ID invoking the action
    #[clap(long)]
    from: String,
    /// Target account ID
    #[clap(short, long)]
    target: String,
    /// Opaque payload. Interpreted as raw string
    #[clap(short, long, conflicts_with = "file")]
    payload: Option<String>,
    /// Read payload from file
    #[clap(short, long, conflicts_with = "payload")]
    file: Option<PathBuf>,
}

impl ActionOptions {
    pub(crate) async fn invoke(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;

        let from = hex::decode(&self.from).context("Invalid <from> format")?;
        let target = hex::decode(&self.target).context("Invalid <target> format")?;

        let mut buf = vec![];
        let payload = if let Some(payload) = self.payload.as_ref() {
            // Use string as UTF-8
            payload.as_bytes().to_vec()
        } else if let Some(path) = self.file.as_ref() {
            let mut file = File::open(path).context("Could not read payload file")?;
            file.read_to_end(&mut buf)?;
            buf
        } else {
            eprintln!("Reading payload from STDIN. Press ENTER to continue..");
            let mut value = String::new();
            std::io::stdin()
                .read_line(&mut value)
                .context("Could not read from STDIN")?;
            value.as_bytes().to_vec()
        };

        let action = sdk::InvokeAction {
            name: self.name.clone(),
            payload,
            from_account: from,
            target: Some(sdk::Target {
                target: Some(sdk::target::Target::AccountId(target)),
            }),
        };
        let response = context
            .submit_transaction(action, config.context_id.clone())
            .await?;
        if let Err(err) = response {
            let err_type = Code::from_i32(err.code).unwrap_or(Code::Unknown);
            eprintln!("Could not invoke action: {:?} {}", err_type, err.message);
            Err(anyhow::anyhow!("failed action"))
        } else {
            eprintln!("Invoked action {}:", self.name);
            Ok(())
        }
    }
}
