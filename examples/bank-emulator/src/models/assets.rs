use m10_protos::sdk;
use m10_sdk::Signer;
use sqlx::{postgres::PgArguments, query::QueryAs, Executor, Postgres};

use crate::{
    auth::{AuthModel, AuthScope, User, Verb},
    config::CurrencyConfig,
    context::Context,
    error::Error,
    utils::submit_transaction,
};

#[derive(sqlx::FromRow, Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Asset {
    pub id: i64,

    pub ledger_account_id: Vec<u8>,

    pub instrument: String,

    pub linked_account: i64,

    pub tenant: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Asset {
    pub(crate) async fn new_for_currency(
        currency: &CurrencyConfig,
        context: &Context,
        name: &str,
        profile_image_url: Option<&str>,
    ) -> Result<Self, Error> {
        let ledger_account_id = currency.ledger_account_id(context).await?;
        let payload = sdk::CreateLedgerAccount {
            parent_id: ledger_account_id.to_vec(),
            frozen: false,
            ..Default::default()
        };
        let ledger_account_id = submit_transaction(payload, vec![], context)
            .await?
            .account_created;
        let account_doc = sdk::Account {
            id: ledger_account_id.clone(),
            owner: context.signer.public_key().to_vec(),
            name: format!("{} Wallet", currency.code.to_uppercase()),
            public_name: name.into(),
            profile_image_url: profile_image_url.unwrap_or_default().to_string(),
        };
        let payload = sdk::Operation::insert(account_doc);
        submit_transaction(payload, vec![], context).await?;
        Ok(Asset {
            id: -1,
            ledger_account_id,
            ..Default::default()
        })
    }

    pub async fn insert(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "INSERT INTO assets (
                ledger_account_id,
                instrument,
                linked_account,
                tenant
            ) VALUES ($1, $2, $3, $4)
            RETURNING *",
        )
        .bind(&self.ledger_account_id)
        .bind(self.instrument.to_lowercase())
        .bind(self.linked_account)
        .bind(self.tenant.clone());
        let asset: Asset = query.fetch_one(txn).await?;
        *self = asset;
        Ok(())
    }

    pub fn find_by_user_id(user_id: &str) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "
            SELECT * FROM assets a
            WHERE
                EXISTS (
                    SELECT
                        1
                    FROM
                        contacts c
                    WHERE
                        a.linked_account = c.account_id AND c.user_id = $1
                )",
        )
        .bind(user_id)
    }

    pub fn find_by_account_id_scoped(
        account_id: i64,
        scope: AuthScope,
    ) -> Result<QueryAs<'static, Postgres, Self, PgArguments>, Error> {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "
                SELECT * FROM assets a
                WHERE
                    a.linked_account = $1 AND
                    EXISTS (
                        SELECT
                            1
                        FROM
                            contacts c
                        WHERE
                            c.account_id = $1 AND c.user_id = $2 
                    )",
            )
            .bind(account_id)
            .bind(u)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "
            SELECT * FROM assets
            WHERE
                tenant = $2 AND linked_account = $1",
            )
            .bind(account_id)
            .bind(tenant)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub fn find_by_contact_id_scoped(
        contact_id: i64,
        scope: AuthScope,
    ) -> Result<QueryAs<'static, Postgres, Self, PgArguments>, Error> {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "
                SELECT * FROM assets a
                WHERE
                    EXISTS (
                        SELECT
                            1
                        FROM
                            contacts c
                        WHERE
                            a.linked_account = c.account_id AND c.id = $1 AND c.user_id = $2 
                    )",
            )
            .bind(contact_id)
            .bind(u)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "
            SELECT * FROM assets a
            WHERE
                a.tenant = $2 AND
                EXISTS (
                    SELECT
                        1
                    FROM
                        contacts c
                    WHERE
                        a.linked_account = c.account_id AND c.id = $1
                )",
            )
            .bind(contact_id)
            .bind(tenant)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub fn find_by_ledger_account_id(id: &[u8]) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM assets WHERE ledger_account_id = $1").bind(id)
    }

    pub fn find_by_user_id_instrument<'a>(
        user_id: &'a str,
        instrument: &'a str,
    ) -> QueryAs<'a, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "
        SELECT * FROM assets a
        WHERE
            EXISTS (
                SELECT
                    1
                FROM
                    contacts c
                WHERE
                    a.linked_account = c.account_id AND c.user_id = $1
            ) AND instrument = $2",
        )
        .bind(user_id)
        .bind(instrument.to_lowercase())
    }

    pub fn find_by_account_id_instrument_scoped(
        account_id: i64,
        instrument: &str,
        scope: AuthScope,
    ) -> Result<QueryAs<'_, Postgres, Self, PgArguments>, Error> {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "
                SELECT * FROM assets a
                WHERE
                    a.linked_account = $1 AND
                    EXISTS (
                        SELECT
                            1
                        FROM
                            contacts c
                        WHERE
                            c.account_id = $1 AND c.user_id = $2 
                    ) AND instrument = $3",
            )
            .bind(account_id)
            .bind(u)
            .bind(instrument.to_lowercase())),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "
            SELECT * FROM assets
            WHERE
                linked_account = $1 AND tenant = $2 AND instrument = $3",
            )
            .bind(account_id)
            .bind(tenant)
            .bind(instrument.to_lowercase())),
            _ => Err(Error::unauthorized()),
        }
    }

    pub fn find_by_contact_id_instrument_scoped(
        contact_id: i64,
        instrument: &str,
        scope: AuthScope,
    ) -> Result<QueryAs<'_, Postgres, Self, PgArguments>, Error> {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "
                SELECT * FROM assets a
                WHERE
                    EXISTS (
                        SELECT
                            1
                        FROM
                            contacts c
                        WHERE
                            a.linked_account = c.account_id AND c.id = $1 AND c.user_id = $2 
                    ) AND instrument = $3",
            )
            .bind(contact_id)
            .bind(u)
            .bind(instrument.to_lowercase())),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "
            SELECT * FROM assets a
            WHERE
                a.tenant = $2 AND
                EXISTS (
                    SELECT
                        1
                    FROM
                        contacts c
                    WHERE
                        a.linked_account = c.account_id AND c.id = $1
                ) AND instrument = $3",
            )
            .bind(contact_id)
            .bind(tenant)
            .bind(instrument.to_lowercase())),
            _ => Err(Error::unauthorized()),
        }
    }
}

pub struct AssetAuth;

impl AuthModel for AssetAuth {
    fn is_authorized(&self, verb: Verb, user: &User) -> Result<(), Error> {
        user.authorize(&"assets".into(), verb).map(|_| ())
    }

    fn auth_scope(&self, verb: Verb, user: &User) -> AuthScope {
        user.query_scope(&"assets".into(), verb)
    }
}
