use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Scope,
};
use m10_protos::sdk::{CreateTransfer, Deposit, TransferStep, Withdraw};
use m10_sdk::{sdk::SetFreezeState, LedgerClient, Metadata, Signer};
use serde_json::{json, Value};
use sqlx::Acquire;
use tracing::info;
use uuid::Uuid;

use crate::{
    auth::{AuthModel, AuthScope, User, Verb},
    bank::Bank,
    context::Context,
    error::Error,
    models::{
        Account, AccountAuth, AccountStatus, AmountRequest, Asset, AssetAuth, Contact,
        CreateAccountRequest, ListResponse, NotificationPreferences, NotificationPreferencesAuth,
        Page, Payment, PaymentQuery,
    },
    rbac::create_contact_rbac_role,
    utils::{self, *},
};

#[post("")]
async fn create(
    request: Json<CreateAccountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    AccountAuth.is_authorized(Verb::Create, &current_user)?;
    let req = request.into_inner();
    let name = req
        .contact
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::not_found("name in contact data"))?;
    let profile_image_url = req
        .contact
        .get("profile_image_url")
        .and_then(|v| v.as_str());

    let account_ref = context.bank.create_account(name).await?;
    let contact_ref = context
        .bank
        .create_contact(
            &account_ref,
            req.contact_type.clone().unwrap_or_default(),
            req.contact.clone(),
        )
        .await?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;

    // Create Account entry
    let mut account = Account {
        tenant: req.tenant.clone(),
        bank_reference: Some(account_ref),
        ..Default::default()
    };
    account.insert(&mut txn).await?;

    // Create Assets on ledger
    let assets = req
        .assets
        .unwrap_or_else(|| context.config.currencies.keys().cloned().collect());
    let mut ledger_accounts = vec![];

    for a in assets {
        if let Some(config) = context.config.currencies.get(&a) {
            if !config.test {
                let mut asset =
                    Asset::new_for_currency(config, &context, name, profile_image_url).await?;
                asset.instrument = a;
                asset.linked_account = account.id;
                asset.tenant = req.tenant.clone();
                asset.insert(&mut txn).await?;
                ledger_accounts.push(asset.ledger_account_id);
            }
        }
    }

    // Create AccountSet on ledger
    let account_set_id = create_account_set(ledger_accounts.to_vec(), &context).await?;

    // Create RBAC role for contact
    let role_id = create_contact_rbac_role(name, account_set_id, ledger_accounts, &context).await?;

    // Create Contact entry
    let mut contact = Contact {
        user_id: current_user.auth0_id,
        account_id: Some(account.id),
        bank_reference: Some(contact_ref),
        contact_data: req.contact,
        contact_type: req.contact_type.unwrap_or_default(),
        rbac_role: role_id,
        account_set: account_set_id,
        tenant: req.tenant,
        ..Default::default()
    };
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

    Ok(Json(account))
}

#[get("")]
async fn list(
    current_user: User,
    page: Query<Page<i64>>,
    context: Data<Context>,
) -> Result<Json<ListResponse<i64, Account>>, Error> {
    let mut conn = context.db_pool.get().await?;
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let limit = page.get_limit();
    let last_token = page.into_inner().try_into()?;
    let accounts = match scope {
        AuthScope::Own(id) => Account::list_page_own(limit, last_token, &id, &mut *conn).await,
        AuthScope::Tenant(tenant) => {
            Account::list_page_by_tenant(limit, last_token, &tenant, &mut *conn).await
        }
        _ => Err(Error::unauthorized()),
    }?;
    let next = accounts.get((limit - 1) as usize).map(|a| a.into());
    Ok(Json(ListResponse {
        data: accounts,
        next,
    }))
}

