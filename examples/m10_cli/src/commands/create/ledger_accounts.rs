use clap::{ArgGroup, Args};
use m10_sdk::account::AccountId;
use m10_sdk::{sdk, AccountBuilder, PublicKey, WithContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::Context;

use super::accounts::CreateAccountMetadataArgs;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("instrument").requires_all(&["code", "decimals"]).multiple(true))]
pub(crate) struct CreateLedgerAccountArgs {
    /// Check for existing account with this id
    #[arg(long)]
    id: Option<AccountId>,
    /// Set owner of the account record
    #[arg(short, long)]
    pub(super) owner: Option<PublicKey>,
    /// Set an account name
    #[arg(long)]
    pub(super) name: Option<String>,
    /// Set a name to be shown in transfers as sender
    #[arg(long, alias = "pn")]
    pub(super) public_name: Option<String>,
    /// Set profile image url
    #[arg(long, aliases = ["image", "pi"])]
    pub(super) profile_image_url: Option<String>,
    /// Set the parent account
    #[arg(long, aliases = ["parent", "pa"], conflicts_with("instrument"))]
    parent_account: Option<AccountId>,
    /// Set freeze state to 'frozen'
    #[arg(short, long)]
    #[serde(default)]
    frozen: bool,
    /// Set account type to 'issuance'
    #[arg(short, long)]
    #[serde(default)]
    issuance: bool,
    /// Currency code
    #[arg(long, aliases = ["currency", "symbol", "cs", "cc"], group = "instrument")]
    code: Option<String>,
    /// Number of relevant currency decimals
    #[arg(short, long, group = "instrument")]
    decimals: Option<u32>,
    /// Currency description
    #[arg(long, alias = "desc", group = "instrument")]
    description: Option<String>,
    /// Holding balance limit
    #[arg(short = 'l', long, aliases = ["limit", "hl"])]
    holding_limit: Option<u64>,
}

impl CreateLedgerAccountArgs {
    pub(super) async fn create(&self, context: &Context) -> anyhow::Result<()> {
        // If an id was given it will be checked if that particular account exists already.
        // In case it exists no new account will be created otherwise a new account will be
        // created with no guaranty that it gets the given id.
        let mut create_new_account = true;
        if let Some(id) = self.id {
            if context.ledger_client().get_account(id).await.is_ok() {
                eprintln!("ledger account {:?} exists already", id);
                create_new_account = false
            }
        }

        // Skips creation of a ledger account if id was given and it exists already.
        let account_id = if create_new_account {
            self.create_ledger_account(context).await?
        } else {
            self.id.unwrap()
        };

        // If any of the Arcadius record fields were set, an Arcadius record will be
        // created with a matching Id.
        if self.owner.is_some() || self.public_name.is_some() {
            let mut account_options = CreateAccountMetadataArgs::from(self);
            let arcadius_id = Uuid::from_u128(account_id.as_raw());
            account_options.id = Some(arcadius_id);
            account_options.if_not_exists = self.id.is_some();
            super::store_create::<_, sdk::AccountMetadata>(account_options, context).await?;
        } else {
            eprintln!("Created ledger account");
            println!("{}", hex::encode(account_id.as_raw().to_be_bytes()));
        }
        Ok(())
    }

    async fn create_ledger_account(&self, context: &Context) -> Result<AccountId, anyhow::Error> {
        let builder = if let Some(parent_id) = self.parent_account {
            AccountBuilder::parent(parent_id)
        } else {
            AccountBuilder::root(
                self.code
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Instrument code missing"))?,
                self.decimals
                    .ok_or_else(|| anyhow::anyhow!("Instrument decimals missing"))?,
                self.description.clone(),
            )
        }
        .issuance(self.issuance)
        .frozen(self.frozen)
        .balance_limit(self.holding_limit.unwrap_or_default())
        .context_id(context.context_id());
        let (_tx_id, account_id) =
            m10_sdk::create_account(context.ledger_client(), builder).await?;
        Ok(account_id)
    }
}
