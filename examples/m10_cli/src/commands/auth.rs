use std::time::Instant;

use bytes::{Buf, Bytes};
use clap::Args;
use http_body_util::{BodyExt, Full};
use hyper::Method;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, json, to_vec, Value};
use tokio::time::{sleep, Duration};

use crate::{context::Context, utils::m10_config_path};

const OAUTH_AUDIENCE: &str = "https://api.m10.net";
const OAUTH_SCOPE: &str = "openid";

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
/// Obtains OAuth id and access token and writes them to
/// id.token
/// access.token
pub(crate) struct Auth {
    #[arg(short, long, aliases = ["name", "user", "un"])]
    pub(crate) username: Option<String>,
    #[arg(long, alias = "pw")]
    pub(crate) password: Option<String>,
    /// Writes access token to stdout
    #[arg(long, alias = "so", default_value_t)]
    pub(crate) stdout: bool,
}

impl Auth {
    pub(crate) async fn run(&self, context: &Context) -> anyhow::Result<()> {
        let Auth {
            username,
            password,
            stdout,
        } = self;

        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http1()
            .build();
        let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new()).build(connector);

        let response = if let Some(username) = username {
            let oauth_token_body = json!({
                "client_id": "directory",
                "grant_type": "password",
                "username": username,
                "password": password.as_ref().unwrap(),
                "audience": OAUTH_AUDIENCE,
                "scope": OAUTH_SCOPE,
            });
            let request = hyper::Request::builder()
                .uri(format!("{}oauth/token", context.addr()?))
                .method(Method::POST)
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .body(Full::from(to_vec(&oauth_token_body)?))?;
            client.request(request).await?
        } else {
            let device_code_body = json!({
                "client_id": "directory",
                "audience": OAUTH_AUDIENCE,
                "scope": OAUTH_SCOPE,
            });
            let request = hyper::Request::builder()
                .uri(format!("{}oauth/device/code", context.addr()?))
                .method(Method::POST)
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .body(Full::from(to_vec(&device_code_body)?))?;
            let device_flow_time = Instant::now();
            let response = client.request(request).await?;

            let status = response.status();
            let response = response.into_body().collect().await?.to_bytes();
            let response = from_reader::<_, Value>(response.reader())?;
            if !status.is_success() {
                anyhow::bail!(response);
            }

            println!("Log in at {}", response["verification_uri_complete"]);
            let interval = Duration::from_secs(response["interval"].as_u64().unwrap());
            let expires_in = Duration::from_secs(response["expires_in"].as_u64().unwrap());
            let oauth_token_body = json!({
                "client_id": "directory",
                "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
                "device_code": response["device_code"],
            });
            loop {
                let request = hyper::Request::builder()
                    .uri(format!("{}oauth/token", context.addr()?))
                    .method(Method::POST)
                    .header(hyper::header::CONTENT_TYPE, "application/json")
                    .body(Full::from(to_vec(&oauth_token_body)?))?;
                let response = client.request(request).await?;
                if response.status().is_success() || device_flow_time.elapsed() > expires_in {
                    break response;
                }
                sleep(interval).await;
            }
        };
        let status = response.status();
        let response = response.into_body().collect().await?.to_bytes();
        let response = from_reader::<_, Value>(response.reader())?;
        if !status.is_success() {
            anyhow::bail!(response);
        }
        let m10_config_path = m10_config_path();
        let access_token = response["access_token"].as_str().unwrap();
        let id_token = response["id_token"].as_str().unwrap();

        std::fs::create_dir_all(&m10_config_path)?;
        std::fs::write(m10_config_path.join("id.token"), id_token)?;
        std::fs::write(m10_config_path.join("access.token"), access_token)?;
        if *stdout {
            println!("{}", access_token);
        } else {
            println!("Authenticated successfully!");
        }
        Ok(())
    }
}
