use std::future::Future;

use crate::error::Error;
use crate::models::{AssetType, TransferChain};
use crate::{context::Context, models::Payment};

use m10_protos::{
    prost::Any,
    sdk::{CreateTransfer, TransferStep},
    MetadataExt,
};
use m10_sdk::{
    account::AccountId,
    contract::FinalizedContractExt,
    directory::{alias, Alias},
    sdk::{self, transaction_data::Data},
    EnhancedTransfer, LedgerClient, Signer,
};
use tracing::{error, info};
use uuid::Uuid;

const ALIAS_DEFAULT_OPERATOR: &str = "m10";

pub(crate) async fn submit_transaction(
    data: impl Into<Data>,
    context_id: Vec<u8>,
    context: &Context,
) -> Result<sdk::TransactionResponse, Error> {
    let payload = LedgerClient::transaction_request(data, context_id.clone());
    let signed_request = context.signer.sign_request(payload).await?;
    let client = context.ledger.clone();
    retry(
        || {
            let mut client = client.clone();
            let signed_request = signed_request.clone();
            async move { client.create_transaction(signed_request).await }
        },
        3,
    )
    .await
    .map_err(Error::from)?
    .tx_error()
    .map_err(Error::from)
}

#[allow(dead_code)]
pub(crate) async fn find_ledger_account(
    account_name: &str,
    owner: Option<&String>,
    context: &Context,
) -> Result<Vec<u8>, Error> {
    let owner = owner
        .map(|o| base64::decode(o).unwrap())
        .unwrap_or_else(|| context.signer.public_key().to_vec());
    let mut client = context.ledger.clone();
    let request = context
        .signer
        .sign_request(sdk::ListAccountMetadataRequest {
            filter: Some(sdk::list_account_metadata_request::Filter::Owner(owner)),
            page: None,
        })
        .await?;
    let docs = client
        .list_account_metadata(request)
        .await
        .map_err(|err| Error::internal_msg(err.to_string()))?;
    let account = docs
        .accounts
        .iter()
        .find(|a| a.public_name == account_name)
        .ok_or_else(|| {
            error!(name = %account_name, "Could not find account");
            Error::internal_msg("account not found")
        })?;
    Ok(account.id.to_vec())
}

pub(crate) async fn create_account_set(
    ledger_accounts: Vec<Vec<u8>>,
    context: &Context,
) -> Result<Uuid, Error> {
    let account_set_id = Uuid::new_v4();
    let payload = sdk::Operation::insert(sdk::AccountSet {
        id: account_set_id.as_bytes().to_vec(),
        owner: context.signer.public_key().to_vec(),
        accounts: ledger_accounts
            .into_iter()
            .map(|account_id| sdk::AccountRef {
                account_id,
                ledger_id: ALIAS_DEFAULT_OPERATOR.to_string(),
            })
            .collect(),
    });
    submit_transaction(payload, vec![], context).await?;
    Ok(account_set_id)
}

pub(crate) async fn create_ledger_account(
    parent_id: Vec<u8>,
    limit: u64,
    name: String,
    public_name: String,
    profile_image_url: Option<&str>,
    context: &Context,
) -> Result<Vec<u8>, Error> {
    let payload = sdk::CreateLedgerAccount {
        parent_id,
        balance_limit: limit,
        frozen: false,
        ..Default::default()
    };
    let ledger_account_id = submit_transaction(payload, vec![], context)
        .await?
        .account_created;
    let account_doc = sdk::AccountMetadata {
        id: ledger_account_id.clone(),
        owner: context.signer.public_key().to_vec(),
        name,
        public_name,
        profile_image_url: profile_image_url.unwrap_or_default().to_string(),
    };
    let payload = sdk::Operation::insert(account_doc);
    submit_transaction(payload, vec![], context).await?;
    Ok(ledger_account_id)
}

