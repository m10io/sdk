use crate::context::Context;
use clap::Parser;
use m10_sdk::contract::FinalizedContractExt;
use m10_sdk::Signer;
use m10_sdk::{
    prost::Message, sdk, sdk::transaction_data::Data, sdk::transaction_error::Code, Metadata,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct Transfer {
    /// Set sending account
    #[clap(short, long)]
    from_account: Option<String>,
    /// Set receiving account
    #[clap(short, long)]
    to_account: Option<String>,
    /// Set amount
    #[clap(short, long)]
    amount: u64,
    /// Set a memo
    #[clap(short, long)]
    memo: Option<String>,
    /// Attach a contract to a transfer
    #[clap(short, long)]
    contract: Option<String>,
    /// If set account will rebalanced to the given amount
    #[clap(short, long)]
    rebalance: bool,
    /// If set the transfer will only be initiated but not committed
    #[clap(short, long)]
    no_commit: bool,
}

impl Transfer {
    pub async fn create(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;

        let account = hex::decode(
            self.from_account
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("missing account"))?,
        )?;
        let other_account = hex::decode(
            self.to_account
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("missing target account"))?,
        )?;

        if self.rebalance {
            let request = context
                .admin
                .sign_request(sdk::GetAccountRequest {
                    id: account.clone(),
                })
                .await?;
            let indexed_account = context.m10_client.get_indexed_account(request).await?;

            let current_balance = indexed_account.balance;

            let (from, to, amount) = match current_balance {
                b if b > self.amount => (account, other_account, current_balance - self.amount),
                b if b < self.amount => (other_account, account, self.amount - current_balance),
                _ => {
                    eprintln!("current balance is already at target");
                    return Ok(());
                }
            };
            self.transfer_funds(from, to, amount, &mut context, config.context_id.clone())
                .await
        } else {
            self.transfer_funds(
                account,
                other_account,
                self.amount,
                &mut context,
                config.context_id.clone(),
            )
            .await
        }
    }

    async fn transfer_funds(
        &self,
        from_account: Vec<u8>,
        to_account: Vec<u8>,
        amount: u64,
        context: &mut Context,
        mut context_id: Vec<u8>,
    ) -> anyhow::Result<()> {
        let contract = if let Some(contract) = &self.contract {
            let file = std::fs::read(&contract)?;
            let contract = sdk::Contract::decode(file.as_slice())?;
            Some(contract)
        } else {
            None
        };

        if let Some(contract) = &contract {
            if !context_id.is_empty() {
                anyhow::bail!(
                    "Adding a contract to a transfer would override the provided context id"
                );
            } else {
                context_id = contract.id();
            }
        }

        let memo = self.memo.as_ref().map(|message| m10_sdk::memo(message));
        let contract = contract.map(|c| c.any());
        let metadata = memo.into_iter().chain(contract.into_iter()).collect();
        let transfer = sdk::CreateTransfer {
            transfer_steps: vec![sdk::TransferStep {
                from_account_id: from_account,
                to_account_id: to_account,
                amount,
                metadata,
            }],
        };
        let transfer = if self.no_commit {
            Data::InitiateTransfer(transfer)
        } else {
            Data::Transfer(transfer)
        };
        let response = context.submit_transaction(transfer, context_id).await?;
        response
            .map(|res| {
                eprintln!("created transfer:");
                println!("{}", res.tx_id);
            })
            .map_err(|err| {
                let err_type = Code::from_i32(err.code).unwrap_or(Code::Unknown);
                eprintln!("Could not create transfer: {:?} {}", err_type, err.message);
                anyhow::anyhow!("failed transfer")
            })
    }
}
