use clap::{Args, Subcommand};
use futures_lite::StreamExt;
use m10_sdk::{account::AccountId, AccountFilter, Format, PrettyPrint, TxId};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct ObserveArgs {
    /// Account IDs
    #[arg(short)]
    ids: Vec<AccountId>,
    /// Transaction ID to start observing from
    #[arg(long, aliases = ["start", "sf", "from"])]
    starting_from: Option<u64>,
    /// output format
    #[arg(short, long, default_value_t)]
    format: Format,
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Observe {
    /// Observe account updates for the provided account IDs
    #[command(alias = "a")]
    Accounts(ObserveArgs),
    /// Observe actions for the provided account IDs
    #[command(alias = "ac")]
    Actions {
        /// Name of the action
        #[arg(short, long)]
        name: String,
        /// Account IDs (a '-i <ID>' for each account ID to observe)
        #[arg(short, long)]
        ids: Vec<AccountId>,
        /// Transaction ID to start observing from
        #[arg(long)]
        starting_from: Option<u64>,
        /// Set output format (one of 'json', 'yaml', 'raw')
        #[arg(short = 'f', long, default_value_t)]
        format: Format,
    },
    /// Observe transaction metrics involving the provided issuance account IDs
    #[command(
        alias = "m",
        after_long_help = "Metrics include transfer volume, number of transfers, transfer errors and accounts \
    created since the last report. Refreshed approximately every 25 seconds."
    )]
    Metrics(ObserveArgs),
    /// Observe transfers involving the provided account IDs
    #[command(alias = "t")]
    Transfers(ObserveArgs),
}

impl Observe {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Observe::Accounts(args) => Self::observe_accounts(args, context).await,
            Observe::Actions {
                name,
                ids,
                starting_from,
                format,
            } => Self::observe_actions(&name, &ids, starting_from, format, context).await,
            Observe::Metrics(args) => Self::observe_metrics(args, context).await,
            Observe::Transfers(args) => Self::observe_transfers(args, context).await,
        }
    }

    async fn observe_accounts(args: ObserveArgs, context: &Context) -> anyhow::Result<()> {
        let ObserveArgs {
            ids,
            starting_from,
            format,
        } = args;
        let mut filter = ids
            .iter()
            .copied()
            .fold(AccountFilter::default(), |filter, id| filter.involves(id));
        if let Some(start) = starting_from {
            filter = filter.starting_from(start);
        }
        let mut updates = context.ledger_client().observe_accounts(filter).await?;

        while let Some(message) = updates.next().await {
            match message {
                Ok(account_updates) => {
                    account_updates.print(format)?;
                }
                Err(err) => {
                    eprintln!("Error while receiving message: {}", err);
                    break;
                }
            }
        }
        eprintln!("Stream terminated");
        Ok(())
    }

    async fn observe_actions(
        name: &str,
        ids: &[AccountId],
        starting_from: Option<TxId>,
        format: Format,
        context: &Context,
    ) -> anyhow::Result<()> {
        let mut filter = ids
            .iter()
            .copied()
            .fold(AccountFilter::name(name), |filter, id| filter.involves(id));
        if let Some(start) = starting_from {
            filter = filter.starting_from(start);
        }
        let mut stream = context.ledger_client().observe_actions(filter).await?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(actions) => {
                    actions.print(format)?;
                }
                Err(err) => {
                    eprintln!("Error while receiving message: {}", err);
                    break;
                }
            }
        }
        eprintln!("Stream terminated");
        Ok(())
    }

    async fn observe_metrics(args: ObserveArgs, context: &Context) -> anyhow::Result<()> {
        let ObserveArgs {
            ids,
            starting_from,
            format,
        } = args;
        let mut filter = ids
            .iter()
            .copied()
            .fold(AccountFilter::default(), |filter, id| filter.involves(id));
        if let Some(start) = starting_from {
            filter = filter.starting_from(start);
        }
        let mut updates = context.ledger_client().observe_metrics(filter).await?;

        while let Some(message) = updates.next().await {
            match message {
                Ok(sample) => {
                    sample.print(format)?;
                }
                Err(err) => {
                    eprintln!("Error while receiving message: {}", err);
                    break;
                }
            }
        }
        eprintln!("Stream terminated");
        Ok(())
    }

    async fn observe_transfers(args: ObserveArgs, context: &Context) -> anyhow::Result<()> {
        let ObserveArgs {
            ids,
            starting_from,
            format,
        } = args;
        let mut filter = ids
            .iter()
            .copied()
            .fold(AccountFilter::default(), |filter, id| filter.involves(id));
        if let Some(start) = starting_from {
            filter = filter.starting_from(start);
        }
        let mut stream = context.ledger_client().observe_transfers(filter).await?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(transfers) => {
                    transfers.print(format)?;
                }
                Err(err) => {
                    eprintln!("Error while receiving message: {}", err);
                    break;
                }
            }
        }
        eprintln!("Stream terminated");
        Ok(())
    }
}
