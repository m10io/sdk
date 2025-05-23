#![allow(clippy::unnecessary_fallible_conversions)]
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Scope,
};
use m10_sdk::{
    sdk::{Deposit, SelfTransfer, SetFreezeState, Withdraw},
    Metadata, StepBuilder, TransferBuilder, TransferStatus, TransferStep,
};
use serde_json::{json, Value};
use sqlx::Acquire;
use tracing::info;
use uuid::Uuid;

use crate::{
    auth::{AuthScope, BankEmulatorRole, User},
    bank::Bank,
    context::Context,
    controllers::contacts::validate_signatures,
    error::Error,
    models::{
        Account, AccountStatus, AmountRequest, Asset, AssetType, AssetTypeQuery, BankTransfer,
        Contact, CreateAccountRequest, LedgerAccountQuery, ListResponse, NotificationPreferences,
        Page, Payment, PaymentQuery, RedeemRequest,
    },
    rbac::create_contact_rbac_role,
    utils::{self, *},
};

#[post("")]
#[allow(clippy::assigning_clones)]
async fn create(
    request: Json<CreateAccountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    current_user.is_authorized(BankEmulatorRole::Create)?;
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

    validate_signatures(req.signatures.clone())?;

    let mut bank = context.bank.clone();
    let account_ref = bank.create_account(name).await?;
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
    account.insert(&mut *txn).await?;

    // Create Assets on ledger
    let assets = req
        .assets
        .unwrap_or_else(|| context.config.currencies.keys().cloned().collect());
    let mut ledger_accounts = vec![];

    for a in assets {
        if let Some(config) = context.config.currencies.get(&a) {
            if config.test.is_none() {
                if config.cbdc_config.is_some() {
                    let mut cbdc_asset = Asset::new_cbdc_for_currency(
                        &config.code,
                        &context,
                        name,
                        profile_image_url,
                    )
                    .await?;
                    cbdc_asset.asset_type = AssetType::IndirectCbdc;
                    cbdc_asset.instrument = a.clone();
                    cbdc_asset.linked_account = account.id;
                    cbdc_asset.tenant = req.tenant.clone();
                    cbdc_asset.insert(&mut *txn).await?;
                    ledger_accounts.push(cbdc_asset.ledger_account_id);
                }
                let mut asset =
                    Asset::new_for_currency(&config.code, &context, name, profile_image_url)
                        .await?;
                asset.asset_type = AssetType::Regulated;
                asset.instrument = a;
                asset.linked_account = account.id;
                asset.tenant = req.tenant.clone();
                asset.insert(&mut *txn).await?;
                ledger_accounts.push(asset.ledger_account_id);
            }
        }
    }

    // Create AccountSet on ledger
    let account_set_id = create_account_set(ledger_accounts.to_vec(), &context).await?;

    // Create RBAC role for contact
    let role_id = create_contact_rbac_role(
        name,
        account_set_id,
        ledger_accounts,
        req.signatures
            .iter()
            .map(|s| s.public_key.clone())
            .collect(),
        &context,
    )
    .await?;

    // Create Contact entry
    let mut contact = Contact {
        user_id: current_user.user_id,
        account_id: Some(account.id),
        bank_reference: Some(contact_ref),
        contact_data: req.contact,
        contact_type: req.contact_type.unwrap_or_default(),
        rbac_role: role_id,
        account_set: account_set_id,
        tenant: req.tenant,
        ..Default::default()
    };
    contact.insert(&mut *txn).await?;
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
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
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
    let scope = current_user.is_authorized(BankEmulatorRole::Delete)?;
    let tenant = scope.authorized_tenant()?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let account = Account::delete(*id, &tenant, &mut *txn)
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
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let mut account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    if let Some(account_ref) = &account.bank_reference {
        let bank_account = context.bank.get_account(account_ref).await?;
        account.bank_reference = Some(bank_account.into());
    }
    Ok(Json(account))
}

