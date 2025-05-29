use std::str::FromStr;

use clap::{Args, Parser};
use m10_sdk::{sdk, PublicKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateBankArgs {
    /// Ignore error if item exists
    #[arg(short = 'e', long)]
    #[serde(default)]
    pub(super) if_not_exists: bool,
    /// Set the bank's ID (e.g. uuid)
    #[arg(short, long)]
    pub(super) id: Option<Uuid>,
    /// Set owner of the account [metadata] record
    #[arg(short, long)]
    owner: Option<PublicKey>,
    /// Set an bank name
    #[arg(long, aliases = ["name", "sn"])]
    short_name: Option<String>,
    /// Set a name to be shown in transfers as sender
    #[arg(short, long, alias = "dn")]
    display_name: Option<String>,
    /// Set account type associated with a ledger account
    #[arg(
        short,
        long,
        required = false,
        long_help = " --accounts 'accounts --account-type Cbdc:Drm --id <ID>'. The account-type (CBDC or DRM) is metadata can be used by external applications."
    )]
    accounts: Vec<BankAccountRefArgs>,
}

impl super::BuildFromArgs for CreateBankArgs {
    type Document = sdk::Bank;
    fn build_from_options(self, default_owner: PublicKey) -> Result<Self::Document, anyhow::Error> {
        let id = self.id.unwrap_or_else(Uuid::new_v4).as_bytes().to_vec();
        let owner = self.owner.unwrap_or(default_owner).0;
        Ok(sdk::Bank {
            id,
            owner,
            short_name: self.short_name.unwrap_or_default(),
            display_name: self.display_name.unwrap_or_default(),
            accounts: self
                .accounts
                .iter()
                .map(BankAccountRefArgs::to_bank_account_ref)
                .collect::<Result<_, anyhow::Error>>()?,
        })
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct BankAccountRefArgs {
    #[arg(short, long)]
    account_type: crate::collections::banks::BankAccountType,
    #[arg(short, long)]
    id: String,
}

impl FromStr for BankAccountRefArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_ascii_whitespace();
        let rule = Self::try_parse_from(args)?;
        Ok(rule)
    }
}

impl BankAccountRefArgs {
    pub(crate) fn to_bank_account_ref(&self) -> anyhow::Result<sdk::BankAccountRef> {
        let id = hex::decode(&self.id)?;
        Ok(sdk::BankAccountRef {
            account_type: self.account_type.clone().into(),
            account_id: id,
        })
    }
}
