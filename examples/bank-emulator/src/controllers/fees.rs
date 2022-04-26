use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Scope,
};

use crate::{
    auth::{AuthModel, User, Verb},
    context::Context,
    error::Error,
    models::{FeeMetadata, FeeResponse, FeeType, FeesAuth},
};

#[post("{currency}/{type}")]
async fn create_schedule(
    mut request: Json<FeeMetadata>,
    path: Path<(String, FeeType)>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<FeeMetadata>, Error> {
    let mut conn = context.db_pool.get().await?;
    FeesAuth.is_authorized(Verb::Create, &current_user)?;

    let (currency, fee_type) = path.into_inner();
    let _currency = context
        .config
        .currencies
        .get(currency.as_str())
        .ok_or_else(|| Error::not_found("currency"))?;
    request.validate()?;
    request
        .insert(&mut *conn, currency.as_str(), fee_type)
        .await?;
    Ok(request)
}

#[get("{id}/{type}")]
async fn get_schedule(
    path: Path<(String, FeeType)>,
    context: Data<Context>,
) -> Result<Json<FeeMetadata>, Error> {
    let mut conn = context.db_pool.get().await?;
    let (currency, fee_type) = path.into_inner();
    let fee_metadata = FeeMetadata::get(&mut *conn, fee_type, currency.as_str()).await?;
    Ok(Json(fee_metadata))
}

#[get("{currency}/{type}/{amount}")]
async fn get(
    path: Path<(String, FeeType, u64)>,
    context: Data<Context>,
) -> Result<Json<FeeResponse>, Error> {
    let mut conn = context.db_pool.get().await?;
    let (currency, fee_type, amount) = path.into_inner();
    let currency_config: &crate::config::CurrencyConfig = context
        .config
        .currencies
        .get(currency.as_str())
        .ok_or_else(|| Error::not_found("currency"))?;
    let fee_metadata = FeeMetadata::get(&mut *conn, fee_type, currency.as_str()).await?;
    let result = fee_metadata.calculate(amount, currency_config)?;
    Ok(Json(result))
}

pub fn scope() -> Scope {
    actix_web::web::scope("fees")
        .service(create_schedule)
        .service(get_schedule)
        .service(get)
}
