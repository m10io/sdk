#![allow(dead_code)]
use actix_web::{web::Data, FromRequest};
use biscuit::{jwk::JWKSet, ClaimsSet, Empty, ValidationOptions, JWT};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, convert::TryFrom, future::Ready, time::Duration};
use tokio::sync::watch;
use tracing::error;

use crate::error::Error;

const JWKS_CACHE_DURATION: Duration = Duration::from_secs(15);

pub type Jwt = JWT<PrivateClaims, Empty>;
pub type Jwks = JWKSet<Empty>;

pub fn empty_jwks() -> Jwks {
    Jwks {
        keys: Vec::default(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceAccess<T> {
    pub roles: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources<T> {
    pub directory: Option<ResourceAccess<String>>,
    pub account: Option<ResourceAccess<String>>,
    #[serde(rename = "bank-emulator")]
    pub bank_emulator: Option<ResourceAccess<T>>,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateClaims {
    scope: String,
    resource_access: Resources<BankEmulatorRole>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Hash, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum BankEmulatorRole {
    AdminTest,
    Admin,
    User,
    Create,
    Read,
    Update,
    Delete,
    #[serde(rename = "scope:own")]
    ScopeOwn,
    #[serde(rename = "scope:m10")]
    ScopeM10,
    #[serde(rename = "scope:m10-test")]
    ScopeM10Test,
}

#[derive(Debug)]
pub struct User {
    pub scope: AuthScope,
    pub roles: HashSet<BankEmulatorRole>,
    pub user_id: String,
    pub token: String,
}

impl User {
    pub fn new(claims_set: ClaimsSet<PrivateClaims>) -> Result<Self, Error> {
        let user_id = claims_set
            .registered
            .subject
            .ok_or_else(Error::unauthorized)?;

        let resource_access = claims_set.private.resource_access;

        let resource = resource_access
            .bank_emulator
            .ok_or_else(Error::unauthorized)?;

        let all_scopes_sorted = vec![
            BankEmulatorRole::ScopeM10,
            BankEmulatorRole::ScopeM10Test,
            BankEmulatorRole::ScopeOwn,
        ];
        let mut scopes = all_scopes_sorted
            .iter()
            .filter(|s| resource.roles.contains(s));

        Ok(Self {
            scope: match scopes.next() {
                Some(BankEmulatorRole::ScopeM10) => AuthScope::Tenant("m10".to_string()),
                Some(BankEmulatorRole::ScopeM10Test) => AuthScope::Tenant("m10-test".to_string()),
                Some(BankEmulatorRole::ScopeOwn) => AuthScope::Own(user_id.clone()),
                _ => AuthScope::None,
            },
            roles: HashSet::from_iter(resource.roles.iter().cloned()),
            token: String::new(),
            user_id,
        })
    }

    pub fn is_authorized(&self, role: BankEmulatorRole) -> Result<AuthScope, Error> {
        self.roles
            .contains(&role)
            .then(|| role)
            .ok_or_else(Error::unauthorized)?;

        Ok(self.scope.clone())
    }
}

impl TryFrom<Jwt> for User {
    type Error = Error;
    fn try_from(jwt: Jwt) -> Result<Self, Error> {
        let (_, claims_set) = jwt.unwrap_decoded();
        let user = User::new(claims_set)?;
        Ok(user)
    }
}

pub fn validate_token(
    jwks_r: &watch::Receiver<Jwks>,
    validation_options: &ValidationOptions,
    token: &str,
) -> Result<Jwt, Error> {
    let jwt: Jwt = JWT::new_encoded(token).decode_with_jwks(&jwks_r.borrow(), None)?;
    jwt.validate(validation_options.clone()).map_err(|err| {
        error!("{:?}", err);
        Error::unauthorized()
    })?;
    Ok(jwt)
}

pub async fn watch_jwks(mut url: Url, jwks_s: watch::Sender<Jwks>) {
    url.set_path("/realms/master/protocol/openid-connect/certs");
    loop {
        if let Ok(jwks) = fetch_jwks(url.clone()).await {
            if jwks_s.send(jwks).is_err() {
                return;
            }
        }
        // TODO: respect cache headers
        tokio::time::sleep(JWKS_CACHE_DURATION).await;
    }
}

async fn fetch_jwks(url: Url) -> eyre::Result<Jwks> {
    let jwks = reqwest::get(url).await?.json().await?;
    Ok(jwks)
}

impl FromRequest for User {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        std::future::ready(user_from_request(req))
    }
}

fn user_from_request(req: &actix_web::HttpRequest) -> Result<User, Error> {
    let jwks = req
        .app_data::<Data<watch::Receiver<Jwks>>>()
        .expect("missing JWKs receiver");
    let validation_options = req
        .app_data::<Data<ValidationOptions>>()
        .expect("missing JWT validation options");

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.split_once("Bearer ").map(|x| x.1))
        .ok_or_else(Error::unauthorized)?;
    let jwt = validate_token(jwks, validation_options, token)?;
    let mut user = User::try_from(jwt)?;
    user.token = token.to_string();
    Ok(user)
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum AuthScope {
    Tenant(String),
    Own(String),
    None,
}

impl AuthScope {
    pub fn is_authorized(&self) -> Result<(), Error> {
        if let Self::None = self {
            Err(Error::unauthorized())
        } else {
            Ok(())
        }
    }

    pub fn authorized_tenant(self) -> Result<String, Error> {
        if let AuthScope::Tenant(t) = self {
            Ok(t)
        } else {
            Err(Error::unauthorized())
        }
    }
}
