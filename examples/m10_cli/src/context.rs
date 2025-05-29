#![allow(clippy::unnecessary_fallible_conversions)]
use crate::config;
use crate::dyn_signer::{DynSigner, DynSignerWrapper};
use crate::utils::m10_config_path;
use m10_protos::sdk::signature::Algorithm;
use m10_sdk::block_explorer::BlockExplorerClient;
use m10_sdk::{
    directory::directory_service_client::DirectoryServiceClient, GrpcClient, HttpClient,
    ImageClient, KeyPair, M10CoreClient, VaultTransit,
};
use std::{cell::OnceCell, fs::File, io::Read, str::FromStr, sync::Arc};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint, Uri};

pub(crate) struct Context {
    context_id: Vec<u8>,
    endpoint: Option<Endpoint>,
    ws_endpoint: Option<Endpoint>,
    signer: Option<DynSignerWrapper>,
    raw_key: Option<Vec<u8>>,
    ledger_client: OnceCell<Box<dyn M10CoreClient<Signer = DynSignerWrapper> + Send + Sync>>,
    http: bool,
    provider: Provider,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Provider {
    Vault,
    KeyFile,
}

impl Context {
    pub(crate) async fn new_from_options(options: &super::Opts) -> anyhow::Result<Self> {
        if let Some(level) = options.verbose {
            if std::env::var("RUST_LOG").is_err() {
                std::env::set_var("RUST_LOG", level.to_string().to_lowercase());
            }
            env_logger::try_init()?;
        }

        let config = config::Config::new()
            .map_err(|err| {
                println!("{err:?}");
                err
            })
            .ok();

        let (signer, raw_key, provider) = init_signer(options, config.as_ref()).await?;
        let signer = signer.map(DynSignerWrapper::new);

        let endpoint = build_endpoint(options, config.as_ref())?;
        let ws_endpoint = build_ws_endpoint(options, config.as_ref())?;

        let context_id = options
            .context_id
            .as_ref()
            .map(|hex_str| {
                hex::decode(hex_str)
                    .map_err(|e| anyhow::anyhow!("Invalid hex string for context_id: {}", e))
            })
            .unwrap_or_else(|| Ok(Vec::new()))?;

        Ok(Self {
            context_id,
            endpoint,
            ws_endpoint,
            signer,
            raw_key,
            ledger_client: OnceCell::new(),
            http: options.http,
            provider,
        })
    }

    pub(crate) fn context_id(&self) -> Vec<u8> {
        self.context_id.clone()
    }

    pub(crate) fn channel(&self) -> anyhow::Result<Channel> {
        self.endpoint
            .clone()
            .map(|ep| ep.connect_lazy())
            .ok_or_else(|| anyhow::anyhow!("server addr missing"))
    }

    pub(crate) fn addr(&self) -> anyhow::Result<Uri> {
        self.endpoint
            .as_ref()
            .map(|ep| ep.uri().clone())
            .ok_or_else(|| anyhow::anyhow!("server addr missing"))
    }

    #[allow(dead_code)]
    pub(crate) fn ws_addr(&self) -> anyhow::Result<Uri> {
        self.ws_endpoint
            .as_ref()
            .map(|ep| ep.uri().clone())
            .ok_or_else(|| anyhow::anyhow!("server ws addr missing"))
    }

    pub(crate) fn signer(&self) -> &DynSignerWrapper {
        self.signer.as_ref().expect("signer missing")
    }

    pub(crate) fn raw_key(&self) -> anyhow::Result<Vec<u8>> {
        self.raw_key
            .as_ref()
            .map(|key| key.clone())
            .ok_or_else(|| anyhow::anyhow!("raw key missing"))
    }

    #[allow(clippy::borrowed_box)]
    pub(crate) fn ledger_client(
        &self,
    ) -> &Box<dyn M10CoreClient<Signer = DynSignerWrapper> + Send + Sync> {
        self.ledger_client.get_or_init(|| {
            if self.http {
                let ws_endpoint = self.ws_endpoint.clone().expect("server ws addr missing");
                Box::new(HttpClient::new(
                    self.endpoint.clone().expect("server addr missing"),
                    ws_endpoint,
                    Some(Arc::new(self.signer.clone().expect("signer missing"))),
                ))
            } else {
                let access_token =
                    std::fs::read_to_string(m10_config_path().join("access.token")).ok();
                Box::new(
                    GrpcClient::new_with_access_token(
                        self.endpoint.clone().expect("server addr missing"),
                        Some(Arc::new(self.signer.clone().expect("signer missing"))),
                        access_token.as_deref(),
                    )
                    .expect("grpc client"),
                )
            }
        })
    }

    pub(crate) fn directory_client(&self) -> anyhow::Result<DirectoryServiceClient<Channel>> {
        let channel = self.channel()?;
        Ok(DirectoryServiceClient::new(channel))
    }

    pub(crate) fn image_client(&self) -> anyhow::Result<ImageClient> {
        let addr = self.addr()?;
        Ok(ImageClient::new(addr.to_string()))
    }

    pub(crate) fn provider(&self) -> Provider {
        self.provider
    }

