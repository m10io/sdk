use crate::collections::accounts::AccountEntry;
use crate::context::Context;
use crate::utils::print_items;
use clap::Parser;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindAccountOptions {
    /// Set owner filter
    #[clap(short, long)]
    owner: Option<String>,
    /// Include ledger account info in result
    #[clap(short = 'l', long)]
    include_ledger: bool,
    /// Set output format (one of 'json', 'yaml', 'raw')
    #[clap(short, long, default_value = "raw")]
    #[serde(default = "super::Format::default")]
    format: super::Format,
}

impl FindAccountOptions {
    pub(crate) async fn find(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;
        let owner = self
            .owner
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing filter"))?;
        let request = sdk::ListAccountsRequest {
            filter: Some(sdk::list_accounts_request::Filter::Owner(base64::decode(
                owner,
            )?)),
            page: None,
        };
        let request = context.admin.sign_request(request).await?;
        let docs = context.m10_client.list_accounts(request).await?;

        let mut result: Vec<AccountEntry> = vec![];
        for account_record in docs.accounts {
            let indexed_account = if self.include_ledger {
                let request = context
                    .admin
                    .sign_request(sdk::GetAccountRequest {
                        id: account_record.id.clone(),
                    })
                    .await?;
                let indexed_account = context.m10_client.get_indexed_account(request).await?;
                Some(indexed_account)
            } else {
                None
            };
            let data = AccountEntry {
                id: hex::encode(&account_record.id),
                owner: base64::encode(&account_record.owner),
                frozen: indexed_account.as_ref().map(|a| a.frozen),
                name: account_record.name.to_owned(),
                public_name: account_record.public_name.to_owned(),
                balance: indexed_account.as_ref().map(|a| a.balance.to_owned()),
                issued_balance: indexed_account
                    .as_ref()
                    .and_then(|a| a.issuance.as_ref())
                    .map(|i| i.issued_balance),
                profile_image_url: account_record.profile_image_url,
                instrument: indexed_account
                    .as_ref()
                    .and_then(|a| a.instrument.to_owned().map(|i| i.into())),
            };
            result.push(data);
        }
        print_items(result, self.format)?;
        Ok(())
    }
}
