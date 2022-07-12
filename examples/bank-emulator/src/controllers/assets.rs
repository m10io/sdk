use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    Scope,
};
use sqlx::Connection;

use crate::{
    auth::{AuthModel, AuthScope, User, Verb},
    context::Context,
    error::Error,
    models::{
        Asset, AssetAuth, Contact, CreateNotificationPreferencesRequest,
        ListNotificationPreferencesFilter, ListResponse, NotificationPreferences,
        NotificationPreferencesAuth, Page, Payment, PaymentQuery,
    },
    utils,
};

#[get("")]
async fn list_assets(
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i32, Asset>>, Error> {
    let mut conn = context.db_pool.get().await?;
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = match scope {
        AuthScope::Own(_) => Asset::find_by_user_id(&current_user.auth0_id),
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
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    AssetAuth.is_authorized(Verb::Read, &current_user)?;
    let mut conn = context.db_pool.get().await?;
    let asset = Asset::find_by_user_id_instrument(&current_user.auth0_id, &*instrument)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    Ok(Json(asset))
}

#[get("{id}/bank_account")]
async fn get_bank_account(
    instrument: Path<String>,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    // get bank issuance account
    let currency = context
        .config
        .currencies
        .get(&*instrument)
        .ok_or_else(|| Error::not_found("currency configuration"))?;
    let ledger_account_id = currency.ledger_account_id(&context).await?.to_vec();

    Ok(Json(Asset {
        ledger_account_id,
        ..Default::default()
    }))
}

#[get("{id}/payments")]
async fn list_payments(
    instrument: Path<String>,
    page: Query<Page<u64>>,
    include_child_accounts: Query<PaymentQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<u64, Payment>>, Error> {
    let mut conn = context.db_pool.get().await?;
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let ledger_account_id = match scope {
        AuthScope::Own(_) => {
            let asset = Asset::find_by_user_id_instrument(&current_user.auth0_id, &*instrument)
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
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Payment>, Error> {
    AssetAuth.is_authorized(Verb::Read, &current_user)?;
    let (instrument, tx_id) = ids.into_inner();
    let mut conn = context.db_pool.get().await?;
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    match scope {
        AuthScope::Own(_) => {
            Asset::find_by_user_id_instrument(&current_user.auth0_id, &instrument)
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
    request: Json<CreateNotificationPreferencesRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<NotificationPreferences>, Error> {
    NotificationPreferencesAuth.is_authorized(Verb::Create, &current_user)?;
    let mut conn = context.db_pool.get().await?;
    let asset = Asset::find_by_user_id_instrument(&current_user.auth0_id, &*instrument)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    let contact = Contact::find_by_user_id(&current_user.auth0_id)
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
    let scope = NotificationPreferencesAuth.auth_scope(Verb::Read, &current_user);
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
