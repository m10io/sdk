#![allow(clippy::unnecessary_fallible_conversions)]
use crate::config;
use crate::dyn_signer::{DynSigner, DynSignerWrapper};
use crate::utils::m10_config_path;
use m10_protos::sdk::signature::Algorithm;
use m10_sdk::block_explorer::BlockExplorerClient;
use m10_sdk::{
    directory::directory_service_client::DirectoryServiceClient, Ed25519, GrpcClient, HttpClient,
    ImageClient, KeyPair, M10CoreClient, VaultTransit,
};
use std::{cell::OnceCell, fs::File, io::Read, str::FromStr, sync::Arc};
use tonic::transport::{Channel, Endpoint, Uri};

type ContextData<'a> = (&'a str, &'a str, &'a str, Option<String>, Option<String>);

pub(crate) struct Context {
    context_id: Vec<u8>,
    endpoint: Option<Endpoint>,
    ws_endpoint: Option<Endpoint>,
    signer: Option<DynSignerWrapper>,
    raw_key: Option<Vec<u8>>,
    ledger_client: OnceCell<Box<dyn M10CoreClient<Signer = DynSignerWrapper> + Send + Sync>>,
    http: bool,
    provider: Provider,
    ca_cert: Option<String>,
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

        let config = config::Config::new().map_err(|err| anyhow::anyhow!(err))?;

        let (signer, raw_key, provider) = init_signer(options, &config).await?;
        let signer = signer.map(DynSignerWrapper::new);

        let ca_cert = options.ca_cert.clone().or_else(|| config.ca_cert.clone());

        let endpoint = build_endpoint(options, &config, ca_cert.as_deref())?;
        let ws_endpoint = build_ws_endpoint(options, &config, ca_cert.as_deref())?;

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
            ca_cert,
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

