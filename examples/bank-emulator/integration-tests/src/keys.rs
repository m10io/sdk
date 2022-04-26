use super::base_url;
use super::utils::*;

#[tokio::test]
#[ignore]
async fn key_routes() {
    let jwt = prepopulated_user_jwt().await;
    let client = reqwest::Client::default();
    let key = public_key();

    // add key
    client
        .put(format!("{}/api/v1/keys", base_url()))
        .bearer_auth(&jwt)
        .body(key)
        .send()
        .await
        .unwrap()
        .assert_success()
        .await;
}
