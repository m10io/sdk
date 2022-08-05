use crate::commands::create::banks::BankAccountRefOptions;
use clap::Parser;
use m10_sdk::sdk;
use m10_sdk::DocumentUpdate;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct UpdateBankOptions {
    pub(super) id: Uuid,
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

impl super::BuildFromOptions for UpdateBankOptions {
    type Document = sdk::Bank;
    fn build_from_options(
        &self,
        builder: &mut DocumentUpdate<Self::Document>,
    ) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(owner_key);
        }
        if let Some(name) = &self.short_name {
            builder.short_name(name.into());
        }
        if let Some(name) = &self.display_name {
            builder.display_name(name.into());
        }
        if !self.accounts.is_empty() {
            builder.accounts(
                self.accounts
                    .iter()
                    .map(BankAccountRefOptions::to_bank_account_ref)
                    .collect::<Result<_, anyhow::Error>>()?,
            );
        }
        Ok(())
    }
}
