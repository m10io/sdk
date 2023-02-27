use crate::context::Context;
use futures_lite::StreamExt;
use m10_sdk::account::AccountId;
use m10_sdk::{AccountFilter, Format, PrettyPrint, TxId};

pub(crate) async fn observe(
    name: &str,
    ids: &[AccountId],
    starting_from: Option<TxId>,
    format: Format,
    config: &crate::Config,
) -> anyhow::Result<()> {
    let context = Context::new(config)?;
    let mut filter = ids
        .iter()
        .copied()
        .fold(AccountFilter::name(name), |filter, id| filter.involves(id));
    if let Some(start) = starting_from {
        filter = filter.starting_from(start);
    }
    let mut stream = context.m10_client.observe_actions(filter).await?;

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
