use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Context as AnyhowContext;
use clap::{ArgGroup, Subcommand};
use m10_sdk::{account::AccountId, ActionBuilder, WithContext};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Invoke {
    /// Invokes a registered action
    #[command(
        alias = "a",
        group = ArgGroup::new("target").required(true),
        help_template = "\
            {before-help}{name} {version}
            {about-with-newline}
            {usage-heading}
                    \x1b[1mm10 invoke action\x1b[0m [OPTIONS] \
                    \x1b[1m--name\x1b[0m <NAME> \
                    \x1b[1m--from\x1b[0m <FROM> \
                    \x1b[1m--to\x1b[0m <TO>

            {all-args}{after-help}"
                )]
    Action {
        /// Name of the registered action
        #[arg(short, long)]
        name: String,
        /// Account ID invoking the action
        #[arg(long)]
        from: AccountId,
        /// Target account ID
        #[arg(short, long, group = "target")]
        to: Option<AccountId>,
        /// Opaque payload. Interpreted as raw string
        #[arg(long, aliases = ["pl", "body"], conflicts_with = "file")]
        payload: Option<String>,
        /// Read payload from file
        #[arg(short, long, conflicts_with = "payload")]
        file: Option<PathBuf>,
    },
}

impl Invoke {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Invoke::Action {
                name,
                from,
                to,
                payload,
                file,
            } => invoke_action(name, from, to, payload, file, context).await,
        }
    }
}

async fn invoke_action(
    name: String,
    from: AccountId,
    to: Option<AccountId>,
    payload: Option<String>,
    file: Option<PathBuf>,
    context: &Context,
) -> anyhow::Result<()> {
    let builder = if let Some(to) = to {
        ActionBuilder::for_account(name.clone(), from, to)
    } else {
        ActionBuilder::for_all(name.clone(), from)
    };

    let mut buf = vec![];
    let payload = if let Some(payload) = payload.as_ref() {
        // Use string as UTF-8
        payload.as_bytes().to_vec()
    } else if let Some(path) = file.as_ref() {
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
    let tx_id = m10_sdk::action(
        context.ledger_client(),
        builder.payload(payload).context_id(context.context_id()),
    )
    .await?;
    eprintln!("Invoked action {} in txId {}", name, tx_id);
    Ok(())
}
