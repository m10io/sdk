use super::base_url;
use super::utils::*;
use reqwest::Body;
use serde_json::Value;

#[tokio::test]
async fn documents_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // list document metadata
    client
        .get(format!("{}/api/v1/documents", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list documents response")
        .assert_status(500)
        .await;

    // upload documents
    client
        .post(format!("{}/api/v1/documents", base_url()))
        .bearer_auth(&jwt)
        .body(Body::from(""))
        .send()
        .await
        .expect("upload documents response")
        .assert_status(500)
        .await;

    // get document metadata
    client
        .get(format!("{}/api/v1/documents/23-78", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list documents response")
        .assert_status(500)
        .await;

    // update document metadata
    client
        .patch(format!("{}/api/v1/documents/23-78", base_url()))
        .bearer_auth(&jwt)
        .json(&Value::default())
        .send()
        .await
        .expect("list documents response")
        .assert_status(500)
        .await;
}

#[tokio::test]
async fn documents_sandbox_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // verify KYC/KYB documents
    client
        .post(format!(
            "{}/api/v1/documents/23-78/sandbox/verify",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("verify documents response")
        .assert_status(500)
        .await;
}
