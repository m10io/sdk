use std::str::FromStr;

use crate::collections::PrettyId;
use crate::commands::create::store_create;
use crate::context::Context;
use clap::Args;
use m10_sdk::{sdk, PublicKey};
use parse_display::helpers::regex::Regex;
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

    /// Set a bank name
    #[arg(long, aliases = ["name", "sn"])]
    short_name: Option<String>,

    /// Set a name to be shown in transfers as sender
    #[arg(short, long, alias = "dn")]
    display_name: Option<String>,

    /// Specify account references as `account_type:account_id`, e.g. --accounts Cbdc:abc123
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
    country_code: Option<CountryCodeArg>,

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

impl CreateBankArgs {
    pub(super) async fn create(&self, context: &Context) -> anyhow::Result<()> {
        if let Some(id) = self.id {
            if context
                .ledger_client()
                .get_bank(Vec::from(id))
                .await
                .is_ok()
            {
                eprintln!("bank {} exists already", id);
                return Ok(());
            }
        }

        store_create::<_, sdk::Bank>(self.clone(), context, false).await?;
        Ok(())
    }
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
            status: self.status.map(|s| i32::from(s.0)).unwrap_or_default(),
            country_code: self.country_code.map(|cc| cc.0).unwrap_or_default(),
            endpoint: self.endpoint.map(|ep| ep.0).unwrap_or_default(),
            logo_url: self.logo_url.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            bic_swift_code: self.bic_swift_code.map(|bic| bic.0).unwrap_or_default(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct BankAccountRefArgs {
    pub account_type: crate::collections::banks::BankAccountType,
    pub id: String,
}

impl FromStr for BankAccountRefArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ':');
        let account_type = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing account_type"))?;
        let id = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing account_id"))?;

        let account_type = account_type
            .parse::<crate::collections::banks::BankAccountType>()
            .map_err(|e| anyhow::anyhow!("invalid account_type '{}': {}", account_type, e))?;

        Ok(BankAccountRefArgs {
            account_type,
            id: id.to_string(),
        })
    }
}

impl BankAccountRefArgs {
    pub(crate) fn to_bank_account_ref(&self) -> anyhow::Result<sdk::BankAccountRef> {
        let id = PrettyId::from_str(&self.id)?.to_vec();
        Ok(sdk::BankAccountRef {
            account_type: self.account_type.clone().into(),
            account_id: id,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct BankStatusArgs(pub crate::collections::banks::BankStatus);

impl FromStr for BankStatusArgs {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<crate::collections::banks::BankStatus>()
            .map_err(|e| anyhow::anyhow!("invalid status: {}", e))?;

        Ok(BankStatusArgs(value))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct EndpointArg(pub String);

impl FromStr for EndpointArg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(EndpointArg(String::new()));
        }

        let url = url::Url::parse(s)
            .map_err(|e| anyhow::anyhow!("invalid endpoint URL '{}': {}", s, e))?;

        if url.scheme().is_empty() || url.host_str().is_none() {
            return Err(anyhow::anyhow!("endpoint URL must include scheme and host"));
        }

        Ok(EndpointArg(s.to_string()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bic(pub String);

impl FromStr for Bic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Bic(String::new()));
        }

        let regex = Regex::new(r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$")?;

        if !regex.is_match(s) {
            return Err(anyhow::anyhow!(
                "invalid BIC format (expected 8 or 11 chars: A-Z{{6}} + A-Z0-9{{2}} + optional A-Z0-9{{3}})"
            ));
        }

        Ok(Bic(s.to_string()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CountryCodeArg(pub String);

impl FromStr for CountryCodeArg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(CountryCodeArg(String::new()));
        }
        if s.len() != 2 || !s.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(anyhow::anyhow!(
                "country_code must be ISO-3166 alpha-2 (2 uppercase letters)"
            ));
        }
        Ok(CountryCodeArg(s.to_string()))
    }
}
