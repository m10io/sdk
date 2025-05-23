use actix_web::{
    get, put,
    web::{Data, Json, Path, Query},
    Scope,
};
use chrono::Utc;
use sqlx::Connection;

use crate::{
    auth::{BankEmulatorRole, User},
    context::Context,
    error::Error,
    models::{
        ListNotificationPreferencesFilter, ListResponse, NotificationPreferences,
        UpdateNotificationPreferencesRequest,
    },
};

#[get("")]
async fn list(
    filter: Query<ListNotificationPreferencesFilter>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i32, NotificationPreferences>>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = NotificationPreferences::find_scoped(filter.into_inner(), scope)?;
    let mut conn = context.db_pool.get().await?;
    let preferences = query.fetch_all(&mut *conn).await?;
    Ok(Json(ListResponse {
        data: preferences,
        next: None,
    }))
}

#[put("{id}")]
async fn update(
    id: Path<i32>,
    request: Json<UpdateNotificationPreferencesRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<NotificationPreferences>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = NotificationPreferences::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut preferences = query.fetch_one(&mut *txn).await?;

    if let Some(toggles) = &request.notification_toggles {
        preferences.notification_toggles = toggles.clone();
    }
    if let Some(token) = &request.device_token {
        preferences.device_token = token.to_string();
    }
    if request.device_token.is_some() || request.notification_toggles.is_some() {
        preferences.updated_at = Some(Utc::now());
        preferences.update(&mut *txn).await?;
    }

    txn.commit().await?;

    Ok(Json(preferences))
}

pub fn scope() -> Scope {
    actix_web::web::scope("notification_preferences")
        .service(list)
        .service(update)
}
