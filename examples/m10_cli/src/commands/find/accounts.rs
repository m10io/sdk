use std::io::{self, BufReader};

use clap::Args;
use m10_sdk::{account::AccountId, Format, NameOrOwnerFilter, PageBuilder, PrettyPrint, PublicKey};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct FindAccountArgs {
    /// Set name filter
    #[arg(short, long, group = "filter")]
    name: Option<String>,
    /// Set owner filter
    #[arg(short, long, group = "filter")]
    owner: Option<PublicKey>,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[arg(short, long, default_value = "raw")]
    #[serde(default)]
    format: Format,
}

impl FindAccountArgs {
    pub(crate) async fn find(&self, context: &Context) -> anyhow::Result<()> {
        context
            .ledger_client()
            .list_account_metadata(self.filter_from_options()?)
            .await?
            .print(self.format)?;
        Ok(())
    }

    fn filter_from_options(&self) -> anyhow::Result<PageBuilder<Vec<u8>, NameOrOwnerFilter>> {
        if let Some(name) = &self.name {
            Ok(PageBuilder::filter(NameOrOwnerFilter::Name(
                name.to_string(),
            )))
        } else if let Some(owner) = &self.owner {
            Ok(PageBuilder::filter(NameOrOwnerFilter::Owner(owner.clone())))
        } else {
            Err(anyhow::anyhow!("missing filter"))
        }
    }
}

#[derive(Debug, Deserialize)]
struct Account {
    id: AccountId,
}

#[derive(Debug, Serialize)]
struct AccountBalance {
    id: AccountId,
    balance: u64,
}

pub(super) async fn list_balances(format: Format, context: &Context) -> anyhow::Result<()> {
    let client = context.ledger_client();
    if format == Format::Csv {
        let mut output = csv::Writer::from_writer(io::stdout());
        let mut rdr = csv::Reader::from_reader(io::stdin());
        for result in rdr.deserialize() {
            let record: Account = result?;
            let account = client.get_account(record.id).await?;
            output.serialize(AccountBalance {
                id: record.id,
                balance: account.balance,
            })?;
        }
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let rdr = BufReader::new(handle);
        let ids: Vec<Account> = match format {
            Format::Json => serde_json::from_reader(rdr)?,
            Format::Yaml => serde_yml::from_reader(rdr)?,
            Format::Raw => ron::de::from_reader(rdr)?,
            Format::Csv => vec![], // Not reachable
        };
        for Account { id } in ids {
            let account = client.get_account(id).await?;
            AccountBalance {
                id,
                balance: account.balance,
            }
            .print(format)?;
        }
    }
    Ok(())
}
