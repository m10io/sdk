use m10_bank_emulator::models::*;
use rust_decimal::Decimal;

use super::base_url;
use super::utils::*;

#[tokio::test]
async fn fees_routes() {
    let jwt = default_user_jwt().await;
    let client = reqwest::Client::default();

    let tr_fees = FeeMetadata {
        schedule: FeeSchedule {
            fees: vec![
                FeeBracket {
                    range: 0..=10,
                    polynomial: vec![Decimal::new(25, 0)],
                },
                FeeBracket {
                    range: 11..=20,
                    polynomial: vec![Decimal::new(50, 0)],
                },
                FeeBracket {
                    range: 21..=501,
                    polynomial: vec![Decimal::new(100, 0)],
                },
                FeeBracket {
                    range: 502..=u64::MAX,
                    polynomial: vec![Decimal::new(125, 0)],
                },
            ],
        },
        split: vec![
            FeeSplit {
                name: "M10".to_string(),
                percent: Decimal::new(75, 2),
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            FeeSplit {
                name: "DD".to_string(),
                percent: Decimal::new(25, 2),
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    };

    let wd_fees = FeeMetadata {
        schedule: FeeSchedule {
            fees: vec![
                FeeBracket {
                    range: 0..=10,
                    polynomial: vec![Decimal::new(50, 0)],
                },
                FeeBracket {
                    range: 11..=20,
                    polynomial: vec![Decimal::new(75, 0)],
                },
                FeeBracket {
                    range: 21..=501,
                    polynomial: vec![Decimal::new(100, 0)],
                },
                FeeBracket {
                    range: 502..=u64::MAX,
                    polynomial: vec![Decimal::new(125, 0)],
                },
            ],
        },
        split: vec![
            FeeSplit {
                name: "M10".to_string(),
                percent: Decimal::new(50, 2),
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            FeeSplit {
                name: "DD".to_string(),
                percent: Decimal::new(50, 2),
                account: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    };

    // insert/update transfer fee schedule
    client
        .post(format!("{}/api/v1/fees/ttt/transfer", base_url()))
        .bearer_auth(&jwt)
        .json(&tr_fees)
        .send()
        .await
        .expect("insert/update transfer fee schedule response")
        .assert_json::<FeeMetadata>()
        .await;

    // insert/update withdraw fee schedule
    client
        .post(format!("{}/api/v1/fees/ttt/withdraw", base_url()))
        .bearer_auth(&jwt)
        .json(&wd_fees)
        .send()
        .await
        .expect("insert/update withdraw fee schedule response")
        .assert_json::<FeeMetadata>()
        .await;

    // get transfer fee schedule
    let resp = client
        .get(format!("{}/api/v1/fees/ttt/transfer", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<FeeMetadata>()
        .await;
    assert_eq!(Decimal::new(75, 2), resp.split[0].percent);

    // get withdraw fee schedule
    let resp = client
        .get(format!("{}/api/v1/fees/ttt/withdraw", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<FeeMetadata>()
        .await;
    assert_eq!(Decimal::new(50, 2), resp.split[0].percent);

    // get transfer fee amount
    let resp = client
        .get(format!("{}/api/v1/fees/ttt/transfer/30300", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get transfer fee amount response")
        .assert_json::<FeeResponse>()
        .await;
    assert_eq!(94, resp.fees[0].amount);

    // get withdraw fee amount
    let resp = client
        .get(format!("{}/api/v1/fees/ttt/withdraw/60600", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get withdraw fee amount response")
        .assert_json::<FeeResponse>()
        .await;
    assert_eq!(63, resp.fees[0].amount);

    // get deposit fee amount
    // expect not found
    client
        .get(format!("{}/api/v1/fees/ttt/deposit/60600", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .expect("get deposit fee amount response")
        .assert_status(404)
        .await;
}
