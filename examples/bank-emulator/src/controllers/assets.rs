use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    Scope,
};
use sqlx::Connection;

use crate::{
    auth::{AuthScope, BankEmulatorRole, User},
    context::Context,
    error::Error,
    models::{
        Asset, AssetType, AssetTypeQuery, Contact, CreateNotificationPreferencesRequest,
        ListNotificationPreferencesFilter, ListResponse, NotificationPreferences, Page, Payment,
        PaymentQuery,
    },
    utils,
};

#[get("")]
async fn list_assets(
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i32, Asset>>, Error> {
    let mut conn = context.db_pool.get().await?;
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = match scope {
        AuthScope::Own(_) => Asset::find_by_user_id(&current_user.user_id),
        _ => {
            return Err(Error::unauthorized());
        }
    };
    let assets = query.fetch_all(&mut *conn).await?;
    Ok(Json(ListResponse {
        data: assets,
        next: None,
    }))
}

#[get("{id}")]
async fn get_asset(
    instrument: Path<String>,
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let _scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let mut conn = context.db_pool.get().await?;
    let asset = Asset::find_by_user_id_instrument_type(
        &current_user.user_id,
        &*instrument,
        (&*asset_type).into(),
    )
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(Error::unauthorized)?;
    Ok(Json(asset))
}

#[get("{id}/bank_account")]
async fn get_bank_account(
    instrument: Path<String>,
    asset_type: Query<AssetTypeQuery>,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    // get bank issuance account
    let currency = context
        .config
        .currencies
        .get(&*instrument)
        .ok_or_else(|| Error::not_found("currency configuration"))?;
    let asset = match (&*asset_type).into() {
        AssetType::Regulated => Asset {
            ledger_account_id: currency.ledger_account_id(&context).await?.to_vec(),
            asset_type: AssetType::Regulated,
            ..Default::default()
        },
        AssetType::IndirectCbdc => Asset {
            ledger_account_id: currency
                .cbdc
                .as_ref()
                .ok_or_else(|| Error::not_found("indirect CBDC configuration"))?
                .ledger_account_id(currency.account_owner.as_ref(), &context)
                .await?
                .to_vec(),
            asset_type: AssetType::IndirectCbdc,
            ..Default::default()
        },
        _ => return Err(Error::internal_msg("unsupported asset type")),
    };

    Ok(Json(asset))
}

#[get("{id}/payments")]
async fn list_payments(
    instrument: Path<String>,
    asset_type: Query<AssetTypeQuery>,
    page: Query<Page<u64>>,
    include_child_accounts: Query<PaymentQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<u64, Payment>>, Error> {
    let mut conn = context.db_pool.get().await?;
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let ledger_account_id = match scope {
        AuthScope::Own(_) => {
            let asset = Asset::find_by_user_id_instrument_type(
                &current_user.user_id,
                &*instrument,
                (&*asset_type).into(),
            )
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;
            Some(asset.ledger_account_id)
        }
        AuthScope::Tenant(_) => None,
        _ => {
            return Err(Error::unauthorized());
        }
    };
    let Page { id, limit } = page.into_inner();
    let PaymentQuery {
        include_child_accounts,
    } = include_child_accounts.into_inner();
    let limit = limit.map(|l| if l > 50 { 50 } else { l }).unwrap_or(50);
    let payments = utils::list_payments(
        &*instrument,
        id.unwrap_or_default(),
        limit as u64,
        include_child_accounts,
        ledger_account_id,
        &context,
    )
    .await?;
    let next = payments.get((limit - 1) as usize).map(|p| p.into());
    Ok(Json(ListResponse {
        data: payments,
        next,
    }))
}

#[get("{id}/payments/{payment}")]
async fn get_payment(
    ids: Path<(String, u64)>,
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Payment>, Error> {
    current_user.is_authorized(BankEmulatorRole::Read)?;
    let (instrument, tx_id) = ids.into_inner();
    let mut conn = context.db_pool.get().await?;
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    match scope {
        AuthScope::Own(_) => {
            Asset::find_by_user_id_instrument_type(
                &current_user.user_id,
                &instrument,
                (&*asset_type).into(),
            )
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;
        }
        AuthScope::Tenant(_) => {}
        _ => {
            return Err(Error::unauthorized());
        }
    }
    let payment = utils::get_payment(&instrument, tx_id, &context).await?;
    Ok(Json(payment))
}

#[post("{id}/notification_preferences")]
async fn create_notification_preferences(
    instrument: Path<String>,
    asset_type: Query<AssetTypeQuery>,
    request: Json<CreateNotificationPreferencesRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<NotificationPreferences>, Error> {
    let _scope = current_user.is_authorized(BankEmulatorRole::Create)?;
    let mut conn = context.db_pool.get().await?;
    let asset = Asset::find_by_user_id_instrument_type(
        &current_user.user_id,
        &*instrument,
        (&*asset_type).into(),
    )
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(Error::unauthorized)?;
    let contact = Contact::find_by_user_id(&current_user.user_id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;

    let req = request.into_inner();
    if req.device_token.is_empty() {
        return Err(Error::validation(
            "device_token",
            "Cannot be empty".to_string(),
        ));
    }

    let mut preferences: NotificationPreferences = req.into();
    preferences.contacts_id = contact.id;
    preferences.asset_id = asset.id;
    let mut txn = conn.begin().await?;
    preferences.insert(&mut txn).await?;
    txn.commit().await?;

    Ok(Json(preferences))
}

#[get("{id}/notification_preferences")]
async fn list_notification_preferences(
    instrument: Path<String>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i32, NotificationPreferences>>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let filter = ListNotificationPreferencesFilter {
        instrument: Some(instrument.into_inner()),
        ..Default::default()
    };
    let query = NotificationPreferences::find_scoped(filter, scope)?;
    let mut conn = context.db_pool.get().await?;
    let preferences = query.fetch_all(&mut *conn).await?;
    Ok(Json(ListResponse {
        data: preferences,
        next: None,
    }))
}

pub fn scope() -> Scope {
    actix_web::web::scope("assets")
        .service(list_assets)
        .service(get_asset)
        .service(get_bank_account)
        .service(list_payments)
        .service(get_payment)
        .service(create_notification_preferences)
        .service(list_notification_preferences)
}
