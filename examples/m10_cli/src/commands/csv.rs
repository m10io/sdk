use crate::context::Context;
use clap::Parser;
use m10_sdk::account::AccountId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(super) enum CsvSubcommands {
    /// Get balance for each account by id (std input and output
    ListBalances(CsvListAccountIdArgs),
}

impl CsvSubcommands {
    pub(super) async fn csv(&self, context: &Context) -> anyhow::Result<()> {
        match self {
            CsvSubcommands::ListBalances(op) => op.list_balances(context).await,
        }
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct CsvListAccountIdArgs {}

#[derive(Debug, Deserialize)]
struct Account {
    id: AccountId,
}
// Is code (name/currency) needed?
#[derive(Debug, Serialize)]
struct AccountBalance {
    id: AccountId,
    balance: u64,
}

impl CsvListAccountIdArgs {
    pub(super) async fn list_balances(&self, context: &Context) -> anyhow::Result<()> {
        let mut output = csv::Writer::from_writer(io::stdout());
        let mut rdr = csv::Reader::from_reader(io::stdin());
        let client = context.ledger_client();
        for result in rdr.deserialize() {
            let record: Account = result?;
            let account = client.get_account(record.id).await?;
            output.serialize(AccountBalance {
                id: record.id,
                balance: account.balance,
            })?;
        }
        Ok(())
    }
}
