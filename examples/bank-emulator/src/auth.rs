#![allow(dead_code)]
use actix_web::{web::Data, FromRequest};
use biscuit::{jwk::JWKSet, Empty, ValidationOptions, JWT};
use enumflags2::{bitflags, BitFlags};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::{collections::HashSet, convert::TryFrom, future::Ready, str::FromStr, time::Duration};
use tokio::sync::watch;
use tracing::error;

use crate::error::Error;

const JWKS_CACHE_DURATION: Duration = Duration::from_secs(15);
const DEFAULT_PERMISSIONS: &[&str] = &["own.*.create|read|update"];

pub type Jwt = JWT<PrivateClaims, Empty>;
pub type Jwks = JWKSet<Empty>;

pub fn empty_jwks() -> Jwks {
    Jwks {
        keys: Vec::default(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct PrivateClaims {
    permissions: Vec<String>,
    scope: String,
}

#[derive(Debug)]
pub struct User {
    pub auth0_id: String,
    pub permissions: Vec<Permission>,
    pub token: String,
}

impl User {
    pub fn authorize(&self, resource: &SmolStr, verb: Verb) -> Result<&Permission, Error> {
        self.permissions
            .iter()
            .find(|p| p.authorize(resource, verb))
            .ok_or_else(Error::unauthorized)
    }

    pub fn query_scope(&self, resource: &SmolStr, verb: Verb) -> AuthScope {
        let mut scopes = self
            .permissions
            .iter()
            .filter_map(|p| p.authorize(resource, verb).then(|| p.scope.clone()));
        match scopes.next() {
            Some(s) if s.as_str() == "own" => AuthScope::Own(self.auth0_id.clone()),
            Some(s) => AuthScope::Tenant(s),
            _ => AuthScope::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Resources {
    All,
    Individual(HashSet<SmolStr>),
}

impl Resources {
    fn authorize(&self, resource: &SmolStr) -> bool {
        match self {
            Self::All => true,
            Self::Individual(r) => r.contains(resource),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Permission {
    pub scope: String,
    pub resources: Resources,
    pub verb: BitFlags<Verb>,
}

impl Permission {
    fn authorize(&self, resource: &SmolStr, verb: Verb) -> bool {
        self.verb.contains(verb) && self.resources.authorize(resource)
    }
}

impl FromStr for Permission {
    type Err = BadPermission;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (scope, permission) = s.split_once('.').ok_or(BadPermission::InvalidFormat)?;
        let (r, v) = permission
            .split_once('.')
            .ok_or(BadPermission::InvalidFormat)?;
        let resources = if r.eq("*") {
            Resources::All
        } else {
            Resources::Individual(r.split_terminator(':').map(SmolStr::from).collect())
        };
        Ok(Self {
            scope: scope.into(),
            resources,
            verb: Verb::parse(v)?,
        })
    }
}

impl Verb {
    fn parse(s: &str) -> Result<BitFlags<Verb>, BadPermission> {
        s.split_terminator('|')
            .try_fold(BitFlags::default(), |acc, verb| {
                let verb = match verb {
                    "create" => Verb::Create,
                    "read" => Verb::Read,
                    "update" => Verb::Update,
                    "delete" => Verb::Delete,
                    _ => return Err(BadPermission::UnknownVerb),
                };
                Ok(acc | verb)
            })
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Verb {
    Create = 0b0000_0001,
    Read = 0b0000_0010,
    Update = 0b0000_0100,
    Delete = 0b0000_1000,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BadPermission {
    InvalidFormat,
    UnknownVerb,
}

impl TryFrom<Jwt> for User {
    type Error = Error;
    fn try_from(jwt: Jwt) -> Result<Self, Error> {
        let (_, claims_set) = jwt.unwrap_decoded();
        let auth0_id = claims_set
            .registered
            .subject
            .ok_or_else(Error::unauthorized)?;
        let user = User {
            permissions: claims_set
                .private
                .permissions
                .iter()
                .map(|s| s.as_str())
                .chain(DEFAULT_PERMISSIONS.iter().copied())
                .filter_map(|p| p.parse().ok())
                .collect(),
            token: String::new(),
            auth0_id,
        };
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
    url.set_path("/.well-known/jwks.json");
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

pub trait AuthModel {
    fn is_authorized(&self, verb: Verb, user: &User) -> Result<(), Error>;

    fn auth_scope(&self, verb: Verb, user: &User) -> AuthScope;
}

#[derive(Eq, PartialEq, Debug)]
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
