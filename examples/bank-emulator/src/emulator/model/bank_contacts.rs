use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{postgres::PgArguments, query::QueryAs, Executor, Postgres};

use crate::{error::Error, models::ContactType};

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_contact_status")]
pub enum BankContactStatus {
    Pending,
    Approved,
    Denied,
    Frozen,
    Retired,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_contact_type")]
pub enum BankContactType {
    Individual,
    LegalEntity,
}

impl From<ContactType> for BankContactType {
    fn from(value: ContactType) -> Self {
        match value {
            ContactType::Individual => Self::Individual,
            ContactType::LegalEntity => Self::LegalEntity,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct BankContact {
    pub id: i64,
    pub account: i64,
    pub contact_type: BankContactType,
    pub contact_status: BankContactStatus,
    pub data: Option<Value>,
    pub payment_methods: Option<Value>,
    pub issues: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<BankContact> for Value {
    fn from(value: BankContact) -> Self {
        json!({
                "id": value.id,
                "account": value.account,
        })
    }
}

impl BankContact {
    pub async fn new(
        account: i64,
        contact_type: BankContactType,
        data: Value,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<BankContact, Error> {
        let query = sqlx::query_as(
            "INSERT INTO bank_contacts (
            account,
            contact_type,
            data
        ) VALUES ($1, $2, $3)
        RETURNING *",
        )
        .bind(account)
        .bind(contact_type)
        .bind(data);
        let contact: Self = query.fetch_one(txn).await?;
        Ok(contact)
    }

    pub fn try_id_from(value: &Value) -> Result<i64, Error> {
        value
            .get("id")
            .and_then(|i| i.as_i64())
            .ok_or_else(|| Error::not_found("contact id"))
    }

    pub fn find_by_id(id: i64) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM bank_contacts WHERE id = $1").bind(id)
    }

    pub fn find_by_account(account: i64) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM bank_contacts WHERE account = $1").bind(account)
    }

    pub async fn update_data(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE bank_contacts
            SET data = $1,
                updated_at = $2
            WHERE id = $3
            RETURNING *",
        )
        .bind(&self.data)
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub async fn update_status(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE bank_contacts
            SET contact_status = $1,
                updated_at = $2
            WHERE id = $3
            RETURNING *",
        )
        .bind(&self.contact_status)
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub async fn update_payment_methods(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE bank_contacts
            SET payment_methods = $1,
                updated_at = $2
            WHERE id = $3
            RETURNING *",
        )
        .bind(&self.payment_methods)
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }
}
