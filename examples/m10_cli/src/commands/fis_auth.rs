use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use bytes::{Buf, Bytes};
use clap::Args;
use http_body_util::{BodyExt, Full};
use hyper::Method;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::utils::m10_config_path;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
/// Obtains OAuth id and access token and writes them to
/// id.token
/// access.token
pub(crate) struct FisAuth {
    /// Auth URL to authenticate against
    /// e.g. "https://login-uat.fisglobal.com/idp/tikkabank243/rest/1.0"
    #[arg(long, alias = "au")]
    pub(crate) auth_url: String,
    /// oauth client secret
    #[arg(long, alias = "cs")]
    pub(crate) client_secret: String,
    /// Writes access token to stdout
    #[arg(long, alias = "so", default_value_t)]
    pub(crate) stdout: bool,
}

impl FisAuth {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let FisAuth {
            auth_url,
            client_secret,
            stdout,
        } = self;

        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http1()
            .build();
        let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new()).build(connector);

        // 1. request device code
        // encode device code request
        let mut device_code_body = HashMap::new();
        device_code_body.insert("client_id", "drm_cli");
        device_code_body.insert("scope", "openid");
        device_code_body.insert("response_type", "code");
        let encoded_device_code_body = serde_urlencoded::to_string(&device_code_body)?;

        // send device code request
        let device_flow_start_time = Instant::now();
        let request = hyper::Request::builder()
            .uri(format!("{}/device/code", auth_url))
            .method(Method::POST)
            .header("X-SunGard-IdP-API-Key", "SunGard-IdP-UI")
            .header(
                hyper::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(Full::from(encoded_device_code_body))?;
        let response = client.request(request).await?;

        // handle device code request error
        let status = response.status();
        if !status.is_success() {
            anyhow::bail!(
                "device code request failed: {}\n response: {:?}",
                status,
                response,
            );
        }

        // parse required oauth parameters
        let response = response.into_body().collect().await?.to_bytes();
        let response = serde_json::from_reader::<_, serde_json::Value>(response.reader())?;
        let verification_url = urlencoding::decode(
            response["verification_url"]
                .as_str()
                .expect("response did not contain verification URL"),
        )?;
        let user_code = response["user_code"]
            .as_str()
            .expect("response did not contain user code");
        let device_code = response["device_code"]
            .as_str()
            .expect("response did not contain device code");
        let interval = Duration::from_secs(
            response["interval"]
                .as_str()
                .expect("response did not contain interval")
                .parse::<u64>()
                .expect("interval could not be parsed"),
        );
        let expires_in = Duration::from_secs(
            response["expires_in"]
                .as_u64()
                .expect("response did not contain expires_in"),
        );

        // 2. user browser-based log in
        println!("Log in at {}", verification_url);
        println!("Your verification code is {}", user_code);

        // 3. loop access token request while user logs in until device code expiry
        let mut access_token_body = HashMap::new();
        access_token_body.insert("client_id", "drm_cli");
        access_token_body.insert("client_secret", &client_secret);
        access_token_body.insert("code", device_code);
        access_token_body.insert("scope", "openid");
        access_token_body.insert("grant_type", "http://oauth.net/grant_type/device/1.0");

        let access_token_response = loop {
            let req = hyper::Request::builder()
                .uri(format!("{}/accesstoken", auth_url))
                .method(Method::POST)
                .header("X-SunGard-IdP-API-Key", "SunGard-IdP-UI")
                .header(hyper::header::ACCEPT, "application/json")
                .header(
                    hyper::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(Full::from(serde_urlencoded::to_string(&access_token_body)?))?;
            let res = client.request(req).await?;
            if res.status().is_success() || device_flow_start_time.elapsed() > expires_in {
                break res;
            }
            sleep(interval).await;
        };

        // verify auth completed successfully
        let status = access_token_response.status();
        if !status.is_success() {
            anyhow::bail!(
                "access code request failed: {}\n response: {:?}",
                status,
                access_token_response,
            );
        }

        // parse access token from response
        let access_token_response = access_token_response
            .into_body()
            .collect()
            .await?
            .to_bytes();
        let access_token_response =
            serde_json::from_reader::<_, serde_json::Value>(access_token_response.reader())?;
        let id_token = access_token_response["id_token"]
            .as_str()
            .expect("response did not contain id_token");

        if *stdout {
            println!("Authenticated successfully, access token: {}", id_token);
        } else {
            println!("Authenticated successfully");
        }

        // 4. write access token to filesystem
        let m10_config_path = m10_config_path();
        std::fs::create_dir_all(&m10_config_path)?;
        std::fs::write(m10_config_path.join("access.token"), id_token)?;

        Ok(())
    }
}
