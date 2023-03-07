use core::fmt;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query::QueryAs, Executor, Postgres};

use crate::{auth::AuthScope, context::Context, error::Error, utils::create_ledger_account};

#[derive(Clone, Deserialize, Serialize)]
pub struct AssetTypeQuery {
    pub r#type: Option<AssetType>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "asset_type")]
pub enum AssetType {
    Regulated,
    Cbdc,
    IndirectCbdc,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Regulated
    }
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Regulated => f.write_str("DRM"),
            _ => f.write_str("CBDC"),
        }
    }
}

impl From<&AssetTypeQuery> for AssetType {
    fn from(v: &AssetTypeQuery) -> Self {
        v.r#type.clone().unwrap_or_default()
    }
}

impl TryFrom<&str> for AssetType {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "regulated" => Ok(AssetType::Regulated),
            "cbdc" => Ok(AssetType::Cbdc),
            "indirect_cbdc" => Ok(AssetType::IndirectCbdc),
            _ => Err(Error::internal_msg(format!(
                "unsupported asset type: {}",
                v
            ))),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,

    pub ledger_account_id: Vec<u8>,

    pub instrument: String,

    pub asset_type: AssetType,

    pub linked_account: i64,

    pub tenant: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Asset {
    pub(crate) async fn new_for_currency(
        currency_code: &str,
        context: &Context,
        name: &str,
        profile_image_url: Option<&str>,
    ) -> Result<Self, Error> {
        let parent_id = context
            .get_currency_regulated_account(currency_code)
            .await?;
        let ledger_account_id = create_ledger_account(
            parent_id,
            0,
            "DRM".to_string(),
            format!("{} (DRM)", name),
            profile_image_url,
            context,
        )
        .await?;
        Ok(Asset {
            id: -1,
            ledger_account_id,
            ..Default::default()
        })
    }

    pub(crate) async fn new_cbdc_for_currency(
        currency_code: &str,
        context: &Context,
        name: &str,
        profile_image_url: Option<&str>,
    ) -> Result<Self, Error> {
        let parent_id = context.get_currency_cbdc_account(currency_code).await?;
        let ledger_account_id = create_ledger_account(
            parent_id,
            0,
            "CBDC".to_string(),
            format!("{} (CBDC)", name),
            profile_image_url,
            context,
        )
        .await?;
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
                asset_type,
                instrument,
                linked_account,
                tenant
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
        )
        .bind(&self.ledger_account_id)
        .bind(&self.asset_type)
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

    pub fn find_by_user_id_instrument_type<'a>(
        user_id: &'a str,
        instrument: &'a str,
        asset_type: AssetType,
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
            ) AND instrument = $2 AND asset_type = $3",
        )
        .bind(user_id)
        .bind(instrument.to_lowercase())
        .bind(asset_type)
    }

    pub fn find_by_account_id_instrument_type(
        account_id: i64,
        instrument: &str,
        asset_type: AssetType,
    ) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "
        SELECT * FROM assets
        WHERE
            linked_account = $1 AND
            instrument = $2 AND asset_type = $3",
        )
        .bind(account_id)
        .bind(instrument.to_lowercase())
        .bind(asset_type)
    }

    pub fn find_by_account_id_instrument_type_scoped(
        account_id: i64,
        instrument: &str,
        asset_type: AssetType,
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
                    ) AND instrument = $3 AND asset_type = $4",
            )
            .bind(account_id)
            .bind(u)
            .bind(instrument.to_lowercase())
            .bind(asset_type)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "
            SELECT * FROM assets
            WHERE
                linked_account = $1 AND tenant = $2 AND instrument = $3 AND asset_type = $4",
            )
            .bind(account_id)
            .bind(tenant)
            .bind(instrument.to_lowercase())
            .bind(asset_type)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub fn find_by_contact_id_instrument_type_scoped(
        contact_id: i64,
        instrument: &str,
        asset_type: AssetType,
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
                    ) AND instrument = $3 AND asset_type = $4",
            )
            .bind(contact_id)
            .bind(u)
            .bind(instrument.to_lowercase())
            .bind(asset_type)),
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
                ) AND instrument = $3 AND asset_type = $4",
            )
            .bind(contact_id)
            .bind(tenant)
            .bind(instrument.to_lowercase())
            .bind(asset_type)),
            _ => Err(Error::unauthorized()),
        }
    }
}
