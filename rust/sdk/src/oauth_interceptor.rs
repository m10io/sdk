use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use toml;
use tonic::{service::Interceptor, Request, Status};

#[derive(Clone)]
pub struct OauthInterceptor {
    access_token: Arc<Mutex<Option<String>>>,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    oauth: OauthConfig,
}

#[derive(Deserialize, Clone)]
pub struct OauthConfig {
    client_id: String,
    client_secret: String,
    base_url: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

fn load_config(filepath: &str) -> Result<Config, std::io::Error> {
    let contents = fs::read_to_string(filepath)?;
    toml::from_str(&contents).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

// TODO: Add handling for expired tokens
/// Handles fetching and adding access tokens to requests. If config is not present,
/// or any errors occur with the interceptor, this will pass any gRPC requests without
/// modification.
impl OauthInterceptor {
    pub fn new() -> Self {
        let interceptor = Self {
            access_token: Arc::new(Mutex::new(None)),
        };

        let interceptor_clone = interceptor.clone();
        tokio::spawn(async move {
            if let Ok(cfg) = load_config("config.toml") {
                let _ = interceptor_clone
                    .set_token_from_http(
                        format!("{}/accesstoken", cfg.oauth.base_url,).as_str(),
                        &cfg.oauth.client_id,
                        &cfg.oauth.client_secret,
                    )
                    .await;
            }
        });

        interceptor
    }

    pub fn set_token(&self, token: &str) {
        let mut token_guard = self.access_token.lock().unwrap();
        *token_guard = Some(token.into());
    }

    pub fn clear_token(&self) {
        let mut token_guard = self.access_token.lock().unwrap();
        *token_guard = None;
    }

    fn get_token(&self) -> Option<String> {
        self.access_token.lock().unwrap().clone()
    }

    pub async fn set_token_from_http(
        &self,
        url: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<(), reqwest::Error> {
        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-SunGard-IdP-API-Key",
            HeaderValue::from_str("SunGard-IdP-UI").unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );

        let encoded_params = serde_urlencoded::to_string(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .unwrap();

        let response = client
            .post(url)
            .headers(headers)
            .body(encoded_params)
            .send()
            .await?
            .error_for_status()?;

        let token_response: TokenResponse = response.json().await?;
        self.set_token(&token_response.access_token);
        Ok(())
    }
}

impl Interceptor for OauthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(token) = self.get_token() {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
            Ok(request)
        } else {
            // pass-through if access token is not initialized
            Ok(request)
        }
    }
}
