use super::accounts::CreateAccountOptions;
use crate::context::Context;
use clap::{ArgGroup, Parser};
use m10_sdk::account::AccountId;
use m10_sdk::{sdk, Signer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(group = ArgGroup::new("instrument").requires_all(&["code", "decimals"]).multiple(true), about)]
pub(crate) struct CreateLedgerAccountOptions {
    /// Check for existing account with this id
    #[clap(long)]
    id: Option<String>,
    /// Set owner of the account record
    #[clap(short, long)]
    pub(super) owner: Option<String>,
    /// Set an account name
    #[clap(long)]
    pub(super) name: Option<String>,
    /// Set a name to be shown in transfers as sender
    #[clap(long)]
    pub(super) public_name: Option<String>,
    /// Set profile image url
    #[clap(long)]
    pub(super) profile_image_url: Option<String>,
    /// Set the parent account
    #[clap(short, long, conflicts_with("instrument"))]
    parent_account: Option<String>,
    /// Set freeze state to 'frozen'
    #[clap(short, long)]
    #[serde(default)]
    frozen: bool,
    /// Set account type to 'issuance'
    #[clap(short, long)]
    #[serde(default)]
    issuance: bool,
    /// Currency code
    #[clap(short, long, group = "instrument")]
    code: Option<String>,
    /// Number of relevant currency decimals
    #[clap(short, long, group = "instrument")]
    decimals: Option<u32>,
    /// Currency description
    #[clap(long, group = "instrument")]
    description: Option<String>,
    /// Holding balance limit
    holding_limit: Option<u64>,
}

impl CreateLedgerAccountOptions {
    pub(super) async fn create(&self, config: &crate::Config) -> anyhow::Result<()> {
        let mut context = Context::new(config).await?;

        // If an id was given it will be checked if that particular account exists already.
        // In case it exists no new account will be created otherwise a new account will be
        // created with no guaranty that it gets the given id.
        let mut crete_new_account = true;
        if let Some(id) = &self.id {
            let request = context
                .admin
                .sign_request(sdk::GetAccountRequest {
                    id: hex::decode(id)?,
                })
                .await?;
            if context
                .m10_client
                .get_indexed_account(request)
                .await
                .is_ok()
            {
                eprintln!("ledger account {:?} exists already", id);
                crete_new_account = false
            }
        }

        // Skips creation of a ledger account if id was given and it exists already.
        let account_id = if crete_new_account {
            self.create_ledger_account(&mut context, config.context_id.clone())
                .await?
        } else {
            let id = self.id.as_ref().unwrap();
            AccountId::try_from_be_slice(&hex::decode(id)?)?
        };

        // If any of the Arcadius record fields were set, an Arcadius record will be
        // created with a matching Id.
        if self.owner.is_some() || self.public_name.is_some() {
            let mut account_options = CreateAccountOptions::from(self);
            let arcadius_id = Uuid::from_u128(account_id.as_raw());
            account_options.id = Some(arcadius_id);
            account_options.if_not_exists = self.id.is_some();
            super::store_create::<_, sdk::Account>(&account_options, config).await?;
        } else {
            eprintln!("Created ledger account");
            println!("{}", hex::encode(&account_id.as_raw().to_be_bytes()));
        }
        Ok(())
    }

    async fn create_ledger_account(
        &self,
        context: &mut Context,
        context_id: Vec<u8>,
    ) -> Result<AccountId, anyhow::Error> {
        let request =
            if let Some(parent_id) = self.parent_account.as_ref().map(hex::decode).transpose()? {
                sdk::CreateLedgerAccount {
                    parent_id,
                    issuance: self.issuance,
                    frozen: self.frozen,
                    instrument: None,
                    balance_limit: self.holding_limit.unwrap_or_default(),
                }
            } else {
                sdk::CreateLedgerAccount {
                    parent_id: vec![],
                    issuance: self.issuance,
                    frozen: self.frozen,
                    instrument: Some(sdk::Instrument {
                        code: self
                            .code
                            .clone()
                            .ok_or(anyhow::anyhow!("Instrument code missing"))?,
                        decimal_places: self
                            .decimals
                            .ok_or(anyhow::anyhow!("Instrument decimals missing"))?,
                        description: self.description.clone().unwrap_or_default(),
                    }),
                    balance_limit: self.holding_limit.unwrap_or_default(),
                }
            };
        let response = context.submit_transaction(request, context_id).await??;
        AccountId::try_from_be_slice(&response.account_created).map_err(anyhow::Error::from)
    }
}