    pub(crate) fn block_explorer_client(&self) -> anyhow::Result<BlockExplorerClient> {
        let addr = self.addr()?;
        Ok(BlockExplorerClient::new(addr.to_string()))
    }
}

async fn init_signer(
    options: &super::Opts,
    config: Option<&config::Config>,
) -> anyhow::Result<(Option<Arc<dyn DynSigner>>, Option<Vec<u8>>, Provider)> {
    if let Some(key_file) = &options.key_file {
        let key_str = load_key(key_file)?;
        let raw_key = base64::decode(&key_str)?;
        let kp = KeyPair::from_str(&key_str)?;
        return Ok((
            Some(Arc::new(kp) as Arc<dyn DynSigner>),
            Some(raw_key),
            Provider::KeyFile,
        ));
    }

    if let Some((vault_addr, vault_token, vault_key_name, vault_mount, vault_namespace)) =
        get_vault_params(options, config)
    {
        let algorithm = match options
            .vault_algorithm
            .as_deref()
            .unwrap_or("ed25519")
            .to_lowercase()
            .as_str()
        {
            "ed25519" => Algorithm::Ed25519,
            "p256" => Algorithm::P256Sha256Asn1,
            other => return Err(anyhow::anyhow!("unsupported vault algorithm: {}", other)),
        };
        let vt = VaultTransit::new(
            vault_addr,
            vault_token,
            vault_key_name.to_string(),
            vault_mount,
            Some(algorithm),
            vault_namespace,
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to initialize vault signer: {:?}", e))?;
        return Ok((
            Some(Arc::new(vt) as Arc<dyn DynSigner>),
            None,
            Provider::Vault,
        ));
    }

    if let Some(cfg) = config {
        let legacy_key = options
            .profile
            .as_ref()
            .and_then(|profile_name| cfg.profile.get(profile_name).map(|p| p.key.clone()))
            .or_else(|| cfg.key.clone())
            .or_else(|| std::env::var("M10_SIGNING_KEY").ok());
        if let Some(key) = legacy_key {
            let raw_key = base64::decode(&key)?;
            let kp = KeyPair::from_str(&key)?;
            return Ok((
                Some(Arc::new(kp) as Arc<dyn DynSigner>),
                Some(raw_key),
                Provider::KeyFile,
            ));
        }
    }

    Ok((None, None, Provider::KeyFile))
}

fn get_vault_params<'a>(
    options: &'a super::Opts,
    config: Option<&'a config::Config>,
) -> Option<(&'a str, &'a str, &'a str, Option<String>, Option<String>)> {
    if options.vault_addr.is_some()
        && options.vault_token.is_some()
        && options.vault_key_name.is_some()
    {
        Some((
            options.vault_addr.as_deref().unwrap(),
            options.vault_token.as_deref().unwrap(),
            options.vault_key_name.as_deref().unwrap(),
            options.vault_mount.clone(),
            options.vault_namespace.clone(),
        ))
    } else if let Some(cfg) = config {
        if let Some(profile_name) = &options.profile {
            if let Some(profile) = cfg.profile.get(profile_name) {
                if profile.vault_addr.is_some()
                    && profile.vault_token.is_some()
                    && profile.vault_key_name.is_some()
                {
                    return Some((
                        profile.vault_addr.as_deref().unwrap(),
                        profile.vault_token.as_deref().unwrap(),
                        profile.vault_key_name.as_deref().unwrap(),
                        profile.vault_mount.clone(),
                        profile.vault_namespace.clone(),
                    ));
                }
            }
        }
        if cfg.vault_addr.is_some() && cfg.vault_token.is_some() && cfg.vault_key_name.is_some() {
            return Some((
                cfg.vault_addr.as_deref().unwrap(),
                cfg.vault_token.as_deref().unwrap(),
                cfg.vault_key_name.as_deref().unwrap(),
                cfg.vault_mount.clone(),
                cfg.vault_namespace.clone(),
            ));
        }
        None
    } else {
        None
    }
}

fn get_addr(options: &super::Opts, config: Option<&config::Config>) -> Option<String> {
    options.server.clone().or_else(|| {
        config
            .and_then(|cfg| {
                options
                    .profile
                    .as_ref()
                    .and_then(|profile_name| {
                        cfg.profile
                            .get(profile_name)
                            .and_then(|profile| profile.addr.clone())
                    })
                    .or_else(|| cfg.addr.clone())
            })
            .or_else(|| std::env::var("M10_APP").ok())
    })
}

fn build_endpoint(
    options: &super::Opts,
    config: Option<&config::Config>,
) -> anyhow::Result<Option<Endpoint>> {
    if let Some(addr) = get_addr(options, config) {
        let scheme = if options.no_tls { "http" } else { "https" };
        let uri = hyper::http::uri::Builder::new()
            .scheme(scheme)
            .authority(addr.as_str())
            .path_and_query("/")
            .build()?;
        let mut endpoint = Endpoint::from_str(uri.to_string().as_str())?
            .keep_alive_while_idle(true)
            .http2_keep_alive_interval(std::time::Duration::from_secs(30));
        if !options.no_tls {
            let tls_config = ClientTlsConfig::with_native_roots(Default::default());
            endpoint = endpoint.tls_config(tls_config)?;
        }
        Ok(Some(endpoint))
    } else {
        Ok(None)
    }
}

fn build_ws_endpoint(
    options: &super::Opts,
    config: Option<&config::Config>,
) -> anyhow::Result<Option<Endpoint>> {
    if let Some(addr) = get_addr(options, config) {
        let scheme = if options.no_tls { "ws" } else { "wss" };
        let uri = hyper::http::uri::Builder::new()
            .scheme(scheme)
            .authority(addr.as_str())
            .path_and_query("/")
            .build()?;
        let mut endpoint = Endpoint::from_str(uri.to_string().as_str())?
            .keep_alive_while_idle(true)
            .http2_keep_alive_interval(std::time::Duration::from_secs(30));
        if !options.no_tls {
            let tls_config = ClientTlsConfig::new();
            endpoint = endpoint.tls_config(tls_config)?;
        }
        Ok(Some(endpoint))
    } else {
        Ok(None)
    }
}

fn load_key(path: &str) -> anyhow::Result<String> {
    let mut key_file = File::open(path)?;
    let mut pkcs8_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut pkcs8_bytes)?;
    Ok(base64::encode(&pkcs8_bytes))
}
