use m10_bank_emulator::models::{Account, Contact, ListResponse};
use reqwest::{Response, StatusCode};
use ring::{
    digest, rand,
    signature::{self, KeyPair},
};
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use tokio::sync::OnceCell;

use crate::{base_url, ledger_addr};

static DEFAULT_USER_JWT: OnceCell<String> = OnceCell::const_new();
static PREPOPULATED_USER_JWT: OnceCell<String> = OnceCell::const_new();
static ADMIN_JWT: OnceCell<String> = OnceCell::const_new();

pub(crate) fn public_key() -> String {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    let peer_public_key_bytes = key_pair.public_key().as_ref();
    base64::encode(peer_public_key_bytes)
}

pub async fn admin_jwt() -> String {
    ADMIN_JWT
        .get_or_init(|| async {
            create_or_get_auth_token("omega-admin@m10test.io", "trust:admin openid").await
        })
        .await
        .clone()
}

pub async fn delete_contact(client: &reqwest::Client, jwt: &str) {
    let resp = client
        .get(format!("{}/api/v1/contacts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Contact>>()
        .await;

    let admin_jwt = admin_jwt().await;
    for contact in resp.data {
        client
            .delete(format!("{}/api/v1/contacts/{}", base_url(), contact.id))
            .bearer_auth(&admin_jwt)
            .send()
            .await
            .unwrap()
            .assert_success()
            .await;
    }
}

pub async fn delete_account(client: &reqwest::Client, jwt: &str) {
    let resp = client
        .get(format!("{}/api/v1/accounts", base_url()))
        .bearer_auth(&jwt)
        .send()
        .await
        .unwrap()
        .assert_json::<ListResponse<i32, Account>>()
        .await;

    let admin_jwt = admin_jwt().await;
    for account in resp.data {
        client
            .delete(format!("{}/api/v1/accounts/{}", base_url(), account.id))
            .bearer_auth(&admin_jwt)
            .send()
            .await
            .unwrap()
            .assert_success()
            .await;
    }
}

pub(crate) async fn default_user_jwt() -> String {
    DEFAULT_USER_JWT
        .get_or_init(|| async { create_or_get_user("omega-default-user@m10test.io").await })
        .await
        .clone()
}

pub(crate) async fn prepopulated_user_jwt() -> String {
    PREPOPULATED_USER_JWT
        .get_or_init(|| async { create_or_get_user("omega-prepopulated-user@m10test.io").await })
        .await
        .clone()
}

pub async fn create_or_get_user(email: &str) -> String {
    // TODO: Own scope for omega Bank
    let jwt = create_or_get_auth_token(email, "trust:user openid").await;
    jwt
}

pub async fn create_or_get_auth_token(email: &str, scope: &str) -> String {
    if let Some(jwt) = login(email, scope).await {
        return jwt;
    }
    println!("user {} doesn't exist, signing up", email);
    signup(email).await;
    login(email, scope).await.expect("failed to login")
}

async fn login(email: &str, scope: &str) -> Option<String> {
    let request = [
        ("grant_type", "password".to_string()),
        ("username", email.to_string()),
        ("password", password(email)),
        ("audience", "https://api.m10.net".to_string()),
        ("scope", scope.to_string()),
    ]
    .into_iter()
    .collect::<HashMap<&'static str, String>>();
    let client = reqwest::Client::new();
    client
        .post(format!("{}/oauth/token", ledger_addr()))
        .json(&request)
        .send()
        .await
        .unwrap()
        .json::<JWTResp>()
        .await
        .ok()
        .map(|jwt_resp| jwt_resp.access_token)
}

fn password(username: &str) -> String {
    // deterministic based on username, complies with Auth0 password policy
    let username_hash = digest::digest(&digest::SHA256, username.as_bytes());
    format!("{}-aA0", &hex::encode(username_hash)[..10])
}

async fn signup(email: &str) {
    let request = [
        ("given_name", "integration tester".to_string()),
        ("family_name", "tester".to_string()),
        ("phone", "+15555555555".to_string()),
        ("tenant", "m10".to_string()),
        ("m10UserId", uuid::Uuid::new_v4().to_string()),
        ("email", email.to_string()),
        ("password", password(email)),
        ("connection", "Username-Password-Authentication".to_string()),
    ]
    .into_iter()
    .collect::<HashMap<&'static str, String>>();
    let client = reqwest::Client::new();
    client
        .post(format!("{}/oauth/signup", ledger_addr()))
        .json(&request)
        .send()
        .await
        .unwrap()
        .assert_success()
        .await;
}

#[derive(serde::Deserialize)]
struct JWTResp {
    access_token: String,
}

#[async_trait::async_trait]
pub(super) trait FallibleRequest {
    async fn assert_json<T: DeserializeOwned>(self) -> T;
    async fn assert_success(self);
    async fn assert_status<T>(self, expected_status: T)
    where
        T: Display + Debug + Send,
        StatusCode: PartialEq<T>;
}

#[async_trait::async_trait]
impl FallibleRequest for Response {
    async fn assert_json<T: DeserializeOwned>(self) -> T {
        let status = self.status();
        if !status.is_success() {
            let msg = self.text().await.expect("Error did not contain text");
            panic!("[{}] {}", status, msg)
        } else {
            self.json::<T>()
                .await
                .expect("Could not deserialize payload")
        }
    }

    async fn assert_success(self) {
        let status = self.status();
        if !status.is_success() {
            let msg = self.text().await.expect("Error did not contain text");
            panic!("[{}] {}", status, msg);
        }
    }

    async fn assert_status<T>(self, expected_status: T)
    where
        T: Display + Debug + Send,
        StatusCode: PartialEq<T>,
    {
        let status = self.status();
        if !status.is_success() && status != expected_status {
            let msg = self.text().await.expect("Error did not contain text");
            eprintln!("[{}] {}", status, msg);
        }
        assert_eq!(status, expected_status);
    }
}
