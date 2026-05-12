use actix_web::{
    get,
    web::{Data, Path},
    Scope,
};
use serde_json::json;

use crate::{
    auth::{BankEmulatorRole, User},
    context::Context,
    error::Error,
};

#[get("{currency}")]
async fn get(
    path: Path<String>,
    current_user: User,
    context: Data<Context>,
) -> Result<String, Error> {
    current_user.is_authorized(BankEmulatorRole::Read)?;

    let currency_raw = path.into_inner();
    let ccy_l = currency_raw.to_lowercase();
    let ccy_u = currency_raw.to_uppercase();

    let cla_name = format!("cla-{}", ccy_l);
    let tdl_reg_name = format!("tdl-{}-regulated", ccy_l);
    let tdl_cbdc_name = format!("tdl-{}-indirect_cbdc", ccy_l);

    let mut conn = context.db_pool.get().await?;

    let cla: i64 = sqlx::query_scalar(
        "SELECT balance
            FROM bank_accounts
        WHERE account_type = 'liability' AND display_name = $1",
    )
    .bind(&cla_name)
    .fetch_optional(&mut *conn)
    .await?
    .unwrap_or(0);

    let sum_traditional: i64 = sqlx::query_scalar(
        "SELECT cla_balance
            FROM v_cla_by_currency
        WHERE currency = $1",
    )
    .bind(&ccy_u)
    .fetch_optional(&mut *conn)
    .await?
    .unwrap_or(0);

    let tdl_regulated: i64 = sqlx::query_scalar(
        "SELECT balance
            FROM bank_accounts
        WHERE account_type = 'liability' AND display_name = $1",
    )
    .bind(&tdl_reg_name)
    .fetch_optional(&mut *conn)
    .await?
    .unwrap_or(0);

    let tdl_cbdc: i64 = sqlx::query_scalar(
        "SELECT balance
            FROM bank_accounts
        WHERE account_type = 'liability' AND display_name = $1",
    )
    .bind(&tdl_cbdc_name)
    .fetch_optional(&mut *conn)
    .await?
    .unwrap_or(0);

    let issuer_reg_id = context.get_currency_regulated_account(&ccy_l).await?;
    let issuer_cbdc_id = context.get_currency_cbdc_account(&ccy_l).await?;

    let issuer_reg = context
        .ledger
        .get_account(issuer_reg_id.as_slice().try_into().map_err(|_| {
            Error::internal_msg("invalid regulated issuer ledger_account_id length")
        })?)
        .await?;
    let issuer_cbdc = context
        .ledger
        .get_account(
            issuer_cbdc_id
                .as_slice()
                .try_into()
                .map_err(|_| Error::internal_msg("invalid cbdc issuer ledger_account_id length"))?,
        )
        .await?;

    let issuance_regulated: i64 =
        issuer_reg.issuance.as_ref().map(|i| i.balance).unwrap_or(0) as i64;
    let issuance_cbdc: i64 = issuer_cbdc
        .issuance
        .as_ref()
        .map(|i| i.balance)
        .unwrap_or(0) as i64;

    let diffs = json!({
        "cla_vs_sum_traditional": cla - sum_traditional,
        "tdl_regulated_vs_issuance": tdl_regulated - issuance_regulated,
        "tdl_cbdc_vs_issuance": tdl_cbdc - issuance_cbdc,
    });

    let body = json!({
        "currency": ccy_l,
        "cla": cla,
        "sum_traditional": sum_traditional,
        "tdl": {
            "regulated": tdl_regulated,
            "indirect_cbdc": tdl_cbdc
        },
        "issuance": {
            "regulated": issuance_regulated,
            "indirect_cbdc": issuance_cbdc
        },
        "diffs": diffs
    });

    Ok(body.to_string())
}

pub fn scope() -> Scope {
    actix_web::web::scope("reconciliation").service(get)
}
