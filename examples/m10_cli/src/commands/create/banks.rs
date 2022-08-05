use clap::Parser;
use m10_sdk::sdk;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug, str::FromStr};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateBankOptions {
    /// Ignore error if item exists
    #[clap(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set an bank's id
    #[clap(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set owner of the account record
    #[clap(short, long)]
    owner: Option<String>,
    /// Set an bank name
    #[clap(short, long)]
    short_name: Option<String>,
    /// Set a name to be shown in transfers as sender
    #[clap(short, long)]
    display_name: Option<String>,
    /// Set account refs associated with a bank
    #[clap(short, long, required = false, parse(try_from_str = BankAccountRefOptions::parse))]
    accounts: Vec<BankAccountRefOptions>,
}

impl super::BuildFromOptions for CreateBankOptions {
    type Document = sdk::Bank;
    fn build_from_options(&self, default_owner: Vec<u8>) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self
            .owner
            .as_ref()
            .map_or::<Result<Vec<u8>, _>, _>(Ok(default_owner), base64::decode)?;
        Ok(sdk::Bank {
            id,
            owner,
            short_name: self.short_name.clone().unwrap_or_default(),
            display_name: self.display_name.clone().unwrap_or_default(),
            accounts: self
                .accounts
                .iter()
                .map(BankAccountRefOptions::to_bank_account_ref)
                .collect::<Result<_, anyhow::Error>>()?,
        })
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct BankAccountRefOptions {
    #[clap(short, long)]
    account_type: crate::collections::banks::BankAccountType,
    #[clap(short, long)]
    id: String,
}

impl BankAccountRefOptions {
    pub(crate) fn parse(s: &str) -> Result<Self, clap::Error> {
        Self::try_parse_from(s.split_whitespace())
    }

    pub(crate) fn to_bank_account_ref(&self) -> anyhow::Result<sdk::BankAccountRef> {
        let id = hex::decode(&self.id)?;
        Ok(sdk::BankAccountRef {
            account_type: self.account_type.clone().into(),
            account_id: id,
        })
    }
}

impl FromStr for BankAccountRefOptions {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_ascii_whitespace();
        let rule = Self::try_parse_from(args)?;
        Ok(rule)
    }
}
