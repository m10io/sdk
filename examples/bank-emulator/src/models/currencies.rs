use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

use crate::error::Error;

#[derive(sqlx::FromRow, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Currency {
    pub code: String,

    pub bank_id: Option<Vec<u8>>,

    pub regulated_account: Option<Vec<u8>>,

    pub cbdc_account: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Currency {
    pub async fn insert(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let code = self.code.to_lowercase();
        let query = sqlx::query_as(
            "INSERT INTO currencies (
                code,
                bank_id,
                regulated_account,
                cbdc_account
            ) VALUES ($1, $2, $3, $4)
            RETURNING *",
        )
        .bind(&code)
        .bind(&self.bank_id)
        .bind(&self.regulated_account)
        .bind(&self.cbdc_account);
        let currency: Self = query.fetch_one(txn).await?;
        *self = currency;
        Ok(())
    }

    pub async fn get(
        code: &str,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<Self, Error> {
        let code = code.to_lowercase();
        Ok(sqlx::query_as("SELECT * FROM currencies WHERE code = $1")
            .bind(&code)
            .fetch_one(txn)
            .await?)
    }

    pub async fn update(
        &mut self,
        txn: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), Error> {
        let code = self.code.to_lowercase();
        let query = sqlx::query_as(
            "UPDATE currencies
            SET bank_id = $1,
                regulated_account = $2,
                cbdc_account = $3,
                updated_at = $4
            WHERE code = $5
            RETURNING *",
        )
        .bind(&self.bank_id)
        .bind(&self.regulated_account)
        .bind(&self.cbdc_account)
        .bind(Utc::now())
        .bind(&code);
        let currency: Self = query.fetch_one(txn).await?;
        *self = currency;
        Ok(())
    }
}
