use clap::Args;
use m10_sdk::{sdk, DocumentUpdate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::commands::create::banks::{BankAccountRefArgs, BankStatusArgs, Bic, EndpointArg};

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
        long = "account",
        required = false,
        value_parser,
        long_help = "--account Cbdc:<ID>\n--account Drm:<ID>\n\nYou can specify multiple accounts by repeating the --account option. The account-type (CBDC or DRM) is metadata can be used by external applications."
    )]
    accounts: Vec<BankAccountRefArgs>,
    /// Specify operational status (accept names: active|pending|suspended|terminated)
    #[arg(
        long,
        value_parser,
        long_help = "--status active\n--status pending\n--status inactive"
    )]
    status: Option<BankStatusArgs>,
    /// Set ISO-3166-1 alpha-2 country code (e.g. US, PL)
    #[arg(long, alias = "cc")]
    country_code: Option<String>,
    /// Set bank integration endpoint (scheme + host required)
    #[arg(long)]
    endpoint: Option<EndpointArg>,
    /// Set logo URL
    #[arg(short, long, aliases = ["logo", "lu"])]
    logo_url: Option<String>,
    /// Set human-readable description
    #[arg(long)]
    description: Option<String>,
    /// Set BIC/SWIFT code identifying the bank in international transfers.
    #[arg(long, alias = "bsc")]
    bic_swift_code: Option<Bic>,
}

impl super::BuildFromArgs for UpdateBankArgs {
    type Document = sdk::Bank;
    fn build_from_args(self, builder: &mut DocumentUpdate<Self::Document>) -> anyhow::Result<bool> {
        let changed = self.owner.is_some()
            || self.short_name.is_some()
            || self.display_name.is_some()
            || !self.accounts.is_empty()
            || self.status.is_some()
            || self.country_code.is_some()
            || self.endpoint.is_some()
            || self.logo_url.is_some()
            || self.description.is_some()
            || self.bic_swift_code.is_some();

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
        if let Some(status) = self.status {
            builder.status(status.0.into());
        }
        if let Some(country_code) = self.country_code {
            builder.country_code(country_code);
        }
        if let Some(endpoint) = self.endpoint {
            builder.endpoint(endpoint.0);
        }
        if let Some(logo_url) = self.logo_url {
            builder.logo_url(logo_url);
        }
        if let Some(description) = self.description {
            builder.description(description);
        }
        if let Some(bic_swift_code) = self.bic_swift_code {
            builder.bic_swift_code(bic_swift_code.0);
        }
        Ok(changed)
    }
}
