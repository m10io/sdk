use m10_bank_emulator::models::*;

use super::base_url;
use super::utils::*;

#[tokio::test]
async fn notification_preferences_routes() {
    let jwt = prepopulated_user_jwt().await;
    let client = reqwest::Client::default();

    // list notification preferences no filter
    let p = client
        .get(format!("{}/api/v1/notification_preferences", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, NotificationPreferences>>()
        .await;
    assert_eq!(p.data.len(), 4);

    // list notification preferences filtered by device
    let p = client
        .get(format!(
            "{}/api/v1/notification_preferences?device_token=android_12",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, NotificationPreferences>>()
        .await;
    assert_eq!(p.data.len(), 2);

    println!("A");
    // list notification preferences filtered by instrument
    let p = client
        .get(format!(
            "{}/api/v1/notification_preferences?instrument=usd",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, NotificationPreferences>>()
        .await;
    assert_eq!(p.data.len(), 2);

    // list notification preferences filtered by device and instrument
    let p = client
        .get(format!(
            "{}/api/v1/notification_preferences?device_token=android_12&instrument=usd",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, NotificationPreferences>>()
        .await;
    assert_eq!(p.data.len(), 1);

    // list own notification preferences by asset
    let p = client
        .get(format!(
            "{}/api/v1/assets/usd/notification_preferences",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, NotificationPreferences>>()
        .await;
    assert_eq!(p.data.len(), 2);

    // list own notification preferences by asset filtered by device
    let p = client
        .get(format!(
            "{}/api/v1/assets/usd/notification_preferences?device_token=android_12",
            base_url()
        ))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, NotificationPreferences>>()
        .await;
    assert_eq!(p.data.len(), 2);

    // create notification preferences
    let np = client
        .post(format!(
            "{}/api/v1/assets/usd/notification_preferences",
            base_url()
        ))
        .bearer_auth(&jwt)
        .json(&CreateNotificationPreferencesRequest {
            device_token: "android_12".into(),
            notification_toggles: 0b000010.into(),
        })
        .send()
        .await
        .unwrap()
        .assert_json::<NotificationPreferences>()
        .await;
    assert_eq!(np.id, 25);

    // update notification preferences
    let np = client
        .put(format!("{}/api/v1/notification_preferences/25", base_url()))
        .bearer_auth(&jwt)
        .json(&UpdateNotificationPreferencesRequest {
            device_token: None,
            notification_toggles: Some(0b000010.into()),
        })
        .send()
        .await
        .unwrap()
        .assert_json::<NotificationPreferences>()
        .await;
    assert_eq!(np.notification_toggles, 0b000010.into());
}
