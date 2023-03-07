use crate::context::Context;
use clap::Parser;
use m10_sdk::account::AccountId;
use m10_sdk::contract::FinalizedContractExt;
use m10_sdk::{prost::Message, sdk};
use m10_sdk::{StepBuilder, TransferBuilder, WithContext};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct Transfer {
    /// Set sending account
    #[clap(short, long)]
    from_account: AccountId,
    /// Set receiving account
    #[clap(short, long)]
    to_account: AccountId,
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
        let mut context = Context::new(config)?;

        if self.rebalance {
            let indexed_account = context.m10_client.get_account(self.from_account).await?;

            let current_balance = indexed_account.balance;

            let (from, to, amount) = match current_balance {
                b if b > self.amount => (
                    self.from_account,
                    self.to_account,
                    current_balance - self.amount,
                ),
                b if b < self.amount => (
                    self.to_account,
                    self.from_account,
                    self.amount - current_balance,
                ),
                _ => {
                    eprintln!("current balance is already at target");
                    return Ok(());
                }
            };
            self.transfer_funds(from, to, amount, &mut context, config.context_id.clone())
                .await
        } else {
            self.transfer_funds(
                self.from_account,
                self.to_account,
                self.amount,
                &mut context,
                config.context_id.clone(),
            )
            .await
        }
    }

    async fn transfer_funds(
        &self,
        from_account: AccountId,
        to_account: AccountId,
        amount: u64,
        context: &mut Context,
        mut context_id: Vec<u8>,
    ) -> anyhow::Result<()> {
        let contract = if let Some(contract) = &self.contract {
            let file = std::fs::read(contract)?;
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

        let mut step = StepBuilder::new(from_account, to_account, amount);
        if let Some(memo) = &self.memo {
            step = step.metadata(m10_sdk::memo(memo));
        }
        if let Some(contract) = contract {
            step = step.metadata(contract);
        }
        let builder = TransferBuilder::default().step(step).context_id(context_id);

        let tx_id = if self.no_commit {
            context.m10_client.initiate_transfer(builder).await
        } else {
            context.m10_client.transfer(builder).await
        }?;
        eprintln!("created transfer:");
        println!("{}", tx_id);
        Ok(())
    }
}
