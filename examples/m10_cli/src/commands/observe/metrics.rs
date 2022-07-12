use crate::commands::{
    observe::{parse_account_id, print_doc},
    Format,
};
use crate::context::Context;
use futures_lite::StreamExt;
use m10_sdk::{sdk, Signer};

pub(crate) async fn observe(
    ids: &[String],
    starting_from: Option<sdk::TxId>,
    format: Format,
    config: &crate::Config,
) -> anyhow::Result<()> {
    let context = Context::new(config).await?;
    let ids = ids
        .iter()
        .map(|x| parse_account_id(x))
        .collect::<Result<Vec<_>, _>>()?;
    let request = context
        .admin
        .sign_request(sdk::ObserveAccountsRequest {
            involved_accounts: ids.iter().map(|id| id.to_be_bytes().to_vec()).collect(),
            starting_from,
        })
        .await?;
    let mut updates = context.m10_client.observe_metrics(request).await?;

    while let Some(message) = updates.next().await {
        match message {
            Ok(sample) => {
                print_doc(sample, format)?;
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
