use clap::Args;
use m10_sdk::{sdk, DocumentUpdate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::commands::create::banks::BankAccountRefArgs;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct UpdateBankArgs {
    pub(super) id: Uuid,
    /// Set owner of the account [metadata] record
    #[arg(short, long)]
    owner: Option<String>,
    /// Set the bank's ID (e.g. uuid)
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

impl super::BuildFromArgs for UpdateBankArgs {
    type Document = sdk::Bank;
    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<()> {
        if let Some(owner) = &self.owner {
            let owner_key = base64::decode(owner)?;
            builder.owner(owner_key);
        }
        if let Some(name) = self.short_name {
            builder.short_name(name);
        }
        if let Some(name) = self.display_name {
            builder.display_name(name);
        }
        if !self.accounts.is_empty() {
            builder.accounts(
                self.accounts
                    .iter()
                    .map(BankAccountRefArgs::to_bank_account_ref)
                    .collect::<Result<_, anyhow::Error>>()?,
            );
        }
        Ok(())
    }
}
