use m10_bank_emulator::models::*;

use super::base_url;
use super::utils::*;

#[tokio::test]
async fn owned_assets_routes() {
    let jwt = prepopulated_user_jwt().await;
    let client = reqwest::Client::default();

    // list assets
    let assets = client
        .get(format!("{}/api/v1/assets", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Asset>>()
        .await;
    assert_eq!(assets.data.len(), 2);

    // get asset
    let asset = client
        .get(format!("{}/api/v1/assets/usd", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<Asset>()
        .await;
    assert_eq!(asset.id, 23);

    // list payments
    let resp = client
        .get(format!("{}/api/v1/assets/usd/payments", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("list payment response for asset");
    if resp.status() == 200 {
        let _ = resp.assert_json::<ListResponse<u64, Payment>>().await;
    } else {
        assert_eq!(404, resp.status(), "get 'not found' list payment for asset");
    }

    // get payment
    let resp = client
        .get(format!("{}/api/v1/assets/usd/payments/3241", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get payment response for asset");
    if resp.status() == 200 {
        let _ = resp.assert_json::<Payment>().await;
    } else {
        assert_eq!(404, resp.status(), "get 'not found' get payment for asset");
    }
}

#[tokio::test]
async fn payments_routes() {
    let admin_jwt = admin_jwt().await;
    let client = reqwest::Client::default();

    // list payments
    let resp = client
        .get(format!("{}/api/v1/assets/usd/payments", base_url()))
        .bearer_auth(&admin_jwt)
        .send()
        .await
        .expect("list payment response for asset");
    if resp.status() == 200 {
        let _ = resp.assert_json::<ListResponse<u64, Payment>>().await;
    } else {
        assert_eq!(404, resp.status(), "get 'not found' list payment for asset");
    }

    // get payment
    let resp = client
        .get(format!("{}/api/v1/assets/usd/payments/3241", base_url()))
        .bearer_auth(&admin_jwt)
        .send()
        .await
        .expect("get payment response for asset");
    if resp.status() == 200 {
        let _ = resp.assert_json::<Payment>().await;
    } else {
        assert_eq!(404, resp.status(), "get 'not found' get payment for asset");
    }
}
