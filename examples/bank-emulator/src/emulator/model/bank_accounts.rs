use chrono::{DateTime, Utc};
use m10_rds_pool::{bb8::PooledConnection, RdsManager};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{postgres::PgArguments, query::QueryAs, Connection, Executor, Postgres};
use uuid::Uuid;

use crate::error::Error;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_account_status")]
pub enum BankAccountStatus {
    Pending,
    Open,
    PendingClosure,
    Closed,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_account_type")]
pub enum BankAccountType {
    Checking,
    Savings,
    Loan,
    Card,
    Holding,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct BankAccount {
    pub id: i64,
    pub account_status: BankAccountStatus,
    pub account_type: BankAccountType,
    pub account_number: i32,
    pub iban: Option<String>,
    pub balance: i64,
    pub currency: Option<String>,
    pub display_name: String,
    pub next_entry: i64,
    pub allowed_overdraft: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<BankAccount> for Value {
    fn from(value: BankAccount) -> Self {
        json!({
                "id": value.id,
                "account_number": value.account_number,
                "name": value.display_name,
                "balance": value.balance,
                "type": value.account_type,
        })
    }
}

impl BankAccount {
    pub async fn new(
        range: i32,
        display_name: &str,
        currency: &str,
        mut conn: PooledConnection<'_, RdsManager>,
    ) -> Result<BankAccount, Error> {
        let mut txn = conn.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(range)
            .execute(&mut txn)
            .await?;
        // TODO(mw): simplify
        let query = sqlx::query_scalar("SELECT new_account($1, $2, $3);")
            .bind(range)
            .bind(display_name)
            .bind(currency.to_uppercase());
        let id: i64 = query.fetch_one(&mut txn).await?;
        let query = sqlx::query_as("SELECT * FROM bank_accounts WHERE id = $1").bind(id);
        let account: Self = query.fetch_one(&mut txn).await?;
        txn.commit().await?;
        Ok(account)
    }

    pub fn try_id_from(value: &Value) -> Result<i64, Error> {
        value
            .get("id")
            .and_then(|i| i.as_i64())
            .ok_or_else(|| Error::not_found("account id"))
    }

    pub fn find_by_id(id: i64) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM bank_accounts WHERE id = $1").bind(id)
    }

    pub fn find_by_name(name: &str) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM bank_accounts WHERE display_name = $1").bind(name)
    }

    pub fn find_by_number(number: i64) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM bank_accounts WHERE account_number = $1").bind(number)
    }

    pub async fn update_status(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE bank_accounts
            SET account_status = $1,
                updated_at = $2
            WHERE id = $3
            RETURNING *",
        )
        .bind(&self.account_status)
        .bind(Utc::now())
        .bind(self.id);
        let contact: Self = query.fetch_one(txn).await?;
        *self = contact;
        Ok(())
    }

    pub async fn deposit_into(
        &mut self,
        account: i64,
        amount: i64,
        reference: &str,
        routing: Option<Value>,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Uuid, Error> {
        // Sandbox deposit w/o real routing
        // Money is issued from this account
        let query = sqlx::query_scalar("SELECT deposit($1, $2, $3, $4, $5, $6);")
            .bind(Uuid::new_v4())
            .bind(self.id)
            .bind(account)
            .bind(amount)
            .bind(reference)
            .bind(routing);
        let txn_id = query.fetch_one(txn).await?;
        Ok(txn_id)
    }

    pub async fn deposit_for_contact(
        &mut self,
        contact: i64,
        amount: i64,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Uuid, Error> {
        // Sandbox deposit w/o real routing
        // Money is issued from this account
        let query =
            sqlx::query_scalar("SELECT deposit_with_contact_method($1, $2, $3, $4, 'SBD');")
                .bind(Uuid::new_v4())
                .bind(self.id)
                .bind(contact)
                .bind(amount);
        let txn_id = query.fetch_one(txn).await?;
        Ok(txn_id)
    }

    pub async fn withdraw_from(
        &mut self,
        account: i64,
        amount: i64,
        reference: &str,
        routing: Option<Value>,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Uuid, Error> {
        // Sandbox withdraw w/o real routing
        // Money id deposited into this account
        let query = sqlx::query_scalar("SELECT withdraw($1, $2, $3, $4, $5, $6);")
            .bind(Uuid::new_v4())
            .bind(account)
            .bind(self.id)
            .bind(amount)
            .bind(reference)
            .bind(routing);
        let txn_id = query.fetch_one(txn).await?;
        Ok(txn_id)
    }

    pub async fn withdraw_for_contact(
        &mut self,
        contact: i64,
        amount: i64,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Uuid, Error> {
        // Sandbox withdraw w/o real routing
        // Money id deposited into this account
        let query = sqlx::query_as::<_, (Uuid,)>(
            "SELECT withdraw_with_contact_method($1, $2, $3, $4, 'SBW');",
        )
        .bind(Uuid::new_v4())
        .bind(contact)
        .bind(self.id)
        .bind(amount);
        let txn_id = query.fetch_one(txn).await?.0;
        Ok(txn_id)
    }

    pub async fn settle_deposit(
        &mut self,
        txn_id: Uuid,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<i64, Error> {
        let query = sqlx::query_scalar("SELECT settle_deposit($1);").bind(txn_id);
        let amount = query.fetch_one(txn).await?;
        Ok(amount)
    }

    pub async fn settle_withdraw(
        &mut self,
        txn_id: Uuid,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<i64, Error> {
        let query = sqlx::query_scalar("SELECT settle_withdraw($1);").bind(txn_id);
        let amount = query.fetch_one(txn).await?;
        Ok(amount)
    }

    pub async fn reverse_deposit(
        &mut self,
        txn_id: Uuid,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<i64, Error> {
        let query = sqlx::query_scalar("SELECT reverse_deposit($1);").bind(txn_id);
        let amount = query.fetch_one(txn).await?;
        Ok(amount)
    }

    pub async fn reverse_withdraw(
        &mut self,
        txn_id: Uuid,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<i64, Error> {
        let query = sqlx::query_scalar("SELECT reverse_withdraw($1);").bind(txn_id);
        let amount = query.fetch_one(txn).await?;
        Ok(amount)
    }

    pub async fn refresh(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as("SELECT * FROM bank_accounts WHERE id = $1").bind(self.id);
        let account: Self = query.fetch_one(txn).await?;
        *self = account;
        Ok(())
    }
}
