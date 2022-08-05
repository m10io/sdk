use actix_multipart::Multipart;
use actix_web::{
    delete, get, patch, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse, Scope,
};
use m10_sdk::sdk::SetFreezeState;
use serde_json::Value;
use sqlx::Connection;

use crate::{
    auth::{AuthScope, BankEmulatorRole, User},
    bank::Bank,
    context::Context,
    error::Error,
    models::{
        Account, Asset, AssetTypeQuery, Contact, CreateContactRequest, ListResponse,
        NotificationPreferences, Page, Payment, PaymentQuery,
    },
    rbac,
    utils::{self, *},
};

#[post("")]
async fn create(
    request: Json<CreateContactRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Contact>, Error> {
    current_user.is_authorized(BankEmulatorRole::Create)?;
    let req = request.into_inner();
    let name = req
        .contact_data
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::not_found("name in contact data"))?;
    // Create AccountSet on ledger
    let account_set_id = create_account_set(vec![], &context).await?;

    // Create RBAC role for contact
    let role_id = rbac::create_contact_rbac_role(name, account_set_id, vec![], &context).await?;

    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut contact: Contact = req.into();
    contact.user_id = current_user.user_id;
    contact.rbac_role = role_id;
    contact.account_set = account_set_id;
    contact.insert(&mut txn).await?;
    txn.commit().await?;

    // Create Alias for contact
    create_alias_from_contact_data(
        &contact.contact_data,
        &current_user.token,
        account_set_id,
        &context,
    )
    .await?;

    Ok(Json(contact))
}

