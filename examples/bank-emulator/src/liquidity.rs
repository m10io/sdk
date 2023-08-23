#![allow(dead_code)]
use m10_bank_emulator_protos::emulator::rtgs::{
    requisition_funds_request::RequisitionType, rtgs_service_client::RtgsServiceClient,
    AccountRequest, AccountType, BalanceRequest, RequisitionFundsRequest, TransferRequest,
};
use m10_sdk::{sdk::Withdraw, Metadata};
use serde_json::json;
use tonic::Request;
use tracing::info;

use crate::{
    bank::Bank,
    config::CurrencyConfig,
    context::Context,
    error::Error,
    utils::{ledger_transfer, parent_of},
};

pub(crate) async fn check_or_inflate_reserves(
    amount: u64,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<u64, Error> {
    let mut rtgs = RtgsServiceClient::new(currency.rtgs_addr.connect_lazy()?);

    let reserve_account_id = currency.reserve_account_id(context).await?;

    let balance_req = BalanceRequest {
        account_number: *reserve_account_id,
    };

    let balance = rtgs
        .get_balance(Request::new(balance_req))
        .await?
        .into_inner();

    let (new_balance, overflow) = balance.balance.overflowing_sub(amount.try_into()?);
    if overflow {
        return Err(Error::internal_msg("amount too large"));
    }

    let reserve_loan = if new_balance.is_negative() {
        let (required, overflow) = currency
            .reserve_config
            .nominal_balance
            .overflowing_add(new_balance.abs().try_into()?);
        if overflow {
            return Err(Error::internal_msg("required amount too large"));
        }
        required
    } else if new_balance
        < currency
            .reserve_config
            .reserve_balance_low_bound()
            .try_into()?
    {
        currency.reserve_config.nominal_balance - u64::try_from(new_balance)?
    } else {
        0
    };
    if reserve_loan != 0 {
        let fund_req = RequisitionFundsRequest {
            account: *reserve_account_id,
            amount: reserve_loan.try_into()?,
            currency_code: currency.code.to_uppercase(),
            instructions: "".into(),
            requisition_type: RequisitionType::GeneralLoan as i32,
        };

        let resp = rtgs
            .requisition_funds(Request::new(fund_req))
            .await?
            .into_inner();

        info!(
            "requested {}{} reserve loan/{}",
            reserve_loan,
            currency.code.to_uppercase(),
            hex::encode(&resp.txn_id),
        );

        let mut bank = context.bank.clone();
        // add loan to reserve loan account
        bank.issue_to(
            *currency
                .reserve_config
                .reserve_loan_account_id
                .get()
                .ok_or_else(|| Error::internal_msg("no reserve loan account initialized"))?,
            reserve_loan,
            &hex::encode(&resp.txn_id),
        )
        .await?;
        Ok(resp.new_balance as u64)
    } else {
        Ok(balance.balance as u64)
    }
}

pub(crate) async fn check_or_deflate_reserves(
    amount: u64,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<u64, Error> {
    let mut rtgs = RtgsServiceClient::new(currency.rtgs_addr.connect_lazy()?);

    let reserve_account_id = currency.reserve_account_id(context).await?;

    let balance_req = BalanceRequest {
        account_number: *reserve_account_id,
    };

    let balance = rtgs
        .get_balance(Request::new(balance_req))
        .await?
        .into_inner();

    let old_balance = u64::try_from(balance.balance)?;
    let (new_balance, overflow) = old_balance.overflowing_add(amount);
    if overflow {
        return Err(Error::internal_msg("amount too large"));
    }

    if new_balance > currency.reserve_config.reserve_balance_high_bound() {
        let loan_return = new_balance - currency.reserve_config.nominal_balance;
        let account_req = AccountRequest {
            institute: "M10 Reserve".into(),
            account_type: AccountType::Reserve as i32,
            currency_code: currency.code.to_uppercase(),
        };

        let cb_account = rtgs
            .find_account(Request::new(account_req))
            .await?
            .into_inner();
        let transfer_req = TransferRequest {
            account: *reserve_account_id,
            receiver: cb_account.account_number,
            amount: loan_return.try_into()?,
            currency_code: currency.code.to_uppercase(),
            instructions: "".to_string(),
        };

        let resp = rtgs
            .transfer_funds(Request::new(transfer_req))
            .await?
            .into_inner();

        info!(
            "released {}{} reserve loan/{}",
            loan_return,
            currency.code.to_uppercase(),
            hex::encode(&resp.txn_id),
        );

        let mut bank = context.bank.clone();
        // add loan to reserve loan account
        bank.destroy_from(
            *currency
                .reserve_config
                .reserve_loan_account_id
                .get()
                .ok_or_else(|| Error::internal_msg("no reserve loan account initialized"))?,
            loan_return,
            &hex::encode(&resp.txn_id),
        )
        .await?;
        Ok(resp.new_balance as u64)
    } else {
        Ok(balance.balance as u64)
    }
}

pub(crate) async fn reserve_cbdc(
    amount: u64,
    account: Vec<u8>,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<(), Error> {
    check_or_inflate_reserves(amount, currency, context).await?;

    let mut rtgs = RtgsServiceClient::new(currency.rtgs_addr.connect_lazy()?);

    let account_req = AccountRequest {
        institute: currency.cb_name.clone(),
        account_type: AccountType::Cbdc as i32,
        currency_code: currency.code.to_uppercase(),
    };

    let resp = rtgs.find_account(Request::new(account_req)).await?;
    let cbdc_reserve_account = resp.into_inner();

    let transfer_req = TransferRequest {
        account: *currency.reserve_account_id(context).await?,
        receiver: cbdc_reserve_account.account_number,
        amount: amount.try_into()?,
        currency_code: currency.code.to_uppercase(),
        instructions: serde_json::to_string(&serde_json::json!({
            "cbdc_receiver": hex::encode(&account),
        }))?,
    };

    let resp = rtgs.transfer_funds(Request::new(transfer_req)).await?;
    info!(
        "requested {}{} CBDC/{}",
        amount,
        currency.code.to_uppercase(),
        hex::encode(resp.into_inner().txn_id),
    );
    Ok(())
}

pub(crate) async fn release_cbdc(
    amount: u64,
    account: Vec<u8>,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<(), Error> {
    let mut rtgs = RtgsServiceClient::new(currency.rtgs_addr.connect_lazy()?);

    let cbdc_reserve_account = parent_of(&account[..])?;
    let ledger_txn = ledger_transfer(
        account.to_vec(),
        cbdc_reserve_account,
        amount,
        vec![Withdraw::default().any()],
        context,
    )
    .await?;

    let reserve_account_id = currency.reserve_account_id(context).await?;
    let fund_req = RequisitionFundsRequest {
        account: *reserve_account_id,
        amount: amount.try_into()?,
        currency_code: currency.code.to_uppercase(),
        requisition_type: RequisitionType::Swap as i32,
        instructions: json!({ "cbdc_return": ledger_txn }).to_string(),
    };

    let resp = rtgs
        .requisition_funds(Request::new(fund_req.clone()))
        .await?
        .into_inner();

    check_or_deflate_reserves(amount, currency, context).await?;

    info!(
        "released {}{} CBDC/{}",
        amount,
        currency.code.to_uppercase(),
        hex::encode(&resp.txn_id),
    );
    Ok(())
}

pub(crate) async fn reserve_drc_holding(
    amount: u64,
    account: Vec<u8>,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<(), Error> {
    check_or_inflate_reserves(amount, currency, context).await?;

    let mut rtgs = RtgsServiceClient::new(currency.rtgs_addr.connect_lazy()?);

    let account_req = AccountRequest {
        institute: currency.cb_name.clone(),
        account_type: AccountType::Cbdc as i32,
        currency_code: currency.code.to_uppercase(),
    };

    let resp = rtgs.find_account(Request::new(account_req)).await?;
    let cbdc_reserve_account = resp.into_inner();

    let transfer_req = TransferRequest {
        account: *currency.reserve_account_id(context).await?,
        receiver: cbdc_reserve_account.account_number,
        amount: amount.try_into()?,
        currency_code: currency.code.to_uppercase(),
        instructions: serde_json::to_string(&serde_json::json!({
            "cbdc_receiver": hex::encode(&account),
        }))?,
    };

    let resp = rtgs.transfer_funds(Request::new(transfer_req)).await?;
    info!(
        "requested {}{} DRC backing CBDC/{}",
        amount,
        currency.code.to_uppercase(),
        hex::encode(resp.into_inner().txn_id),
    );

    Ok(())
}

pub(crate) async fn release_drc_holding(
    amount: u64,
    account: Vec<u8>,
    currency: &CurrencyConfig,
    context: &Context,
) -> Result<(), Error> {
    let mut rtgs = RtgsServiceClient::new(currency.rtgs_addr.connect_lazy()?);

    let cbdc_reserve_account = parent_of(&account[..])?;
    let ledger_txn = ledger_transfer(
        account.to_vec(),
        cbdc_reserve_account,
        amount,
        vec![Withdraw::default().any()],
        context,
    )
    .await?;

    let reserve_account_id = currency.reserve_account_id(context).await?;
    let fund_req = RequisitionFundsRequest {
        account: *reserve_account_id,
        amount: amount.try_into()?,
        currency_code: currency.code.to_uppercase(),
        requisition_type: RequisitionType::Swap as i32,
        instructions: json!({ "cbdc_return": ledger_txn }).to_string(),
    };

    let resp = rtgs
        .requisition_funds(Request::new(fund_req.clone()))
        .await?
        .into_inner();

    check_or_deflate_reserves(amount, currency, context).await?;

    info!(
        "released {}{} DRC backing CBDC/{}",
        amount,
        currency.code.to_uppercase(),
        hex::encode(&resp.txn_id),
    );

    Ok(())
}