pub(crate) fn check_if_parent(child: &[u8], parent: &[u8]) -> Result<(), Error> {
    let child_id = AccountId::try_from_be_bytes((&child[0..16]).try_into()?)?;
    let parent_id = AccountId::try_from_be_bytes((&parent[0..16]).try_into()?)?;
    let expected_parent_id = child_id
        .parent_id()
        .ok_or_else(|| Error::internal_msg("holding account can't be a root account"))?;
    if parent_id != expected_parent_id {
        return Err(Error::internal_msg("accounts are not of same type"));
    }
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn is_parent(child: &[u8], parent: &[u8]) -> Result<bool, Error> {
    let child_id = AccountId::try_from_be_bytes((&child[0..16]).try_into()?)?;
    let parent_id = AccountId::try_from_be_bytes((&parent[0..16]).try_into()?)?;
    Ok(child_id.is_descendant_of(parent_id))
}

#[allow(dead_code)]
pub(crate) fn is_common_parent(
    parent_id: &[u8],
    child_a: &[u8],
    child_b: &[u8],
) -> Result<bool, Error> {
    let child_a = AccountId::try_from_be_bytes((&child_a[0..16]).try_into()?)?;
    let child_b = AccountId::try_from_be_bytes((&child_b[0..16]).try_into()?)?;
    let parent_id = AccountId::try_from_be_bytes((&parent_id[0..16]).try_into()?)?;
    Ok(child_a.is_descendant_of(parent_id) && child_b.is_descendant_of(parent_id))
}

pub(crate) fn parent_of(account: &[u8]) -> Result<Vec<u8>, Error> {
    let child_id = AccountId::try_from_be_bytes((&account[0..16]).try_into()?)?;
    Ok(child_id
        .parent_id()
        .ok_or_else(|| Error::internal_msg("holding account can't be a root account"))?
        .to_be_bytes()
        .to_vec())
}

pub(crate) async fn ledger_transfer(
    from_account_id: Vec<u8>,
    to_account_id: Vec<u8>,
    amount: u64,
    metadata: Vec<Any>,
    context: &Context,
) -> Result<u64, Error> {
    // create transaction
    let payload = LedgerClient::transaction_request(
        m10_sdk::ledger::transaction_data::Data::Transfer(CreateTransfer {
            transfer_steps: vec![TransferStep {
                from_account_id,
                to_account_id,
                amount,
                metadata,
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
    Ok(txn.tx_id)
}

async fn payment_from_transfer(
    transfer: EnhancedTransfer,
    instrument: &str,
    context: &Context,
) -> Result<Payment, Error> {
    let id = transfer.transfer.tx_id;
    let contract: Option<sdk::Contract> = transfer.transfer.metadata();
    let mut transfers = vec![];
    let contract_id = if let Some(contract) = contract {
        let contract_id = contract.id();
        let transfer_info = contract.transfer_info().map_err(|err| {
            error!(%err, "Invalid transfer info");
            Error::validation("transfers", err.to_string())
        })?;
        if transfer.transfer.context_id == contract_id {
            // TODO: @sadroeck This requires access to all involved ledgers
            // But the current setup is always a single ledger, so just assume it's a single one
            let ledger_id = &transfer_info[0].ledger_id;
            let req = sdk::ListTransferRequest {
                filter: Some(sdk::list_transfer_request::Filter::ContextId(
                    contract_id.clone(),
                )),
                ..Default::default()
            };
            let mut ledger = context.ledger.clone();
            let req = context.signer.sign_request(req).await?;
            let contract_transfers = ledger.list_transfers(req).await?;
            let contract_transfers = ledger
                .enhance_transfers(contract_transfers.transfers, &context.signer)
                .await?;
            for transfer in contract_transfers {
                transfers.push(TransferChain::try_from_transfer(
                    transfer,
                    ledger_id_to_instrument(ledger_id)?,
                )?);
            }
            Some(hex::encode(&contract_id))
        } else {
            // Not a contract
            None
        }
    } else {
        transfers.push(TransferChain::try_from_transfer(transfer, instrument)?);
        None
    };

    Ok(Payment {
        id,
        contract_id,
        transfers,
    })
}

pub(crate) async fn get_payment(
    instrument: &str,
    tx_id: u64,
    context: &Context,
) -> Result<Payment, Error> {
    let instrument = instrument.to_lowercase();
    let currency = context
        .config
        .currencies
        .get(&instrument)
        .ok_or_else(|| Error::internal_msg("unknown instrument"))?;

    let mut ledger = context.ledger.clone();
    let request = context
        .signer
        .sign_request(sdk::GetTransferRequest { tx_id })
        .await?;
    let transfer = match ledger.get_transfer(request).await {
        Ok(t) => t,
        Err(status) if status.code() == tonic::Code::NotFound => {
            return Err(Error::not_found("transfer"));
        }
        Err(status) => {
            info!("get transfer error {:?}", status);
            return Err(Error::internal_msg("getting transfers"));
        }
    };
    let transfer = ledger
        .enhance_transfer(transfer, &context.signer)
        .await
        .map_err(|err| {
            info!(%err, "enhancing transfer");
            Error::internal_msg("enhancing transfers")
        })?;

    let payment = payment_from_transfer(transfer, &currency.code, context)
        .await
        .map_err(|_| Error::internal_msg("getting transfers"))?;
    Ok(payment)
}

pub(crate) async fn list_payments(
    instrument: &str,
    min_tx_id: u64,
    limit: u64,
    include_child_accounts: bool,
    ledger_account_id: Option<Vec<u8>>,
    asset_type: AssetType,
    context: &Context,
) -> Result<Vec<Payment>, Error> {
    let mut ledger = context.ledger.clone();
    let ledger_account_id = match ledger_account_id {
        Some(id) => id,
        None => match asset_type {
            AssetType::Regulated => context.get_currency_regulated_account(instrument).await?,
            AssetType::IndirectCbdc => context.get_currency_cbdc_account(instrument).await?,
            _ => {
                return Err(Error::internal_msg("unsupported asset type"));
            }
        },
    };
    let req = context
        .signer
        .sign_request(sdk::ListTransferRequest {
            filter: Some(sdk::list_transfer_request::Filter::AccountId(
                ledger_account_id,
            )),
            min_tx_id,
            limit,
            include_child_accounts,
            ..Default::default()
        })
        .await?;
    let transfers = ledger
        .list_transfers(req)
        .await
        .map_err(|_| Error::internal_msg("getting transfers"))?;
    let mut transfers = ledger
        .enhance_transfers(transfers.transfers, &context.signer)
        .await
        .map_err(|_| Error::internal_msg("enhancing transfers"))?;

    let mut payments = vec![];

    for transfer in transfers.drain(..) {
        payments.push(
            payment_from_transfer(transfer, instrument, context)
                .await
                .map_err(|_| Error::internal_msg("getting transfers"))?,
        );
    }
    Ok(payments)
}

pub(crate) async fn create_alias_from_contact_data(
    contact: &serde_json::Value,
    token: &str,
    account_set_id: Uuid,
    context: &Context,
) -> Result<(), Error> {
    let name = contact
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::not_found("name in contact data"))?;

    let email = contact
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::not_found("email in contact data"))?;

    let alias = Alias {
        account_set_id: account_set_id.as_bytes().to_vec(),
        alias_type: alias::Type::Email.into(),
        handle: email.into(),
        display_name: name.into(),
        operator: ALIAS_DEFAULT_OPERATOR.to_string(),
    };
    let mut request = tonic::Request::new(alias);
    let access_token = tonic::metadata::MetadataValue::from_str(&format!("Bearer {}", token))?;
    request.metadata_mut().insert("authorization", access_token);
    if let Err(err) = context.directory.clone().create_alias(request).await {
        error!(%err, "alias creation failed");
    }
    Ok(())
}

async fn retry<T, E, F>(mut fut: impl FnMut() -> F, count: usize) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
{
    let mut i = 0;
    loop {
        match fut().await {
            Ok(ok) => return Ok(ok),
            Err(err) => {
                if i >= count {
                    return Err(err);
                }
            }
        }
        i += 1;
    }
}

fn ledger_id_to_instrument(ledger_id: &str) -> Result<&str, Error> {
    ledger_id
        .split('.')
        .next()
        .ok_or_else(|| Error::internal_msg("invalid ledger id"))
}
