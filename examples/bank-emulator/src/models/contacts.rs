use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query::QueryAs, Executor, Postgres};
use uuid::Uuid;

use crate::{auth::AuthScope, error::Error};

use super::{Asset, NextPageToken};

#[derive(sqlx::Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "contact_type")]
pub enum ContactType {
    Individual,
    LegalEntity,
}

impl Default for ContactType {
    fn default() -> Self {
        ContactType::Individual
    }
}

#[derive(sqlx::FromRow, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,

    pub user_id: String,

    #[serde(skip)]
    pub co_owner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,

    pub contact_data: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_reference: Option<serde_json::Value>,

    pub contact_type: ContactType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<i64>,

    #[serde(skip)]
    pub rbac_role: Uuid,

    pub account_set: Uuid,

    pub tenant: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<&Contact> for NextPageToken<i64> {
    fn from(value: &Contact) -> NextPageToken<i64> {
        let Contact { id, .. } = value;
        NextPageToken { id: *id }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateContactRequest {
    pub tenant: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,

    pub contact_data: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<ContactType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<String>>,
}

impl From<CreateContactRequest> for Contact {
    fn from(value: CreateContactRequest) -> Self {
        let CreateContactRequest {
            tenant,
            account_id,
            contact_data,
            contact_type,
            ..
        } = value;
        Self {
            account_id,
            contact_data,
            contact_type: contact_type.unwrap_or_default(),
            rbac_role: Uuid::new_v4(),
            tenant,
            ..Default::default()
        }
    }
}

impl Contact {
    pub async fn insert(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "INSERT INTO contacts (
                user_id,
                co_owner,
                account_id,
                contact_data,
                bank_reference,
                contact_type,
                rbac_role,
                account_set,
                tenant
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *",
        )
        .bind(&self.user_id)
        .bind(&self.co_owner)
        .bind(self.account_id)
        .bind(&self.contact_data)
        .bind(&self.bank_reference)
        .bind(&self.contact_type)
        .bind(self.rbac_role)
        .bind(self.account_set)
        .bind(&self.tenant);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub async fn update(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE contacts
            SET contact_data = $1,
                updated_at = $2
            WHERE id = $3 AND retired_since IS NULL
            RETURNING *",
        )
        .bind(&self.contact_data)
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub async fn update_bank_reference(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE contacts
            SET bank_reference = $1,
                updated_at = $2
            WHERE id = $3 AND retired_since IS NULL
            RETURNING *",
        )
        .bind(&self.bank_reference)
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub fn find_by_user_id(user_id: &str) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM contacts WHERE user_id = $1 AND retired_since IS NULL")
            .bind(user_id)
    }

    pub fn find_by_id_scoped(
        id: i64,
        scope: AuthScope,
    ) -> Result<QueryAs<'static, Postgres, Self, PgArguments>, Error> {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "SELECT * FROM contacts WHERE id = $1 AND user_id = $2 AND retired_since IS NULL",
            )
            .bind(id)
            .bind(u)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "SELECT * FROM contacts WHERE id = $1 AND tenant = $2",
            )
            .bind(id)
            .bind(tenant)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub fn find_by_account_id(account_id: i64) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM contacts WHERE account_id = $1").bind(account_id)
    }

    pub async fn list_page_own(
        limit: i32,
        last_token: Option<NextPageToken<i64>>,
        user_id: &str,
        executor: impl Executor<'_, Database = Postgres>,
    ) -> Result<Vec<Self>, Error> {
        let mut query_str: String = "SELECT * FROM contacts WHERE user_id = $1".to_string();
        let query = if let Some(NextPageToken { id, .. }) = last_token {
            query_str += " AND id < $3 ORDER BY id DESC LIMIT $2";
            sqlx::query_as(&query_str)
                .bind(user_id)
                .bind(limit)
                .bind(id)
        } else {
            query_str += " ORDER BY id DESC LIMIT $2";
            sqlx::query_as(&query_str).bind(user_id).bind(limit)
        };
        Ok(query.fetch_all(executor).await?)
    }

    pub async fn list_page_by_tenant(
        limit: i32,
        last_token: Option<NextPageToken<i64>>,
        tenant: &str,
        executor: impl Executor<'_, Database = Postgres>,
    ) -> Result<Vec<Self>, Error> {
        let mut query_str: String = "SELECT * FROM contacts WHERE tenant = $1".to_string();
        let query = if let Some(NextPageToken { id, .. }) = last_token {
            query_str += " AND id < $3 ORDER BY id DESC LIMIT $2";
            sqlx::query_as(&query_str).bind(tenant).bind(limit).bind(id)
        } else {
            query_str += " ORDER BY id DESC LIMIT $2";
            sqlx::query_as(&query_str).bind(tenant).bind(limit)
        };
        Ok(query.fetch_all(executor).await?)
    }

    pub fn get_asset(id: i64, instrument: &str) -> QueryAs<'_, Postgres, Asset, PgArguments> {
        sqlx::query_as(
            "SELECT a.*
                 FROM assets a, contacts c
                 WHERE
                   c.id = $1 AND 
                   a.account_id = c.account_id AND
                   a.instrument = $2",
        )
        .bind(id)
        .bind(instrument)
    }

    pub fn get_assets(id: i64) -> QueryAs<'static, Postgres, Asset, PgArguments> {
        sqlx::query_as(
            "SELECT a.*
                 FROM assets a, contacts c
                 WHERE
                   c.id = $1 AND
                   a.account_id = c.account_id",
        )
        .bind(id)
    }

    pub fn get_notification_preferences_scoped(
        id: i64,
        scope: AuthScope,
    ) -> Result<QueryAs<'static, Postgres, super::NotificationPreferences, PgArguments>, Error>
    {
        match scope {
            AuthScope::Own(u) => Ok(sqlx::query_as(
                "SELECT n.* FROM contacts, notification_preferences n 
                WHERE id = $1 AND user_id = $2 
                    AND retired_since IS NULL
                    AND n.contacts_id = id",
            )
            .bind(id)
            .bind(u)),
            AuthScope::Tenant(tenant) => Ok(sqlx::query_as(
                "SELECT n.* FROM contacts, notification_preferences n 
                WHERE id = $1 AND tenant = $2 AND n.contacts_id = id",
            )
            .bind(id)
            .bind(tenant)),
            _ => Err(Error::unauthorized()),
        }
    }

    pub async fn retire(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE contacts
            SET retired_since = $1,
                updated_at = $1
            WHERE id = $2 AND retired_since IS NULL
            RETURNING *",
        )
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub async fn delete(
        id: i64,
        tenant: &str,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("DELETE FROM contacts WHERE id = $1 AND tenant = $2 RETURNING *")
            .bind(id)
            .bind(tenant)
            .fetch_optional(txn)
            .await
            .map_err(Error::from)
    }

    pub async fn link(
        &mut self,
        other: &Self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE contacts
                SET
                    relationship = $2,
                WHERE
                    id = $1 AND
                    co_owner = $3
                RETURNING *;",
        )
        .bind(self.id)
        .bind(other.id)
        .bind(&other.user_id);
        let customer: Self = query.fetch_one(txn).await?;
        *self = customer;
        Ok(())
    }
}
