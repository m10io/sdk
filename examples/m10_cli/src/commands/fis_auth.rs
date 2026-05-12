use anyhow::Context;
use bytes::{Buf, Bytes};
use clap::Args;
use http_body_util::{BodyExt, Full};
use hyper::Method;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::time::sleep;

use crate::utils::m10_config_path;

fn format_response_body(body: &[u8]) -> String {
    if body.is_empty() {
        return "<empty response body>".to_string();
    }

    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(body) {
        return serde_json::to_string_pretty(&json)
            .unwrap_or_else(|_| String::from_utf8_lossy(body).into_owned());
    }

    String::from_utf8_lossy(body).into_owned()
}

fn response_error_message(context: &str, status: hyper::StatusCode, body: &[u8]) -> String {
    format!(
        "{} failed with status {}.\nResponse body:\n{}",
        context,
        status,
        format_response_body(body)
    )
}

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
    pub(crate) async fn run(&self, ca_cert: Option<&str>) -> anyhow::Result<()> {
        let FisAuth {
            auth_url,
            client_secret,
            stdout,
        } = self;

        let connector = crate::tls::build_https_connector(ca_cert)?;
        let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new()).build(connector);

        // 1. request device code
        let mut device_code_body = HashMap::new();
        device_code_body.insert("client_id", "drm_cli");
        device_code_body.insert("scope", "openid");
        device_code_body.insert("response_type", "code");
        let encoded_device_code_body = serde_urlencoded::to_string(&device_code_body)?;

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

        let status = response.status();
        let response = response.into_body().collect().await?.to_bytes();
        if !status.is_success() {
            log::debug!(
                "device code request failed response [{}]:\n{}",
                status,
                format_response_body(response.as_ref())
            );
            anyhow::bail!(response_error_message(
                "device code request",
                status,
                response.as_ref()
            ));
        }

        // parse required oauth parameters
        let response = serde_json::from_reader::<_, serde_json::Value>(response.reader())?;
        let verification_url = urlencoding::decode(
            response["verification_url"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("response did not contain verification URL"))?,
        )?;
        let user_code = response["user_code"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("response did not contain user code"))?;
        let device_code = response["device_code"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("response did not contain device code"))?;
        let interval = Duration::from_secs(
            response["interval"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("response did not contain interval"))?
                .parse::<u64>()
                .context("interval could not be parsed")?,
        );
        let expires_in = Duration::from_secs(
            response["expires_in"]
                .as_u64()
                .ok_or_else(|| anyhow::anyhow!("response did not contain expires_in"))?,
        );

        // 2. user browser-based log in
        println!("Log in at {}", verification_url);
        println!("Your verification code is {}", user_code);

        // 3. loop access token request while user logs in until device code expiry
        let mut access_token_body = HashMap::new();
        access_token_body.insert("client_id", "drm_cli");
        access_token_body.insert("client_secret", client_secret);
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
            let status = res.status();
            let body = res.into_body().collect().await?.to_bytes();

            log::debug!(
                "access token response [{}]:\n{}",
                status,
                format_response_body(body.as_ref())
            );

            if status.is_success() {
                break body;
            }

            if device_flow_start_time.elapsed() > expires_in {
                anyhow::bail!(response_error_message(
                    "access token request",
                    status,
                    body.as_ref()
                ));
            }

            sleep(interval).await;
        };

        // parse access token from response
        let access_token_response =
            serde_json::from_reader::<_, serde_json::Value>(access_token_response.reader())?;
        let id_token = access_token_response["id_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("response did not contain id_token"))?;

        if *stdout {
            println!("Authenticated successfully, access token: {}", id_token);
        } else {
            println!("Authenticated successfully");
        }

        // 4. write access token to filesystem
        let m10_config_path = m10_config_path();
        std::fs::create_dir_all(&m10_config_path)
            .context("failed to create m10 config directory")?;
        std::fs::write(m10_config_path.join("access.token"), id_token)
            .context("failed to write access token")?;

        Ok(())
    }
}
