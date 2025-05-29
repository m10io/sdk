use clap::Subcommand;
use m10_sdk::{contract::FinalizedContractExt, prost::Message, sdk::Contract, Signer};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Endorse {
    /// Endorse contracts
    #[command(alias = "c")]
    Contract {
        /// Path to the contract proposal
        #[arg(long)]
        path: String,
        /// Path to output the endorsed contract
        #[arg(short, long)]
        output: Option<String>,
    },
}

impl Endorse {
    pub(super) async fn run(self, context: &Context) -> anyhow::Result<()> {
        match self {
            Endorse::Contract { path, output } => endorse_contract(path, output, context).await,
        }
    }
}

async fn endorse_contract(
    path: String,
    output: Option<String>,
    context: &Context,
) -> anyhow::Result<()> {
    let output = output.ok_or_else(|| anyhow::anyhow!("No output path specified"))?;
    let file = std::fs::read(path)?;
    let mut contract = Contract::decode(file.as_slice())?;

    // TODO: fetch ledger-id
    let ledger_id = "usd.m10";
    let contract_id = hex::encode(contract.id()).to_uppercase();
    let signing_key = context.signer().public_key();
    if contract
        .endorsements
        .iter()
        .filter_map(|e| e.signature.as_ref())
        .any(|sig| sig.public_key == signing_key)
    {
        println!("Contract {} is already endorsed by you", contract_id);
        return Ok(());
    }
    context
        .signer()
        .endorse(&mut contract, ledger_id.to_string())
        .await?;
    let buf = contract.encode_to_vec();
    std::fs::write(output, buf)?;
    println!("Endorsed contract {}", contract_id);
    Ok(())
}
