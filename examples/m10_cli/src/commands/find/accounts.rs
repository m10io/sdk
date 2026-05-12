use std::io::{self, BufReader};

use clap::Args;
use m10_sdk::{
    account::AccountId, AccountMetadataFilter, Format, PageBuilder, PrettyPrint, PublicKey,
};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct FindAccountArgs {
    /// Set name filter
    #[arg(short, long, group = "filter")]
    name: Option<String>,
    /// Set public-name filter
    #[arg(long = "public-name", group = "filter")]
    public_name: Option<String>,
    /// Set owner filter
    #[arg(short, long, group = "filter")]
    owner: Option<PublicKey>,
    /// Set ISIN filter (exact match)
    #[arg(long, group = "filter")]
    isin: Option<String>,
    /// Set DTI filter (exact match)
    #[arg(long, group = "filter")]
    dti: Option<String>,
    /// Set Issuer Bank Id filter (exact match)
    #[arg(long, group = "filter")]
    issuer_bank_id: Option<String>,
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

    fn filter_from_options(&self) -> anyhow::Result<PageBuilder<Vec<u8>, AccountMetadataFilter>> {
        if let Some(name) = &self.name {
            Ok(PageBuilder::filter(AccountMetadataFilter::Name(
                name.to_string(),
            )))
        } else if let Some(public_name) = &self.public_name {
            Ok(PageBuilder::filter(AccountMetadataFilter::PublicName(
                public_name.to_string(),
            )))
        } else if let Some(owner) = &self.owner {
            Ok(PageBuilder::filter(AccountMetadataFilter::Owner(
                owner.clone(),
            )))
        } else if let Some(isin) = &self.isin {
            Ok(PageBuilder::filter(AccountMetadataFilter::Isin(
                isin.to_string(),
            )))
        } else if let Some(dti) = &self.dti {
            Ok(PageBuilder::filter(AccountMetadataFilter::Dti(
                dti.to_string(),
            )))
        } else if let Some(issuer_bank_id) = &self.issuer_bank_id {
            Ok(PageBuilder::filter(AccountMetadataFilter::IssuerBankId(
                issuer_bank_id.to_string(),
            )))
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