    pub(crate) fn signer(&self) -> anyhow::Result<&DynSignerWrapper> {
        self.signer
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("signer missing"))
    }

    pub(crate) fn raw_key(&self) -> anyhow::Result<Vec<u8>> {
        self.raw_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("raw key missing"))
    }

    #[allow(clippy::borrowed_box)]
    pub(crate) fn ledger_client(
        &self,
    ) -> &Box<dyn M10CoreClient<Signer = DynSignerWrapper> + Send + Sync> {
        self.ledger_client.get_or_init(|| {
            if self.http {
                let ws_endpoint = self.ws_endpoint.clone().unwrap_or_else(|| {
                    eprintln!("error: server ws addr missing");
                    std::process::exit(1);
                });
                let endpoint = self.endpoint.clone().unwrap_or_else(|| {
                    eprintln!("error: server addr missing");
                    std::process::exit(1);
                });
                let signer = Some(Arc::new(self.signer.clone().unwrap_or_else(|| {
                    eprintln!("error: signer missing");
                    std::process::exit(1);
                })));

                if let Some(ca_cert_path) = &self.ca_cert {
                    let pem = std::fs::read(ca_cert_path).unwrap_or_else(|e| {
                        eprintln!("error: failed to read CA cert file: {e}");
                        std::process::exit(1);
                    });
                    Box::new(
                        HttpClient::new_with_ca_cert(endpoint, ws_endpoint, signer, &pem)
                            .unwrap_or_else(|e| {
                                eprintln!("error: failed to create HTTP client with CA cert: {e}");
                                std::process::exit(1);
                            }),
                    )
                } else {
                    Box::new(HttpClient::new(endpoint, ws_endpoint, signer))
                }
            } else {
                let access_token =
                    std::fs::read_to_string(m10_config_path().join("access.token")).ok();
                Box::new(
                    GrpcClient::new_with_access_token(
                        self.endpoint.clone().unwrap_or_else(|| {
                            eprintln!("error: server addr missing");
                            std::process::exit(1);
                        }),
                        Some(Arc::new(self.signer.clone().unwrap_or_else(|| {
                            eprintln!("error: signer missing");
                            std::process::exit(1);
                        }))),
                        access_token.as_deref(),
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("error: failed to connect to ledger: {}", e.get_message());
                        std::process::exit(1);
                    }),
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

    pub(crate) fn ca_cert(&self) -> Option<&str> {
        self.ca_cert.as_deref()
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
    config: &config::Config,
) -> anyhow::Result<(Option<Arc<dyn DynSigner>>, Option<Vec<u8>>, Provider)> {
    if let Some(key_file) = &options.key_file {
        let key_str = load_key(key_file)?;
        let raw_key = base64::decode(&key_str)?;
        let kp = match options.key_algorithm.as_deref() {
            Some("ed25519ph") => match KeyPair::from_str(&key_str)? {
                KeyPair::P256(_) => {
                    return Err(anyhow::anyhow!(
                        "P256 key is incompatible with --key-algorithm ed25519ph"
                    ))
                }
                KeyPair::Ed25519(_) => KeyPair::Ed25519(Ed25519::from_pkcs8_ph(&raw_key)?),
            },
            Some(other) => {
                return Err(anyhow::anyhow!(
                    "invalid value '{}' for --key-algorithm: only 'ed25519ph' is supported",
                    other
                ))
            }
            None => KeyPair::from_str(&key_str)?,
        };
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
            "ed25519ph" => Algorithm::Ed25519PhSha512,
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
        .map_err(|e| anyhow::anyhow!("failed to initialize vault signer: {}", e))?;
        return Ok((
            Some(Arc::new(vt) as Arc<dyn DynSigner>),
            None,
            Provider::Vault,
        ));
    }

    let legacy_key = options
        .profile
        .as_ref()
        .and_then(|profile_name| config.profile.get(profile_name).map(|p| p.key.clone()))
        .or_else(|| config.key.clone())
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

    Ok((None, None, Provider::KeyFile))
}

fn get_vault_params<'a>(
    options: &'a super::Opts,
    config: &'a config::Config,
) -> Option<ContextData<'a>> {
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
    } else {
        if let Some(profile_name) = &options.profile {
            if let Some(profile) = config.profile.get(profile_name) {
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
        if config.vault_addr.is_some()
            && config.vault_token.is_some()
            && config.vault_key_name.is_some()
        {
            return Some((
                config.vault_addr.as_deref().unwrap(),
                config.vault_token.as_deref().unwrap(),
                config.vault_key_name.as_deref().unwrap(),
                config.vault_mount.clone(),
                config.vault_namespace.clone(),
            ));
        }
        None
    }
}

fn get_addr(options: &super::Opts, config: &config::Config) -> Option<String> {
    options.server.clone().or_else(|| {
        options
            .profile
            .as_ref()
            .and_then(|profile_name| {
                config
                    .profile
                    .get(profile_name)
                    .and_then(|profile| profile.addr.clone())
            })
            .or_else(|| config.addr.clone())
            .or_else(|| std::env::var("M10_APP").ok())
    })
}

fn build_endpoint(
    options: &super::Opts,
    config: &config::Config,
    ca_cert: Option<&str>,
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
            let tls_config = crate::tls::build_tonic_tls_config(ca_cert)?;
            endpoint = endpoint.tls_config(tls_config)?;
        }
        Ok(Some(endpoint))
    } else {
        Ok(None)
    }
}

fn build_ws_endpoint(
    options: &super::Opts,
    config: &config::Config,
    ca_cert: Option<&str>,
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
            let tls_config = crate::tls::build_tonic_tls_config(ca_cert)?;
            endpoint = endpoint.tls_config(tls_config)?;
        }
        Ok(Some(endpoint))
    } else {
        Ok(None)
    }
}

fn load_key(path: &str) -> anyhow::Result<String> {
    let mut key_file = File::open(path).map_err(|_| anyhow::anyhow!("file not found: {}", path))?;
    let mut pkcs8_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut pkcs8_bytes)?;
    Ok(base64::encode(&pkcs8_bytes))
}
