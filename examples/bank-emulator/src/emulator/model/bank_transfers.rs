use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgArguments, query::QueryAs, Postgres};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_transaction_status")]
pub enum TransactionStatus {
    Pending,
    Settled,
    EnRoute,
    Canceled,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "bank_transaction_type")]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct BankTransfer {
    pub txn_id: Uuid,
    pub reference: String,
    pub amount: i64,
    pub routing: Option<Value>,
    pub account: i64,
    pub other_account: i64,
    pub transaction_type: TransactionType,
    pub transaction_status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl BankTransfer {
    pub fn find_by_id_and_type(
        txn_id: Uuid,
        tx_type: TransactionType,
    ) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "
            SELECT  
                tf.txn_id, tf.reference, tf.amount, tf.routing,
                tx.account, tx.other_account, tx.transaction_type,
                tx.transaction_status, tf.created_at, tx.updated_at
            FROM bank_transfers tf
            JOIN bank_transactions tx ON tx.txn_id = tf.txn_id
            WHERE 
                tf.txn_id = $1 AND
                tx.transaction_type = $2
            ",
        )
        .bind(txn_id)
        .bind(tx_type)
    }

    pub fn find_by_reference(reference: &str) -> QueryAs<'_, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "
            SELECT  
                tf.txn_id, tf.reference, tf.amount, tf.routing,
                tx.account, tx.other_account, tx.transaction_type,
                tx.transaction_status, tf.created_at, tx.updated_at
            FROM bank_transfers tf
            JOIN bank_transactions tx ON tx.txn_id = tf.txn_id
            WHERE 
                tf.reference = $1 AND
                tx.transaction_type = 'debit'
            ",
        )
        .bind(reference)
    }
}
