use clap::Parser;
use m10_sdk::contract::FinalizedContractExt;
use m10_sdk::{prost::Message, sdk::Contract, Signer};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::context::Context;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct EndorseContractOptions {
    /// Path to the contract proposal
    #[clap(short, long)]
    path: String,
    /// Path to output the endorsed contract
    #[clap(short, long)]
    output: Option<String>,
}

impl EndorseContractOptions {
    pub(in crate::commands) async fn endorse(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config).await?;

        let output = self
            .output
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No output path specified"))?;
        let file = std::fs::read(&self.path)?;
        let mut contract = Contract::decode(file.as_slice())?;

        // TODO: fetch ledger-id
        let ledger_id = "usd.m10";
        let contract_id = hex::encode(&contract.id()).to_uppercase();

        if contract
            .endorsements
            .iter()
            .filter_map(|e| e.signature.as_ref())
            .any(|sig| sig.public_key == context.admin.public_key())
        {
            println!("Contract {} is already endorsed by you", contract_id);
            return Ok(());
        }
        context
            .admin
            .endorse(&mut contract, ledger_id.to_string())
            .await?;
        let buf = contract.encode_to_vec();
        std::fs::write(output, buf)?;
        println!("Endorsed contract {}", contract_id);
        Ok(())
    }
}