#[get("")]
async fn list(
    page: Query<Page<i64>>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i64, Contact>>, Error> {
    let mut conn = context.db_pool.get().await?;
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let limit = page.get_limit();
    let last_token = page.into_inner().try_into()?;
    let contacts = match scope {
        AuthScope::Own(id) => Contact::list_page_own(limit, last_token, &id, &mut *conn).await,
        AuthScope::Tenant(tenant) => {
            Contact::list_page_by_tenant(limit, last_token, &tenant, &mut *conn).await
        }
        _ => Err(Error::unauthorized()),
    }?;
    let next = contacts.get((limit - 1) as usize).map(|c| c.into());
    Ok(Json(ListResponse {
        data: contacts,
        next,
    }))
}

#[patch("")]
async fn update_own(
    request: Json<Value>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Contact>, Error> {
    current_user.is_authorized(BankEmulatorRole::Update)?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut contact = Contact::find_by_user_id(&current_user.user_id)
        .fetch_optional(&mut txn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    contact.contact_data = request.into_inner();
    contact.update(&mut txn).await?;
    txn.commit().await?;
    Ok(Json(contact))
}

#[delete("")]
async fn delete_own(current_user: User, context: Data<Context>) -> Result<Json<Contact>, Error> {
    current_user.is_authorized(BankEmulatorRole::Update)?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut contact = Contact::find_by_user_id(&current_user.user_id)
        .fetch_optional(&mut txn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    contact.retire(&mut txn).await?;
    txn.commit().await?;
    Ok(Json(contact))
}

#[get("{id}")]
async fn get(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Contact>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let contact = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    Ok(Json(contact))
}

#[patch("{id}")]
async fn update(
    id: Path<i64>,
    request: Json<Value>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Contact>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut contact = query
        .fetch_optional(&mut txn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    contact.contact_data = request.into_inner();
    contact.update(&mut txn).await?;
    txn.commit().await?;
    Ok(Json(contact))
}

#[delete("{id}")]
async fn delete(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Contact>, Error> {
    let tenant = current_user
        .is_authorized(BankEmulatorRole::Delete)?
        .authorized_tenant()?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let contact = Contact::delete(*id, &tenant, &mut txn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    txn.commit().await?;
    Ok(Json(contact))
}

#[get("{id}/accounts")]
async fn get_account(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_for_contact_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    Ok(Json(account))
}

#[get("{id}/assets")]
async fn list_assets(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i64, Asset>>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_contact_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let assets = query.fetch_all(&mut *conn).await?;
    Ok(Json(ListResponse {
        data: assets,
        next: None,
    }))
}

#[get("{id}/assets/{asset}")]
async fn get_asset(
    ids: Path<(i64, String)>,
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_contact_id_instrument_type_scoped(
        id,
        &instrument,
        (&*asset_type).into(),
        scope,
    )?;
    let mut conn = context.db_pool.get().await?;
    let asset = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    Ok(Json(asset))
}

#[post("{id}/assets/{asset}/freeze")]
async fn freeze_asset(
    ids: Path<(i64, String)>,
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_contact_id_instrument_type_scoped(
        id,
        &instrument,
        (&*asset_type).into(),
        scope,
    )?;
    let mut conn = context.db_pool.get().await?;
    let asset = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    let freeze_state_request = SetFreezeState {
        account_id: asset.ledger_account_id.clone(),
        frozen: true,
    };
    submit_transaction(freeze_state_request, vec![], &context).await?;
    Ok(Json(asset))
}

#[get("{id}/assets/{asset}/payments")]
async fn list_payments(
    ids: Path<(i64, String)>,
    asset_type: Query<AssetTypeQuery>,
    page: Query<Page<u64>>,
    include_child_accounts: Query<PaymentQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<u64, Payment>>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_contact_id_instrument_type_scoped(
        id,
        &instrument,
        (&*asset_type).into(),
        scope,
    )?;
    let mut conn = context.db_pool.get().await?;
    let asset = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
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
        Some(asset.ledger_account_id),
        &context,
    )
    .await?;
    let next = payments.get((limit - 1) as usize).map(|p| p.into());
    Ok(Json(ListResponse {
        data: payments,
        next,
    }))
}

#[get("{id}/assets/{asset}/payments/{payment}")]
async fn get_payment(
    ids: Path<(i64, String, u64)>,
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Payment>, Error> {
    let (id, instrument, tx_id) = ids.into_inner();
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_contact_id_instrument_type_scoped(
        id,
        &instrument,
        (&*asset_type).into(),
        scope,
    )?;
    let mut conn = context.db_pool.get().await?;
    query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    let payment = utils::get_payment(&instrument, tx_id, &context).await?;
    Ok(Json(payment))
}

#[post("{id}/assets/{asset}/unfreeze")]
async fn unfreeze_asset(
    ids: Path<(i64, String)>,
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_contact_id_instrument_type_scoped(
        id,
        &instrument,
        (&*asset_type).into(),
        scope,
    )?;
    let mut conn = context.db_pool.get().await?;
    let asset = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(Error::unauthorized)?;
    let freeze_state_request = SetFreezeState {
        account_id: asset.ledger_account_id.clone(),
        frozen: false,
    };
    submit_transaction(freeze_state_request, vec![], &context).await?;
    Ok(Json(asset))
}

#[get("{id}/notification_preferences")]
async fn list_notification_preferences(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i32, NotificationPreferences>>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Contact::get_notification_preferences_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let preferences = query.fetch_all(&mut *conn).await?;
    Ok(Json(ListResponse {
        data: preferences,
        next: None,
    }))
}

#[get("{id}/documents")]
async fn list_documents(
    _id: Path<i64>,
    _page: Query<Page<String>>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<ListResponse<String, Value>>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("{id}/documents")]
async fn upload_documents(
    _id: Path<i64>,
    _request: Multipart,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[get("{id}/documents/{doc}")]
async fn get_documents(
    _ids: Path<(i64, String)>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<serde_json::Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[patch("{id}/documents/{doc}")]
async fn update_documents(
    _ids: Path<(i64, String)>,
    _request: Json<Value>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<serde_json::Value>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("{id}/documents/{doc}/sandbox/verify")]
async fn verify_documents(
    _ids: Path<(i32, String)>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<HttpResponse, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[put("{id}/key")]
async fn add_key(
    id: Path<i64>,
    request: String,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let contact = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;

    let key = base64::decode(request)?;

    if !rbac::is_key_known(contact.rbac_role, &key, &context).await? {
        rbac::add_key(contact.rbac_role, &key, &context).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/notification")]
async fn notification(
    _id: Path<i64>,
    _request: Json<Value>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<HttpResponse, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("{id}/sandbox/approve")]
async fn approve_cip(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let contact = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    if let Some(contact_ref) = contact.bank_reference.as_ref() {
        context.bank.approve_contact(contact_ref).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/sandbox/deny")]
async fn deny_cip(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let contact = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    if let Some(contact_ref) = contact.bank_reference.as_ref() {
        context.bank.deny_contact(contact_ref).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/sandbox/freeze")]
async fn freeze(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let contact = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    if let Some(contact_ref) = contact.bank_reference.as_ref() {
        context.bank.freeze_contact(contact_ref).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/sandbox/trigger_kyb")]
async fn trigger_kyb(
    _id: Path<i64>,
    _request: Json<Value>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<HttpResponse, Error> {
    Err(Error::internal_msg("unimplemented"))
}

#[post("{id}/sandbox/unfreeze")]
async fn unfreeze(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Update)?;
    let query = Contact::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let contact = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("contacts"))?;
    if let Some(contact_ref) = contact.bank_reference.as_ref() {
        context.bank.unfreeze_contact(contact_ref).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("/relationship/{other}")]
async fn connect_with(
    _other: Path<i32>,
    _current_user: User,
    _context: Data<Context>,
) -> Result<Json<Contact>, Error> {
    Err(Error::internal_msg("unimplemented"))
}

pub fn scope() -> Scope {
    actix_web::web::scope("contacts")
        .service(create)
        .service(list)
        .service(update_own)
        .service(delete_own)
        .service(get)
        .service(update)
        .service(delete)
        .service(get_account)
        .service(list_assets)
        .service(get_asset)
        .service(freeze_asset)
        .service(list_payments)
        .service(get_payment)
        .service(unfreeze_asset)
        .service(list_notification_preferences)
        .service(list_documents)
        .service(upload_documents)
        .service(get_documents)
        .service(update_documents)
        .service(verify_documents)
        .service(add_key)
        .service(notification)
        .service(approve_cip)
        .service(deny_cip)
        .service(freeze)
        .service(trigger_kyb)
        .service(unfreeze)
        .service(connect_with)
}