#[delete("{id}")]
async fn delete(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    let tenant = AccountAuth
        .auth_scope(Verb::Delete, &current_user)
        .authorized_tenant()?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let account = Account::delete(*id, &tenant, &mut txn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    txn.commit().await?;
    Ok(Json(account))
}

#[get("{id}")]
async fn get(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::find_by_id_scoped(*id, scope)?;
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
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = Asset::find_by_account_id_scoped(*id, scope)?;
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
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = Asset::find_by_account_id_instrument_scoped(id, &instrument, scope)?;
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
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = Asset::find_by_account_id_instrument_scoped(id, &instrument, scope)?;
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
    page: Query<Page<u64>>,
    include_child_accounts: Query<PaymentQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<u64, Payment>>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = Asset::find_by_account_id_instrument_scoped(id, &instrument, scope)?;
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
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Payment>, Error> {
    let (id, instrument, tx_id) = ids.into_inner();
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = Asset::find_by_account_id_instrument_scoped(id, &instrument, scope)?;
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
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
    let query = Asset::find_by_account_id_instrument_scoped(id, &instrument, scope)?;
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
    let scope = NotificationPreferencesAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::get_notification_preferences_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let preferences = query.fetch_all(&mut *conn).await?;
    Ok(Json(ListResponse {
        data: preferences,
        next: None,
    }))
}

#[post("{id}/deposit")]
async fn deposit(
    id: Path<i64>,
    request: Json<AmountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Value>, Error> {
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    let bank_txn = if let Some(account_ref) = account.bank_reference.as_ref() {
        let mut bank = context.bank.clone();
        let txn_id = bank
            .account_deposit(request.amount_in_cents, account_ref)
            .await?;
        Some(txn_id)
    } else {
        None
    };
    Ok(Json(json!({ "bank_tx": bank_txn })))
}

#[post("{id}/sandbox/fund")]
async fn fund(
    id: Path<i64>,
    request: Json<AmountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    if let Some(account_ref) = account.bank_reference.as_ref() {
        let mut bank = context.bank.clone();
        bank.fund_account(request.amount_in_cents, account_ref)
            .await?;
    }
    if let Some(instrument) = request.currency.as_ref() {
        let instrument = instrument.to_lowercase();
        let currency = context
            .config
            .currencies
            .get(&instrument)
            .ok_or_else(|| Error::not_found("currency configuration"))?;
        let ledger_account_id = currency.ledger_account_id(&context).await?;
        let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
        let query = Asset::find_by_account_id_instrument_scoped(*id, &instrument, scope)?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;

        // create transaction
        let deposit_metadata = Deposit::default().any();
        let payload = LedgerClient::transaction_request(
            m10_sdk::ledger::transaction_data::Data::Transfer(CreateTransfer {
                transfer_steps: vec![TransferStep {
                    from_account_id: ledger_account_id.to_vec(),
                    to_account_id: asset.ledger_account_id.to_vec(),
                    amount: request.amount_in_cents,
                    metadata: vec![deposit_metadata],
                }],
            }),
            vec![],
        );
        let signed_request = context.signer.sign_request(payload).await?;
        let mut ledger = context.ledger.clone();
        ledger
            .create_transaction(signed_request)
            .await?
            .tx_error()?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/sandbox/open")]
async fn open(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    let scope = AccountAuth.auth_scope(Verb::Update, &current_user);
    let query = Account::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut account = query
        .fetch_optional(&mut txn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    if let Some(account_ref) = account.bank_reference.as_ref() {
        context.bank.open_account(account_ref).await?;
    }
    account.status = AccountStatus::Open;
    account.update_status(&mut txn).await?;
    txn.commit().await?;
    Ok(Json(account))
}

#[post("{id}/sandbox/settle_deposit/{deposit_id}")]
async fn settle_deposit(
    ids: Path<(i64, Uuid)>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let (id, txn_id) = ids.into_inner();
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::find_by_id_scoped(id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    let mut bank = context.bank.clone();
    let amount = bank.settle_deposit(txn_id).await?;

    if let Some(instrument) = account
        .bank_reference
        .as_ref()
        .and_then(|v| v.get("currency").and_then(|c| c.as_str()))
    {
        let instrument = instrument.to_lowercase();
        let currency = context
            .config
            .currencies
            .get(&instrument)
            .ok_or_else(|| Error::not_found("currency configuration"))?;
        let ledger_account_id = currency.ledger_account_id(&context).await?;

        let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
        let query = Asset::find_by_account_id_instrument_scoped(id, &instrument, scope)?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;

        // create transaction
        let deposit_metadata = Deposit::default().any();
        let payload = LedgerClient::transaction_request(
            m10_sdk::ledger::transaction_data::Data::Transfer(CreateTransfer {
                transfer_steps: vec![TransferStep {
                    from_account_id: ledger_account_id.to_vec(),
                    to_account_id: asset.ledger_account_id.to_vec(),
                    amount,
                    metadata: vec![deposit_metadata],
                }],
            }),
            vec![],
        );
        let signed_request = context.signer.sign_request(payload).await?;
        let mut ledger = context.ledger.clone();
        let txn = ledger
            .create_transaction(signed_request)
            .await?
            .tx_error()?;
        info!("Deposit mirrored on ledger: {}", txn.tx_id);
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/sandbox/settle_withdraw/{withdraw_id}")]
async fn settle_withdraw(
    ids: Path<(i64, Uuid)>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let (id, txn_id) = ids.into_inner();
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::find_by_id_scoped(id, scope)?;
    let mut conn = context.db_pool.get().await?;
    query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    let mut bank = context.bank.clone();
    bank.settle_withdraw(txn_id).await?;
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/withdraw")]
async fn withdraw(
    id: Path<i64>,
    request: Json<AmountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Value>, Error> {
    let scope = AccountAuth.auth_scope(Verb::Read, &current_user);
    let query = Account::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    let bank_txn = if let Some(account_ref) = account.bank_reference.as_ref() {
        let mut bank = context.bank.clone();
        let txn_id = bank
            .account_withdraw(request.amount_in_cents, account_ref)
            .await?;
        Some(txn_id)
    } else {
        None
    };
    let ledger_txn = if let Some(instrument) = request.currency.as_ref() {
        let instrument = instrument.to_lowercase();
        let currency = context
            .config
            .currencies
            .get(&instrument)
            .ok_or_else(|| Error::not_found("currency configuration"))?;
        let ledger_account_id = currency.ledger_account_id(&context).await?;
        let scope = AssetAuth.auth_scope(Verb::Read, &current_user);
        let query = Asset::find_by_account_id_instrument_scoped(*id, &instrument, scope)?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;

        // create transaction
        let withdraw_metadata = Withdraw::default().any();
        let payload = LedgerClient::transaction_request(
            m10_sdk::ledger::transaction_data::Data::Transfer(CreateTransfer {
                transfer_steps: vec![TransferStep {
                    from_account_id: asset.ledger_account_id.to_vec(),
                    to_account_id: ledger_account_id.to_vec(),
                    amount: request.amount_in_cents,
                    metadata: vec![withdraw_metadata],
                }],
            }),
            vec![],
        );
        let signed_request = context.signer.sign_request(payload).await?;
        let mut ledger = context.ledger.clone();
        let txn = ledger
            .create_transaction(signed_request)
            .await?
            .tx_error()?;
        Some(txn.tx_id)
    } else {
        None
    };
    Ok(Json(
        json!({ "bank_tx": bank_txn, "ledger_tx": ledger_txn }),
    ))
}

pub fn scope() -> Scope {
    actix_web::web::scope("accounts")
        .service(create)
        .service(list)
        .service(delete)
        .service(get)
        .service(list_assets)
        .service(get_asset)
        .service(freeze_asset)
        .service(list_payments)
        .service(get_payment)
        .service(unfreeze_asset)
        .service(list_notification_preferences)
        .service(deposit)
        .service(fund)
        .service(open)
        .service(settle_deposit)
        .service(settle_withdraw)
        .service(withdraw)
}
