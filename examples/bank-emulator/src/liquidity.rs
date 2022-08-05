use m10_bank_emulator_protos::rtgs::{
    rtgs_service_client::RtgsServiceClient, AccountRequest, AccountType, TransferRequest,
};
use m10_sdk::{sdk, Signer};
use tonic::Request;
use tracing::info;

use crate::{config::CurrencyConfig, context::Context, error::Error};

pub(crate) async fn reserve_cbdc(
    amount: u64,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<Vec<u8>, Error> {
    let cbdc = currency
        .cbdc
        .as_ref()
        .ok_or_else(|| Error::not_found("indirect CBDC configuration"))?;

    let mut client = context.ledger.clone();

    let account = cbdc
        .ledger_account_id(currency.account_owner.as_ref(), context)
        .await?;

    // check balance of ledger account
    let request = context
        .signer
        .sign_request(sdk::GetAccountRequest {
            id: account.to_vec(),
        })
        .await?;
    let indexed_account = client.get_indexed_account(request).await?;
    let issuance = indexed_account.issuance.unwrap(); // Note this is an issuance account

    // Note: Balance check is overly simplified, will be refined with [M10-3553]
    let request_amount = if indexed_account.balance < cbdc.reserve_balance_low_bound() as u64 {
        cbdc.reserve_balance_high_bound() as u64 - indexed_account.balance
    } else if issuance.issued_balance + amount > indexed_account.balance {
        cbdc.customer_limit * cbdc.reserve_threshold as u64
    } else {
        0
    };
    if request_amount > 0 {
        let rtgs_config = currency
            .rtgs
            .as_ref()
            .ok_or_else(|| Error::not_found("RTGS configuration"))?;
        let mut rtgs = RtgsServiceClient::new(rtgs_config.addr.connect_lazy()?);

        let account_req = AccountRequest {
            institute: rtgs_config.institute_name.clone(),
            account_type: AccountType::Cbdc as i32,
            currency_symbol: currency.code.to_uppercase(),
        };

        let resp = rtgs.find_account(Request::new(account_req)).await?;
        let cbdc_reserve_account = resp.into_inner();

        let transfer_req = TransferRequest {
            account: *currency.reserve_account_id(context).await?,
            receiver: cbdc_reserve_account.account_number,
            amount: amount as i64,
            currency_symbol: currency.code.to_uppercase(),
            instructions: serde_json::to_string(&serde_json::json!({
                "cbdc_receiver": hex::encode(&account),
            }))?,
        };

        let resp = rtgs.transfer_funds(Request::new(transfer_req)).await?;
        info!(
            "requested {}{} CBDC/{}",
            amount,
            currency.code.to_uppercase(),
            hex::encode(&resp.into_inner().txn_id),
        );
    }

    Ok(account.to_vec())
}
