use serde_json::Value;

use super::base_url;
use super::utils::*;

#[tokio::test]
async fn transfer_methods_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    // list transfer methods
    client
        .get(format!("{}/api/v1/transfer_methods", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list transfer methods response")
        .assert_status(500)
        .await;

    // create transfer method
    client
        .post(format!("{}/api/v1/transfer_methods", base_url()))
        .bearer_auth(&jwt)
        .json(&Value::default())
        .send()
        .await
        .expect("create transfer method response")
        .assert_status(500)
        .await;

    // get transfer method
    client
        .get(format!("{}/api/v1/transfer_methods/44-66", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get transfer method response")
        .assert_status(500)
        .await;

    // deactivate transfer method
    client
        .post(format!(
            "{}/api/v1/transfer_methods/44-66/deactivate",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("deactivate transfer method response")
        .assert_status(500)
        .await;
}
