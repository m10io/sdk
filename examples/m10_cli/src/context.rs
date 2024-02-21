#![allow(clippy::unnecessary_fallible_conversions)]
use m10_sdk::{
    directory::directory_service_client::DirectoryServiceClient, GrpcClient, HttpClient,
    ImageClient, KeyPair, M10CoreClient,
};
use std::{cell::OnceCell, fs::File, io::Read, str::FromStr, sync::Arc};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint, Uri};

use crate::config;

pub(crate) struct Context {
    context_id: Vec<u8>,
    endpoint: Option<Endpoint>,
    ws_endpoint: Option<Endpoint>,
    signing_key: Option<Arc<KeyPair>>,
    raw_key: Option<Vec<u8>>,
    ledger_client: OnceCell<Box<dyn M10CoreClient<Signer = KeyPair> + Send + Sync>>,
    http: bool,
}

impl Context {
    pub(crate) fn new_from_options(options: &super::Opts) -> anyhow::Result<Self> {
        let config = config::Config::new()
            .map_err(|err| {
                println!("{err:?}");
                err
            })
            .ok();

        let raw_key = options
            .key_file
            .as_ref()
            .map(|p| load_key(p))
            .transpose()?
            .or_else(|| {
                config
                    .as_ref()
                    .and_then(|c| {
                        options
                            .profile
                            .as_ref()
                            .and_then(|p| c.profile.get(p).map(|p| p.key.clone()))
                            .or_else(|| c.key.clone())
                    })
                    .or_else(|| std::env::var("M10_SIGNING_KEY").ok())
            });

        let signing_key = raw_key
            .as_ref()
            .map(|k| Ok::<_, anyhow::Error>(Arc::new(KeyPair::from_str(k)?)))
            .transpose()?;

        let endpoint = options
            .server
            .clone()
            .or_else(|| {
                config
                    .as_ref()
                    .and_then(|c| {
                        options
                            .profile
                            .as_ref()
                            .and_then(|p| c.profile.get(p).and_then(|p| p.addr.clone()))
                            .or_else(|| c.addr.clone())
                    })
                    .or_else(|| std::env::var("M10_APP").ok())
            })
            .map(|a| {
                let uri = hyper::http::uri::Builder::new()
                    .scheme(if options.no_tls { "http" } else { "https" })
                    .authority(a.as_str())
                    .path_and_query("/")
                    .build()?;
                Ok::<_, anyhow::Error>(
                    if !options.no_tls {
                        let tls_config = ClientTlsConfig::new();
                        Endpoint::from_str(uri.to_string().as_str())?.tls_config(tls_config)?
                    } else {
                        Endpoint::from_str(uri.to_string().as_str())?
                    }
                    .keep_alive_while_idle(true)
                    .http2_keep_alive_interval(std::time::Duration::from_secs(30)),
                )
            })
            .transpose()?;

        let ws_endpoint = options
            .server
            .clone()
            .or_else(|| {
                config
                    .as_ref()
                    .and_then(|c| {
                        options
                            .profile
                            .as_ref()
                            .and_then(|p| c.profile.get(p).and_then(|p| p.addr.clone()))
                            .or_else(|| c.addr.clone())
                    })
                    .or_else(|| std::env::var("M10_APP").ok())
            })
            .map(|a| {
                let uri = hyper::http::uri::Builder::new()
                    .scheme(if options.no_tls { "ws" } else { "wss" })
                    .authority(a.as_str())
                    .path_and_query("/")
                    .build()?;
                Ok::<_, anyhow::Error>(
                    if !options.no_tls {
                        let tls_config = ClientTlsConfig::new();
                        Endpoint::from_str(uri.to_string().as_str())?.tls_config(tls_config)?
                    } else {
                        Endpoint::from_str(uri.to_string().as_str())?
                    }
                    .keep_alive_while_idle(true)
                    .http2_keep_alive_interval(std::time::Duration::from_secs(30)),
                )
            })
            .transpose()?;

        let context_id = options
            .context_id
            .as_ref()
            .map(|i| i.0.clone())
            .unwrap_or_default();

        Ok(Self {
            context_id,
            endpoint,
            ws_endpoint,
            signing_key,
            raw_key: raw_key.as_ref().map(base64::decode).transpose()?,
            ledger_client: OnceCell::new(),
            http: options.http,
        })
    }

    pub(crate) fn context_id(&self) -> Vec<u8> {
        self.context_id.clone()
    }

    pub(crate) fn channel(&self) -> anyhow::Result<Channel> {
        self.endpoint
            .clone()
            .map(|ep| ep.connect_lazy())
            .transpose()?
            .ok_or_else(|| anyhow::anyhow!("server addr missing"))
    }

    pub(crate) fn addr(&self) -> anyhow::Result<Uri> {
        self.endpoint
            .clone()
            .map(|ep| ep.uri().clone())
            .ok_or_else(|| anyhow::anyhow!("server addr missing"))
    }

    #[allow(dead_code)]
    pub(crate) fn ws_addr(&self) -> anyhow::Result<Uri> {
        self.ws_endpoint
            .clone()
            .map(|ep| ep.uri().clone())
            .ok_or_else(|| anyhow::anyhow!("server ws addr missing"))
    }

    pub(crate) fn signing_key(&self) -> anyhow::Result<&KeyPair> {
        Ok(self
            .signing_key
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("signing key missing"))?)
    }

    pub(crate) fn raw_key(&self) -> anyhow::Result<Vec<u8>> {
        self.raw_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("signing key missing"))
    }

    #[allow(clippy::borrowed_box)]
    pub(crate) fn ledger_client(&self) -> &Box<dyn M10CoreClient<Signer = KeyPair> + Send + Sync> {
        self.ledger_client.get_or_init(|| {
            let endpoint = self.endpoint.as_ref().expect("endpoint").clone();
            if self.http {
                let ws_endpoint = self.ws_endpoint.as_ref().expect("ws_endpoint").clone();
                Box::new(HttpClient::new(
                    endpoint,
                    ws_endpoint,
                    self.signing_key.clone(),
                ))
            } else {
                Box::new(GrpcClient::new(endpoint, self.signing_key.clone()).expect("grpc client"))
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
}

fn load_key(path: &str) -> anyhow::Result<String> {
    let mut key_file = File::open(path)?;
    let mut pkcs8_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut pkcs8_bytes)?;
    Ok(base64::encode(&pkcs8_bytes))
}