#[get("{id}/assets")]
async fn list_assets(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<ListResponse<i64, Asset>>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
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
    asset_type: Query<AssetTypeQuery>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Asset>, Error> {
    let (id, instrument) = ids.into_inner();
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Asset::find_by_account_id_instrument_type_scoped(
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
    let query = Asset::find_by_account_id_instrument_type_scoped(
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
    let asset_type = AssetType::from(&asset_type.into_inner());
    let query = Asset::find_by_account_id_instrument_type_scoped(
        id,
        &instrument,
        asset_type.clone(),
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
        &instrument,
        id.unwrap_or_default(),
        limit as u64,
        include_child_accounts,
        Some(asset.ledger_account_id),
        asset_type,
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
    let query = Asset::find_by_account_id_instrument_type_scoped(
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
    let query = Asset::find_by_account_id_instrument_type_scoped(
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
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let request = request.into_inner();
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    let bank_txn = if let Some(account_ref) = account.bank_reference.as_ref() {
        let mut bank = context.bank.clone();
        let reference = format!(
            "customer deposit for:{}",
            serde_json::to_string(&request.asset_type.unwrap_or_default())?
        );
        let txn_id = bank
            .account_deposit(request.amount_in_cents, account_ref, &reference)
            .await?;
        Some(txn_id)
    } else {
        None
    };
    Ok(Json(json!({ "bank_tx": bank_txn })))
}

#[post("{id}/request_funds")]
async fn convert_into(
    id: Path<i64>,
    ledger_account_id: Query<LedgerAccountQuery>,
    request: Json<AmountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<BankTransfer>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let request = request.into_inner();
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;

    let instrument = context.bank.currency().to_lowercase();

    // get bank issuance account
    let asset_type = request.asset_type.unwrap_or_default();
    let from_account = match asset_type {
        AssetType::Regulated => context.get_currency_regulated_account(&instrument).await?,
        AssetType::IndirectCbdc => context.get_currency_cbdc_account(&instrument).await?,
        _ => return Err(Error::internal_msg("unsupported asset type")),
    };

    // take customer ledger account from query param or asset
    let to_account = if let Some(id) = &ledger_account_id.ledger_account_id {
        hex::decode(id)?
    } else {
        let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
        let query = Asset::find_by_account_id_instrument_type_scoped(
            *id,
            &instrument,
            asset_type.clone(),
            scope,
        )?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;
        asset.ledger_account_id
    };

    // Check if accounts are directly related (issuing account needs to be
    // parent of customer account)
    check_if_parent(&to_account, &from_account)?;

    // Withdraw money from linked bank account
    let bank_account = account
        .bank_reference
        .ok_or_else(|| Error::not_found("bank account"))?;
    let bank_account_number = crate::bank::try_account_number_from(&bank_account)?;
    let mut bank = context.bank.clone();
    let bank_txn = bank
        .account_withdraw(request.amount_in_cents, &bank_account)
        .await?;

    let self_meta = SelfTransfer {
        from_account_name: "Checking".to_string(),
        to_account_name: asset_type.to_string(),
    };

    let transfer = match ledger_transfer(
        from_account,
        to_account,
        request.amount_in_cents,
        vec![Deposit::default().any(), self_meta.any()],
        &context,
    )
    .await
    {
        Ok(_) => {
            let amount = bank.settle_withdraw(bank_txn).await?;
            BankTransfer {
                id: bank_txn,
                from_account: bank_account_number,
                to_account: bank.account_number(),
                amount: Some(amount),
                status: 0,
                error: None,
            }
        }
        Err(err) => {
            bank.reverse_withdraw(bank_txn).await?;
            BankTransfer {
                id: bank_txn,
                from_account: bank_account_number,
                to_account: bank.account_number(),
                amount: None,
                status: 1,
                error: Some(err.to_string()),
            }
        }
    };
    Ok(Json(transfer))
}

#[post("{id}/redeem_direct")]
async fn convert_direct_from(
    id: Path<i64>,
    ledger_account_id: Query<LedgerAccountQuery>,
    request: Json<AmountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<BankTransfer>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let request = request.into_inner();
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;

    let instrument = context.bank.currency().to_lowercase();

    // get bank issuance account
    let asset_type = request.asset_type.unwrap_or_default();
    let to_account = match asset_type {
        AssetType::Regulated => context.get_currency_regulated_account(&instrument).await?,
        AssetType::IndirectCbdc => context.get_currency_cbdc_account(&instrument).await?,
        _ => return Err(Error::internal_msg("unsupported asset type")),
    };

    // take customer ledger account from query param or asset
    let from_account = if let Some(id) = &ledger_account_id.ledger_account_id {
        hex::decode(id)?
    } else {
        let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
        let query = Asset::find_by_account_id_instrument_type_scoped(
            *id,
            &instrument,
            asset_type.clone(),
            scope,
        )?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;
        asset.ledger_account_id
    };

    // Check if accounts are directly related (issuing account needs to be
    // parent of customer account)
    check_if_parent(&from_account, &to_account)?;

    // Deposit money from bank reserve account
    let bank_account = account
        .bank_reference
        .ok_or_else(|| Error::not_found("bank account"))?;
    let bank_account_number = crate::bank::try_account_number_from(&bank_account)?;

    let self_meta = SelfTransfer {
        from_account_name: asset_type.to_string(),
        to_account_name: "Checking".to_string(),
    };
    let ledger_txn = ledger_transfer(
        from_account,
        to_account,
        request.amount_in_cents,
        vec![Withdraw::default().any(), self_meta.any()],
        &context,
    )
    .await?;
    let mut bank = context.bank.clone();
    let bank_txn = bank
        .account_deposit(
            request.amount_in_cents,
            &bank_account,
            &format!("ledger: {}", ledger_txn),
        )
        .await?;

    let amount = bank.settle_deposit(bank_txn).await?;
    let transfer = BankTransfer {
        id: bank_txn,
        from_account: bank.account_number(),
        to_account: bank_account_number,
        amount: Some(amount),
        status: 0,
        error: None,
    };
    Ok(Json(transfer))
}

#[post("{id}/redeem")]
async fn convert_from(
    id: Path<i64>,
    request: Json<RedeemRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<BankTransfer>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let request = request.into_inner();
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;

    let instrument = context.bank.currency().to_lowercase();

    // get bank issuance account
    let asset_type = request.asset_type.unwrap_or_default();
    let to_account_id = match asset_type {
        AssetType::Regulated => context.get_currency_regulated_account(&instrument).await?,
        AssetType::IndirectCbdc => context.get_currency_cbdc_account(&instrument).await?,
        _ => return Err(Error::internal_msg("unsupported asset type")),
    };

    // get transaction from ledger
    let transfer = context.ledger.get_transfer(request.txn_id).await?;

    if transfer.status != TransferStatus::Accepted {
        return Err(Error::internal_msg("Transfer not accepted (state)"));
    }

    // find transfer step that has bank reserve account as target
    let to_account_id = to_account_id.as_slice().try_into()?;
    let TransferStep { amount, .. } = transfer
        .steps
        .into_iter()
        .find(|s| s.to == to_account_id)
        .ok_or_else(|| Error::internal_msg("Transfer not accepted (target account"))?;

    let mut bank = context.bank.clone();
    let reference = format!("ledger: {}", request.txn_id);

    // Check that ledger transfer hasn't been used yet
    let transfers = bank.transfers_by_reference(&reference).await?;
    if !transfers.is_empty() {
        return Err(Error::internal_msg("transaction already used"));
    }

    // Deposit money from bank reserve account
    let bank_account = account
        .bank_reference
        .ok_or_else(|| Error::not_found("bank account"))?;
    let bank_account_number = crate::bank::try_account_number_from(&bank_account)?;

    let bank_txn = bank
        .account_deposit(amount, &bank_account, &reference)
        .await?;

    let amount = bank.settle_deposit(bank_txn).await?;
    let transfer = BankTransfer {
        id: bank_txn,
        from_account: bank.account_number(),
        to_account: bank_account_number,
        amount: Some(amount),
        status: 0,
        error: None,
    };

    Ok(Json(transfer))
}

#[post("{id}/sandbox/fund")]
async fn fund(
    id: Path<i64>,
    request: Json<AmountRequest>,
    current_user: User,
    context: Data<Context>,
) -> Result<HttpResponse, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let request = request.into_inner();
    let mut conn = context.db_pool.get().await?;
    let account = query
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    if let Some(account_ref) = account.bank_reference.as_ref() {
        let mut bank = context.bank.clone();
        bank.fund_account(request.amount_in_cents, account_ref, "sandbox fund")
            .await?;
    }
    if let Some(instrument) = request.currency.as_ref() {
        let instrument = instrument.to_lowercase();
        let asset_type = request.asset_type.unwrap_or_default();
        let ledger_account_id = match asset_type {
            AssetType::Regulated => context.get_currency_regulated_account(&instrument).await?,
            AssetType::IndirectCbdc => context.get_currency_cbdc_account(&instrument).await?,
            _ => return Err(Error::internal_msg("unsupported asset type")),
        };
        let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
        let query =
            Asset::find_by_account_id_instrument_type_scoped(*id, &instrument, asset_type, scope)?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;

        // create transaction
        let transfer = TransferBuilder::new().step(
            StepBuilder::new(
                ledger_account_id.as_slice().try_into()?,
                asset.ledger_account_id.as_slice().try_into()?,
                request.amount_in_cents,
            )
            .metadata(Deposit::default()),
        );
        m10_sdk::transfer(&context.ledger, transfer).await?;
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("{id}/sandbox/open")]
async fn open(
    id: Path<i64>,
    current_user: User,
    context: Data<Context>,
) -> Result<Json<Account>, Error> {
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let mut conn = context.db_pool.get().await?;
    let mut txn = conn.begin().await?;
    let mut account = query
        .fetch_optional(&mut *txn)
        .await?
        .ok_or_else(|| Error::not_found("account"))?;
    if let Some(account_ref) = account.bank_reference.as_ref() {
        context.bank.open_account(account_ref).await?;
    }
    account.status = AccountStatus::Open;
    account.update_status(&mut *txn).await?;
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
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
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
        let txn = bank.transfer_by_id(txn_id).await?;
        let asset_type = txn
            .refernce
            .split(':')
            .last()
            .ok_or_else(|| Error::not_found("asset type in deposit reference"))?
            .try_into()?;

        let ledger_account_id = match asset_type {
            AssetType::Regulated => context.get_currency_regulated_account(&instrument).await?,
            AssetType::IndirectCbdc => context.get_currency_cbdc_account(&instrument).await?,
            _ => return Err(Error::internal_msg("unsupported asset type")),
        };

        let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
        let query =
            Asset::find_by_account_id_instrument_type_scoped(id, &instrument, asset_type, scope)?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;

        // create transaction
        let transfer = TransferBuilder::new().step(
            StepBuilder::new(
                ledger_account_id.as_slice().try_into()?,
                asset.ledger_account_id.as_slice().try_into()?,
                amount,
            )
            .metadata(Deposit::default()),
        );
        let tx_id = m10_sdk::transfer(&context.ledger, transfer).await?;
        info!("Deposit mirrored on ledger: {}", tx_id);
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
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
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
    let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
    let query = Account::find_by_id_scoped(*id, scope)?;
    let request = request.into_inner();
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
        let asset_type = request.asset_type.unwrap_or_default();
        let ledger_account_id = match asset_type {
            AssetType::Regulated => context.get_currency_regulated_account(&instrument).await?,
            AssetType::IndirectCbdc => context.get_currency_cbdc_account(&instrument).await?,
            _ => return Err(Error::internal_msg("unsupported asset type")),
        };
        let scope = current_user.is_authorized(BankEmulatorRole::Read)?;
        let query =
            Asset::find_by_account_id_instrument_type_scoped(*id, &instrument, asset_type, scope)?;
        let asset = query
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(Error::unauthorized)?;

        // create transaction
        let transfer = TransferBuilder::new().step(
            StepBuilder::new(
                asset.ledger_account_id.as_slice().try_into()?,
                ledger_account_id.as_slice().try_into()?,
                request.amount_in_cents,
            )
            .metadata(Withdraw::default()),
        );
        let tx_id = m10_sdk::transfer(&context.ledger, transfer).await?;
        Some(tx_id)
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
        .service(convert_into)
        .service(convert_direct_from)
        .service(convert_from)
        .service(deposit)
        .service(fund)
        .service(open)
        .service(settle_deposit)
        .service(settle_withdraw)
        .service(withdraw)
}
