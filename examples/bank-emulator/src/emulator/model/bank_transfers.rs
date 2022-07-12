use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgArguments, query::QueryAs, Postgres};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_transaction_state")]
pub enum TransactionState {
    Pending,
    Settled,
    EnRoute,
    Canceled,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_transaction_type")]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct BankTransfer {
    pub txn_id: Uuid,
    pub refernce: String,
    pub amount: i64,
    pub routing: Option<Value>,
    pub account: i32,
    pub other_account: i32,
    pub transaction_type: TransactionType,
    pub transaction_status: TransactionState,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl BankTransfer {
    pub fn find_by_reference(reference: &str) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "
            SELECT  
                tf.txn_id, tf.reference, tf.amount, tf.routing,
                tx.account, tx.other_account, tx.transaction_type,
                tx.transaction_status, tf.created_at, tx.updated_at
            FROM bank_transfers tf, bank_transactions tx
            WHERE 
                tf.reference = $1 AND
                tx.transaction_type = 'debit'
            ",
        )
        .bind(reference)
    }
}
