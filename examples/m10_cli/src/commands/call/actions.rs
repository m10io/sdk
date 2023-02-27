use crate::context::Context;
use anyhow::Context as AnyhowContext;
use clap::ArgGroup;
use clap::Parser;
use m10_sdk::account::AccountId;
use m10_sdk::ActionBuilder;
use m10_sdk::WithContext;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("target").required(true), about)]
pub(crate) struct ActionOptions {
    /// Name of the registered action
    #[clap(short, long)]
    name: String,
    /// Account ID invoking the action
    #[clap(long)]
    from: AccountId,
    /// Target account ID
    #[clap(short, long, group = "target")]
    to: Option<AccountId>,
    #[clap(short, long, group = "target")]
    broadcast: bool,
    /// Opaque payload. Interpreted as raw string
    #[clap(short, long, conflicts_with = "file")]
    payload: Option<String>,
    /// Read payload from file
    #[clap(short, long, conflicts_with = "payload")]
    file: Option<PathBuf>,
}

impl ActionOptions {
    pub(crate) async fn invoke(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;

        let builder = if let Some(to) = self.to {
            ActionBuilder::for_account(self.name.clone(), self.from, to)
        } else {
            ActionBuilder::for_all(self.name.clone(), self.from)
        };

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
        let tx_id = context
            .m10_client
            .action(
                builder
                    .payload(payload)
                    .context_id(config.context_id.clone()),
            )
            .await?;
        eprintln!("Invoked action {} in txId {}", self.name, tx_id);
        Ok(())
    }
}
