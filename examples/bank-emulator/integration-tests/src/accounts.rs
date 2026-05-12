use m10_bank_emulator::models::*;
use m10_sdk::{Signature, Signer};
use m10_sdk::{StepBuilder, TransferBuilder};
use serde_json::json;
use serde_json::Value;
use std::time::Duration;

use super::base_url;
use super::utils::*;

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
struct Recon {
    currency: String,
    cla: i64,
    sum_traditional: i64,
    tdl: ReconTdl,
    issuance: ReconTdl,
    diffs: ReconDiffs,
}
#[derive(serde::Deserialize, Debug)]
struct ReconTdl {
    regulated: i64,
    indirect_cbdc: i64,
}
#[derive(serde::Deserialize, Debug)]
struct ReconDiffs {
    cla_vs_sum_traditional: i64,
    tdl_regulated_vs_issuance: i64,
    tdl_cbdc_vs_issuance: i64,
}

async fn reconcile(client: &reqwest::Client, jwt: &str, ccy: &str) -> Recon {
    const MAX_ATTEMPTS: usize = 5;
    let url = format!("{}/api/v1/reconciliation/{}", base_url(), ccy);

    for attempt in 1..=MAX_ATTEMPTS {
        match client.get(&url).bearer_auth(jwt).send().await {
            Ok(resp) if resp.status().is_success() => return resp.assert_json::<Recon>().await,
            Ok(resp) => {
                if attempt == MAX_ATTEMPTS {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    panic!(
                        "reconciliation failed after {} attempts: [{}] {}",
                        MAX_ATTEMPTS, status, body
                    );
                }
            }
            Err(err) => {
                if attempt == MAX_ATTEMPTS {
                    panic!(
                        "reconciliation request failed after {} attempts: {}",
                        MAX_ATTEMPTS, err
                    );
                }
            }
        }

        tokio::time::sleep(Duration::from_millis((attempt as u64) * 250)).await;
    }

    unreachable!("retry loop should return or panic");
}

