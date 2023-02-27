#![allow(dead_code)]
use chrono::TimeZone;
use m10_sdk::sdk;
use pala_types::TxId;
use sqlx::{postgres::PgArguments, query::QueryAs, Executor, Postgres};

use crate::error::Error;

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "transfer_handler")]
pub enum TransferHandler {
    CbdcLimits,
    CbdcReserves,
    DrcReserves,
}

impl Default for TransferHandler {
    fn default() -> Self {
        TransferHandler::CbdcLimits
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[sqlx(type_name = "ledger_transfers")]
pub(crate) struct LedgerTransfer {
    pub(crate) tx_id: Vec<u8>,
    pub(crate) target: Vec<u8>,
    #[sqlx(rename = "symbol")]
    pub(crate) currency_code: String,
    pub(crate) handler: TransferHandler,
    pub(crate) handled: bool,
    pub(crate) timestamp: chrono::DateTime<chrono::Utc>,
}

impl LedgerTransfer {
    pub(crate) fn new(
        account: &[u8],
        currency_code: &str,
        response: &sdk::TransactionResponse,
        handler: TransferHandler,
    ) -> Self {
        Self {
            tx_id: response.tx_id.to_be_bytes().to_vec(),
            target: account.to_vec(),
            currency_code: currency_code.into(),
            handler,
            handled: false,
            timestamp: chrono::Utc.timestamp_millis(response.timestamp as i64),
        }
    }

    pub(crate) async fn insert(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "INSERT INTO ledger_transfers (
                tx_id, target, symbol, handler, timestamp
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
        )
        .bind(&self.tx_id)
        .bind(&self.target)
        .bind(&self.currency_code)
        .bind(&self.handler)
        .bind(self.timestamp);
        let transfer: Self = query.fetch_one(txn).await?;
        *self = transfer;
        Ok(())
    }

    pub(crate) fn find_latest(
        handler: TransferHandler,
    ) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as(
            "SELECT * FROM ledger_transfers WHERE handler = $1 ORDER BY tx_id DESC LIMIT 1",
        )
        .bind(handler)
    }

    pub(crate) fn find_block(
        handler: TransferHandler,
        tx_id: TxId,
    ) -> QueryAs<'static, Postgres, Self, PgArguments> {
        let height = tx_id.height();
        sqlx::query_as(
            "SELECT * FROM ledger_transfers
            WHERE handler = $1 AND tx_id >= $2 AND tx_id < $3 
            ORDER BY tx_id DESC LIMIT 1",
        )
        .bind(handler)
        .bind(height.to_be_bytes().to_vec())
        .bind((height + 1).to_be_bytes().to_vec())
    }

    pub(crate) fn find_unhandled(
        handler: TransferHandler,
    ) -> QueryAs<'static, Postgres, Self, PgArguments> {
        sqlx::query_as("SELECT * FROM ledger_transfers WHERE handled = false AND handler = $1 ORDER BY tx_id ASC").bind(handler)
    }

    pub(crate) async fn set_handled(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let query = sqlx::query_as(
            "UPDATE ledger_transfers
            SET handled = true
            WHERE tx_id = $1
            RETURNING *",
        )
        .bind(&self.tx_id);
        let transfer: Self = query.fetch_one(txn).await?;
        *self = transfer;
        Ok(())
    }
}
