use super::base_url;
use super::utils::*;
use m10_bank_emulator::models::*;
use reqwest::Body;
use serde_json::json;
use serde_json::Value;

#[tokio::test]
async fn unimplemented_contacts_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // send notification
    client
        .post(format!("{}/api/v1/contacts/11/notification", base_url()))
        .bearer_auth(&jwt)
        .json(&Value::default())
        .send()
        .await
        .expect("notification response")
        .assert_status(500)
        .await;

    // add relationship
    client
        .post(format!("{}/api/v1/contacts/relationship/12", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("add relationship response")
        .assert_status(500)
        .await;
}

#[tokio::test]
async fn user_unauthorized_contacts_assets_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // freeze asset
    println!("freeze asset");
    client
        .post(format!(
            "{}/api/v1/contacts/11/assets/usd/freeze",
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
            "{}/api/v1/contacts/11/assets/usd/payments",
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
            "{}/api/v1/contacts/11/assets/usd/payments/3241",
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
            "{}/api/v1/contacts/11/assets/usd/unfreeze",
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
async fn unimplemented_contacts_documents_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // list document metadata
    client
        .get(format!("{}/api/v1/contacts/11/documents", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list documents response")
        .assert_status(500)
        .await;

    // upload documents
    client
        .post(format!("{}/api/v1/contacts/11/documents", base_url()))
        .bearer_auth(&jwt)
        .body(Body::from(""))
        .send()
        .await
        .expect("upload documents response")
        .assert_status(500)
        .await;

    // get document metadata
    client
        .get(format!("{}/api/v1/contacts/11/documents/23-78", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list documents response")
        .assert_status(500)
        .await;

    // update document metadata
    client
        .patch(format!("{}/api/v1/contacts/11/documents/23-78", base_url()))
        .bearer_auth(&jwt)
        .json(&Value::default())
        .send()
        .await
        .expect("list documents response")
        .assert_status(500)
        .await;
}

#[tokio::test]
async fn unimplemented_contacts_sandbox_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // trigger KYB
    client
        .post(format!(
            "{}/api/v1/contacts/11/sandbox/trigger_kyb",
            base_url()
        ))
        .bearer_auth(&jwt)
        .json(&Value::default())
        .send()
        .await
        .expect("trigger KYB response")
        .assert_status(500)
        .await;

    // verify KYC/KYB documents
    client
        .post(format!(
            "{}/api/v1/contacts/11/documents/23-78/sandbox/verify",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("verify documents response")
        .assert_status(500)
        .await;
}

#[tokio::test]
async fn contacts_sandbox_routes() {
    let jwt = create_or_get_user("omega-sandbox-user@m10test.io").await;
    let client = reqwest::Client::default();
    delete_contact(&client, &jwt).await;

    let req = CreateContactRequest {
        tenant: "m10-test".into(),
        contact_data: serde_json::to_value(json!({
            "name": "default",
            "email": "omega-sandbox-user@m10test.io",
        }))
        .unwrap(),
        contact_type: Some(ContactType::Individual),
        ..Default::default()
    };

    // create sandbox customer
    println!("create sandbox customer");
    let contact = client
        .post(format!("{}/api/v1/contacts", base_url()))
        .bearer_auth(&jwt)
        .json(&req)
        .send()
        .await
        .unwrap()
        .assert_json::<Contact>()
        .await;

    // approve cip
    println!("sandbox: approve contact");
    client
        .post(format!(
            "{}/api/v1/contacts/{}/sandbox/approve",
            base_url(),
            contact.id
        ))
        .bearer_auth(&jwt)
        .json(&AmountRequest::default())
        .send()
        .await
        .expect("approve cip response")
        .assert_success()
        .await;

    // deny cip
    println!("sandbox: deny contact");
    client
        .post(format!(
            "{}/api/v1/contacts/{}/sandbox/deny",
            base_url(),
            contact.id
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("deny cip response")
        .assert_success()
        .await;

    // freeze
    println!("sandbox: freeze contact");
    client
        .post(format!(
            "{}/api/v1/contacts/{}/sandbox/freeze",
            base_url(),
            contact.id
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("freeze response")
        .assert_success()
        .await;

    // unfreeze
    println!("sandbox: unfreeze contact");
    client
        .post(format!(
            "{}/api/v1/contacts/{}/sandbox/unfreeze",
            base_url(),
            contact.id
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("unfreeze response")
        .assert_success()
        .await;
}

#[tokio::test]
async fn contacts_crud() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();
    delete_contact(&client, &jwt).await;

    let req = CreateContactRequest {
        tenant: "m10-test".into(),
        contact_data: serde_json::to_value(json!({
            "name": "default",
            "email": "omega-default-user@m10test.io",
        }))
        .unwrap(),
        contact_type: Some(ContactType::Individual),
        ..Default::default()
    };

    // create default customer
    println!("create default customer");
    let contact = client
        .post(format!("{}/api/v1/contacts", base_url()))
        .bearer_auth(&jwt)
        .json(&req)
        .send()
        .await
        .unwrap()
        .assert_json::<Contact>()
        .await;

    // get customer by id
    let c = client
        .get(format!("{}/api/v1/contacts/{}", base_url(), contact.id))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Contact>()
        .await;
    assert_eq!(c.id, contact.id, "get customer by id");

    // get own customer directly
    let resp = client
        .get(format!("{}/api/v1/contacts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Contact>>()
        .await;
    assert_eq!(resp.data.len(), 1, "get own customer directly");
    assert_eq!(resp.data[0].id, contact.id, "get own customer directly");

    // delete customer by id not allowed
    println!("delete customer by id not allowed");
    client
        .delete(format!("{}/api/v1/contacts/{}", base_url(), contact.id))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("delete contact response")
        .assert_status(401)
        .await;

    // retire own customer
    let c = client
        .delete(format!("{}/api/v1/contacts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Contact>()
        .await;
    assert_eq!(c.id, contact.id, "retire own customer");

    // can't repeatedly retire own customer
    println!("can't repeatedly retire own customer");
    client
        .delete(format!("{}/api/v1/contacts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("delete contact response")
        .assert_status(401)
        .await;

    // get customer expect to not be found by id after delete
    println!("get customer expect to not be found by id after delete");
    client
        .get(format!("{}/api/v1/contacts/{}", base_url(), contact.id))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get customer response")
        .assert_status(404)
        .await;
}

#[tokio::test]
async fn contacts_account() {
    let jwt = prepopulated_user_jwt().await;
    let client = reqwest::Client::default();

    // get account
    let account = client
        .get(format!("{}/api/v1/contacts/19/accounts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Account>()
        .await;
    assert_eq!(account.id, 19, "get account by owner");
}

#[tokio::test]
#[ignore]
async fn contacts_key() {
    let jwt = prepopulated_user_jwt().await;
    let client = reqwest::Client::default();

    // add key
    client
        .put(format!("{}/api/v1/contacts/19/key", base_url()))
        .bearer_auth(&jwt)
        .body(Body::from(""))
        .send()
        .await
        .unwrap()
        .assert_success()
        .await;
}

#[tokio::test]
async fn contacts_pagination() {
    let admin_jwt = admin_jwt().await;
    let client = reqwest::Client::default();
    let mut pages = 0;
    let mut resp = client
        .get(format!("{}/api/v1/contacts?limit=5", base_url()))
        .bearer_auth(&admin_jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Contact>>()
        .await;
    println!("items returned: {}", resp.data.len());
    println!("- first: {:?}", resp.data.first());
    println!("- last: {:?}", resp.data.last());
    while let Some(next) = resp.next {
        println!("Next: {:?}", next);
        resp = client
            .get(format!(
                "{}/api/v1/contacts?limit=5&id={}",
                base_url(),
                next.id,
            ))
            .bearer_auth(&admin_jwt)
            .send()
            .await
            .unwrap()
            .assert_json::<ListResponse<i32, Contact>>()
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
async fn contacts_assets_routes() {
    let jwt = admin_jwt().await;
    let client = reqwest::Client::default();

    // list assets
    let resp = client
        .get(format!("{}/api/v1/contacts/11/assets", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Asset>>()
        .await;
    assert_eq!(resp.data.len(), 2);

    // get asset
    let asset = client
        .get(format!("{}/api/v1/contacts/11/assets/usd", base_url()))
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