#[tokio::test]
async fn accounts_wire_routes() {
    let jwt = create_or_get_user("omega-user-test-wire@m10test.io").await;
    let client = reqwest::Client::default();
    delete_contact(&client, &jwt).await;
    delete_account(&client, &jwt).await;

    let key_pair = key_pair();
    let signature = key_pair.sign_payload(key_pair.public_key()).await.unwrap();

    let req = CreateAccountRequest {
        tenant: "m10-test".into(),
        contact: serde_json::to_value(json!({
            "name": "default",
            "email": "omega-user-test-wire@m10test.io",
        }))
        .unwrap(),
        contact_type: Some(ContactType::Individual),
        assets: Some(vec!["usd".into()]),
        signatures: vec![Signature::from(signature)],
        account_set_id: None,
    };

    // create account
    println!("create account for wire");
    let account = client
        .post(format!("{}/api/v1/accounts", base_url()))
        .bearer_auth(&jwt)
        .json(&req)
        .send()
        .await
        .unwrap()
        .assert_json::<Account>()
        .await;

    // open
    println!("open account for wire");
    client
        .post(format!(
            "{}/api/v1/accounts/{}/sandbox/open",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("open account response")
        .assert_success()
        .await;

    let admin = admin_jwt().await;
    let r0 = reconcile(&client, &admin, "usd").await;
    println!("reconciliation before: {:?}", r0);

    // fund
    println!("fund account for wire");
    client
        .post(format!(
            "{}/api/v1/accounts/{}/sandbox/fund",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            amount_in_cents: 50000,
            currency: Some("usd".into()),
            ..Default::default()
        })
        .send()
        .await
        .expect("fund account response")
        .assert_success()
        .await;
    let r1 = reconcile(&client, &admin, "usd").await;
    println!("reconciliation after: {:?}", r1);
    assert_eq!(r1.cla - r0.cla, 50_000, "CLA should increase on fund");
    assert_eq!(r1.diffs.cla_vs_sum_traditional, 0);

    // deposit
    let txn = client
        .post(format!(
            "{}/api/v1/accounts/{}/deposit",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            currency: None,
            amount_in_cents: 700,
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
        .assert_json::<Value>()
        .await;

    let txn_id = txn.get("bank_tx").and_then(|i| i.as_str()).expect("txn id");

    // settle deposit
    client
        .post(format!(
            "{}/api/v1/accounts/{}/sandbox/settle_deposit/{}",
            base_url(),
            account.id,
            txn_id,
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("settle deposit response")
        .assert_success()
        .await;
    let r2 = reconcile(&client, &admin, "usd").await;
    assert_eq!(
        r2.cla - r1.cla,
        700,
        "CLA should increase on settled deposit"
    );
    assert_eq!(r2.diffs.cla_vs_sum_traditional, 0);

    // withdraw
    let txn = client
        .post(format!(
            "{}/api/v1/accounts/{}/withdraw",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            currency: None,
            amount_in_cents: 500,
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
        .assert_json::<Value>()
        .await;

    let txn_id = txn.get("bank_tx").and_then(|i| i.as_str()).expect("txn id");

    // settle withdraw
    client
        .post(format!(
            "{}/api/v1/accounts/{}/sandbox/settle_withdraw/{}",
            base_url(),
            account.id,
            txn_id,
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("settle withdraw response")
        .assert_success()
        .await;
    let r3 = reconcile(&client, &admin, "usd").await;
    assert_eq!(
        r3.cla - r2.cla,
        -500,
        "CLA should decrease on settled withdraw"
    );
    assert_eq!(r3.diffs.cla_vs_sum_traditional, 0);

    // fund ledger account from bank account
    println!("fund ledger account from bank account");

    let balance_before = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );

    let transfer = client
        .post(format!(
            "{}/api/v1/accounts/{}/request_funds",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            amount_in_cents: 10000,
            currency: Some("usd".into()),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
        .assert_json::<BankTransfer>()
        .await;
    println!("transfer {:?}", transfer);
    assert_eq!(transfer.status, 0);
    assert_eq!(transfer.amount, Some(10000));

    let balance_after = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );
    println!("balance after transfer: {}", balance_after);
    assert_eq!(balance_after + 10000, balance_before);
    let r4 = reconcile(&client, &admin, "usd").await;
    assert_eq!(r4.cla - r3.cla, -10_000, "CLA must decrease on issuance");
    assert_eq!(
        r4.tdl.regulated - r3.tdl.regulated,
        10_000,
        "TDL(reg) must increase on issuance"
    );
    assert_eq!(r4.diffs.tdl_regulated_vs_issuance, 0);

    // fund CBDC account from bank account
    println!("fund CBDC account from bank account");

    let balance_before = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );

    let transfer = client
        .post(format!(
            "{}/api/v1/accounts/{}/request_funds",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            amount_in_cents: 5500,
            currency: Some("usd".into()),
            asset_type: Some(AssetType::IndirectCbdc),
        })
        .send()
        .await
        .unwrap()
        .assert_json::<BankTransfer>()
        .await;
    println!("transfer {:?}", transfer);
    assert_eq!(transfer.status, 0);
    assert_eq!(transfer.amount, Some(5500));

    let balance_after = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );
    println!("balance after transfer: {}", balance_after);
    assert_eq!(balance_after + 5500, balance_before);
    let r5 = reconcile(&client, &admin, "usd").await;
    assert_eq!(r5.cla - r4.cla, -5_500);
    assert_eq!(r5.tdl.indirect_cbdc - r4.tdl.indirect_cbdc, 5_500);
    assert_eq!(r5.diffs.tdl_cbdc_vs_issuance, 0);

    // Note temporary disabled, need to find a new way to
    // test failed transfer since this fails now before the actual transfer
    // fund invalid ledger account from bank account
    // println!("fund invalid ledger account from bank account");
    // let transfer = client
    //     .post(format!(
    //     "{}/api/v1/accounts/{}/request_funds?ledger_account_id=03800000000000001f0000000000000a",
    //     base_url(),
    //     account.id
    // ))
    //     .bearer_auth(&jwt)
    //     .json(&AmountRequest {
    //         amount_in_cents: 50000,
    //         currency: Some("usd".into()),
    //         ..Default::default()
    //     })
    //     .send()
    //     .await
    //     .unwrap()
    //     .assert_json::<BankTransfer>()
    //     .await;
    // println!("transfer {:?}", transfer);
    // assert_eq!(transfer.status, 1);
    // assert!(transfer.error.is_some());

    // Redeem with transaction
    println!("Redeem with transaction");
    let asset = client
        .get(format!("{}/api/v1/assets/usd", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Asset>()
        .await;
    println!("asset: {:?}", asset);
    let bank_asset = client
        .get(format!("{}/api/v1/assets/usd/bank_account", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Asset>()
        .await;
    println!("bank asset: {:?}", bank_asset);

    let ledger = crate::ledger_client(key_pair);

    let transfer = TransferBuilder::new().step(StepBuilder::new(
        asset.ledger_account_id.as_slice().try_into().unwrap(),
        bank_asset.ledger_account_id.as_slice().try_into().unwrap(),
        2200,
    ));

    let txn_id = m10_sdk::transfer(&ledger, transfer).await.unwrap();

    let balance_before = balance_after;
    let transfer = client
        .post(format!(
            "{}/api/v1/accounts/{}/redeem",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&RedeemRequest {
            txn_id,
            amount_in_cents: 2200,
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
        .assert_json::<BankTransfer>()
        .await;
    println!("transfer {:?}", transfer);
    assert_eq!(transfer.status, 0);
    assert_eq!(transfer.amount, Some(2200));

    let balance_after = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );
    println!("balance after transfer: {}", balance_after);
    assert_eq!(balance_after - 2200, balance_before);
    let r6 = reconcile(&client, &admin, "usd").await;
    assert_eq!(r6.cla - r5.cla, 2_200);
    assert_eq!(r6.tdl.regulated - r5.tdl.regulated, -2_200);
    assert_eq!(r6.diffs.tdl_regulated_vs_issuance, 0);

    // Redeem direct
    println!("Redeem direct");
    let balance_before = balance_after;
    let transfer = client
        .post(format!(
            "{}/api/v1/accounts/{}/redeem_direct",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&RedeemRequest {
            txn_id,
            amount_in_cents: 3300,
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
        .assert_json::<BankTransfer>()
        .await;
    println!("transfer {:?}", transfer);
    assert_eq!(transfer.status, 0);
    assert_eq!(transfer.amount, Some(3300));

    let balance_after = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );
    println!("balance after transfer: {}", balance_after);
    assert_eq!(balance_after - 3300, balance_before);
    let r7 = reconcile(&client, &admin, "usd").await;
    assert_eq!(r7.cla - r6.cla, 3_300);
    assert_eq!(r7.tdl.regulated - r6.tdl.regulated, -3_300);
    assert_eq!(r7.diffs.tdl_regulated_vs_issuance, 0);

    // Redeem CBDC direct
    println!("Redeem CBDC direct");
    let balance_before = balance_after;
    let transfer = client
        .post(format!(
            "{}/api/v1/accounts/{}/redeem_direct",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&RedeemRequest {
            txn_id,
            amount_in_cents: 1100,
            asset_type: Some(AssetType::IndirectCbdc),
        })
        .send()
        .await
        .unwrap()
        .assert_json::<BankTransfer>()
        .await;
    println!("transfer {:?}", transfer);
    assert_eq!(transfer.status, 0);
    assert_eq!(transfer.amount, Some(1100));

    let balance_after = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );
    println!("balance after transfer: {}", balance_after);
    assert_eq!(balance_after - 1100, balance_before);
    let r8 = reconcile(&client, &admin, "usd").await;
    assert_eq!(r8.cla - r7.cla, 1_100);
    assert_eq!(r8.tdl.indirect_cbdc - r7.tdl.indirect_cbdc, -1_100);
    assert_eq!(r8.diffs.tdl_cbdc_vs_issuance, 0);

    // fund CBDC account from bank account over limit
    println!("fund CBDC account from bank account over limit");

    let balance_before = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );

    let assets = client
        .get(format!("{}/api/v1/assets", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Asset>>()
        .await;

    let regulated_asset = assets
        .data
        .iter()
        .find(|a| a.asset_type == AssetType::Regulated)
        .expect("regulated asset");
    let cbdc_asset = assets
        .data
        .iter()
        .find(|a| a.asset_type == AssetType::IndirectCbdc)
        .expect("cbdc asset");

    let regulated_account_before = ledger
        .get_account(
            regulated_asset
                .ledger_account_id
                .as_slice()
                .try_into()
                .unwrap(),
        )
        .await
        .expect("regulated ledger account");

    let transfer = client
        .post(format!(
            "{}/api/v1/accounts/{}/request_funds",
            base_url(),
            account.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            amount_in_cents: 1100_00,
            currency: Some("usd".into()),
            asset_type: Some(AssetType::IndirectCbdc),
        })
        .send()
        .await
        .unwrap()
        .assert_json::<BankTransfer>()
        .await;
    println!("transfer {:?}", transfer);
    assert_eq!(transfer.status, 0);
    assert_eq!(transfer.amount, Some(1100_00));

    // Expect transfer to happen
    let balance_after = account_balance(
        &client
            .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<Account>()
            .await,
    );
    println!("balance after transfer: {}", balance_after);
    assert_eq!(balance_after, balance_before - 1100_00);
    let r9 = reconcile(&client, &admin, "usd").await;
    let delta_cla = r9.cla - r8.cla;
    let delta_tdl_total =
        (r9.tdl.indirect_cbdc - r8.tdl.indirect_cbdc) + (r9.tdl.regulated - r8.tdl.regulated);
    assert_eq!(delta_cla, -110_000, "CLA must decrease by issued amount");
    assert_eq!(
        delta_tdl_total, 110_000,
        "Total TDL must increase by issued amount"
    );

    // wait for adjustment
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    let mut sleep_ms = 200u64;
    let (cbdc_ok, regulated_ok, last_cbdc_bal, last_reg_bal) = loop {
        let regulated_account_after = ledger
            .get_account(
                regulated_asset
                    .ledger_account_id
                    .as_slice()
                    .try_into()
                    .unwrap(),
            )
            .await
            .expect("regulated ledger account");

        let cbdc_account = ledger
            .get_account(cbdc_asset.ledger_account_id.as_slice().try_into().unwrap())
            .await
            .expect("cbdc ledger account");

        let cbdc_ok = cbdc_account.balance <= 1000_00;
        let regulated_ok = regulated_account_after.balance > regulated_account_before.balance;

        if cbdc_ok && regulated_ok {
            break (
                cbdc_ok,
                regulated_ok,
                cbdc_account.balance,
                regulated_account_after.balance,
            );
        }
        if std::time::Instant::now() >= deadline {
            break (
                cbdc_ok,
                regulated_ok,
                cbdc_account.balance,
                regulated_account_after.balance,
            );
        }

        let _ = reconcile(&client, &admin, "usd").await;

        tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
        sleep_ms = std::cmp::min(sleep_ms.saturating_mul(2), 2000);
    };

    assert!(
        cbdc_ok,
        "cbdc balance above limit (last={} expected<=100000)",
        last_cbdc_bal
    );
    assert!(
        regulated_ok,
        "regulated account balance not increased (before={} last={})",
        regulated_account_before.balance, last_reg_bal
    );

    let rec_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let (aligned, last_diff, r10) = loop {
        let r = reconcile(&client, &admin, "usd").await;
        let diff = r.diffs.tdl_cbdc_vs_issuance;
        if diff == 0 {
            break (true, diff, r);
        }
        if std::time::Instant::now() >= rec_deadline {
            break (false, diff, r);
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    };
    assert!(aligned, "TDL(CBDC) != issuance (diff = {})", last_diff);
    assert_eq!(r10.diffs.tdl_cbdc_vs_issuance, 0, "TDL(CBDC) != issuance");
    assert_eq!(
        r10.diffs.tdl_regulated_vs_issuance, 0,
        "TDL(REG) != issuance"
    );
    assert_eq!(
        r10.diffs.cla_vs_sum_traditional, 0,
        "CLA != sum(traditional)"
    );
}

#[tokio::test]
async fn user_unauthorized_accounts_assets_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // freeze asset
    println!("freeze asset");
    client
        .post(format!(
            "{}/api/v1/accounts/11/assets/usd/freeze",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("freeze asset response")
        .assert_status(401)
        .await;

    // list payments
    println!("list payments");
    client
        .get(format!(
            "{}/api/v1/accounts/11/assets/usd/payments",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list payments response")
        .assert_status(401)
        .await;

    // get payment
    println!("get payment");
    client
        .get(format!(
            "{}/api/v1/accounts/11/assets/usd/payments/3241",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get payments response")
        .assert_status(401)
        .await;

    // unfreeze asset
    println!("unfreeze asset");
    client
        .post(format!(
            "{}/api/v1/accounts/11/assets/usd/unfreeze",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("unfreeze asset response")
        .assert_status(401)
        .await;
}

#[tokio::test]
async fn accounts_sandbox_routes() {
    let jwt = prepopulated_user_jwt().await;
    let client = reqwest::Client::default();

    // fund
    client
        .post(format!("{}/api/v1/accounts/19/sandbox/fund", base_url()))
        .bearer_auth(&jwt)
        .json(&AmountRequest {
            amount_in_cents: 500,
            currency: None,
            ..Default::default()
        })
        .send()
        .await
        .expect("fund account response")
        .assert_success()
        .await;

    // open
    client
        .post(format!("{}/api/v1/accounts/19/sandbox/open", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("open account response")
        .assert_success()
        .await;
}

#[tokio::test]
async fn accounts_crud() {
    let jwt = create_or_get_user("omega-user-with-account@m10test.io").await;
    let client = reqwest::Client::default();
    delete_contact(&client, &jwt).await;
    delete_account(&client, &jwt).await;

    let req = CreateAccountRequest {
        tenant: "m10-test".into(),
        contact: serde_json::to_value(json!({
            "name": "default",
            "email": "omega-user-with-account@m10test.io",
        }))
        .unwrap(),
        contact_type: Some(ContactType::Individual),
        ..Default::default()
    };

    // create account
    println!("create account");
    let account = client
        .post(format!("{}/api/v1/accounts", base_url()))
        .bearer_auth(&jwt)
        .json(&req)
        .send()
        .await
        .unwrap()
        .assert_json::<Account>()
        .await;

    // Check that traditional account was created and funded
    let balance = account_balance(&account);
    assert!(balance > 0, "account not funded");

    // get account by id
    let a = client
        .get(format!("{}/api/v1/accounts/{}", base_url(), account.id))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Account>()
        .await;
    assert_eq!(a.id, account.id, "get account by id");

    // get own account directly
    let resp = client
        .get(format!("{}/api/v1/accounts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Account>>()
        .await;
    assert_eq!(resp.data.len(), 1, "get own account directly");
    assert_eq!(resp.data[0].id, account.id, "get own account directly");

    // delete account by id not allowed
    println!("delete account by id not allowed");
    client
        .delete(format!("{}/api/v1/accounts/{}", base_url(), account.id))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("delete account response")
        .assert_status(401)
        .await;
}

#[tokio::test]
async fn accounts_pagination() {
    let admin_jwt = admin_jwt().await;
    let client = reqwest::Client::default();
    let mut pages = 0;
    let mut resp = client
        .get(format!("{}/api/v1/accounts?limit=5", base_url()))
        .bearer_auth(&admin_jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Account>>()
        .await;
    println!("items returned: {}", resp.data.len());
    println!("- first: {:?}", resp.data.first());
    println!("- last: {:?}", resp.data.last());
    while let Some(next) = resp.next {
        println!("Next: {:?}", next);
        resp = client
            .get(format!(
                "{}/api/v1/accounts?limit=5&id={}",
                base_url(),
                next.id,
            ))
            .bearer_auth(&admin_jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<ListResponse<i32, Account>>()
            .await;
        println!("items returned: {}", resp.data.len());
        println!("- first: {:?}", resp.data.first());
        println!("- last: {:?}", resp.data.last());
        pages += 1;
        if pages >= 5 {
            break;
        }
    }
    // Expect at least 3 full pages
    assert!(pages >= 3, "too less pages")
}

#[tokio::test]
async fn accounts_assets_routes() {
    let jwt = admin_jwt().await;
    let client = reqwest::Client::default();

    // list assets
    let resp = client
        .get(format!("{}/api/v1/accounts/11/assets", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Asset>>()
        .await;
    assert_eq!(resp.data.len(), 2);

    // get asset
    let asset = client
        .get(format!("{}/api/v1/accounts/11/assets/usd", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Asset>()
        .await;
    assert_eq!(
        asset.ledger_account_id,
        hex::decode("0280000100000e000000000000000003").unwrap()
    );
}
