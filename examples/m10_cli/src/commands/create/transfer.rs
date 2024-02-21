use clap::Args;
use m10_sdk::{
    account::AccountId, contract::FinalizedContractExt, prost::Message, sdk, StepBuilder,
    TransferBuilder, WithContext,
};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateTransferArgs {
    /// Set sending account
    #[arg(short, long)]
    from_account: AccountId,
    /// Set receiving account
    #[arg(short, long)]
    to_account: AccountId,
    /// Set amount
    #[arg(short, long)]
    amount: u64,
    /// Set a memo
    #[arg(short, long)]
    memo: Option<String>,
    /// Attach a contract to a transfer
    #[arg(long)]
    contract: Option<String>,
    /// If set account will rebalanced to the given amount
    #[arg(short, long)]
    rebalance: bool,
    /// If set the transfer will only be initiated but not committed
    #[arg(short, long)]
    no_commit: bool,
}

impl CreateTransferArgs {
    pub async fn create(&self, context: &Context) -> anyhow::Result<()> {
        if self.rebalance {
            let indexed_account = context
                .ledger_client()
                .get_account(self.from_account)
                .await?;

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
            self.transfer_funds(from, to, amount, context).await
        } else {
            self.transfer_funds(self.from_account, self.to_account, self.amount, context)
                .await
        }
    }

    async fn transfer_funds(
        &self,
        from_account: AccountId,
        to_account: AccountId,
        amount: u64,
        context: &Context,
    ) -> anyhow::Result<()> {
        let contract = if let Some(contract) = &self.contract {
            let file = std::fs::read(contract)?;
            let contract = sdk::Contract::decode(file.as_slice())?;
            Some(contract)
        } else {
            None
        };

        let context_id = if let Some(contract) = &contract {
            if !context.context_id().is_empty() {
                anyhow::bail!(
                    "Adding a contract to a transfer would override the provided context id"
                );
            } else {
                contract.id()
            }
        } else {
            context.context_id()
        };

        let mut step = StepBuilder::new(from_account, to_account, amount);
        if let Some(memo) = &self.memo {
            step = step.metadata(m10_sdk::memo(memo));
        }
        if let Some(contract) = contract {
            step = step.metadata(contract);
        }
        let builder = TransferBuilder::default().step(step).context_id(context_id);

        let tx_id = if self.no_commit {
            m10_sdk::initiate_transfer(context.ledger_client(), builder).await
        } else {
            m10_sdk::transfer(context.ledger_client(), builder).await
        }?;
        eprintln!("created transfer:");
        println!("{}", tx_id);
        Ok(())
    }
}
