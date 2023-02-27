use crate::utils::{key_pair_from_env, load_key_pair};
use m10_sdk::ImageClient;
use m10_sdk::KeyPair;
use m10_sdk::M10Client;
use tonic::transport::{Channel, ClientTlsConfig, Uri};

pub(crate) struct Context {
    pub(crate) image_client: ImageClient,
    pub(crate) m10_client: M10Client<KeyPair>,
}

impl Context {
    pub(crate) fn channel_from_address(
        address: &str,
        no_tls: bool,
    ) -> Result<Channel, anyhow::Error> {
        let endpoint: Uri = address.parse()?;
        let is_local = matches!(endpoint.host(), Some("localhost") | Some("127.0.0.1"));

        let builder = Channel::builder(endpoint)
            .keep_alive_while_idle(true)
            .http2_keep_alive_interval(std::time::Duration::from_secs(30));
        let builder = if !(no_tls || is_local) {
            let tls_config = ClientTlsConfig::new();
            builder.tls_config(tls_config)?
        } else {
            builder
        };
        let channel = builder.connect_lazy()?;
        Ok(channel)
    }

    pub(crate) fn new(config: &crate::Config) -> anyhow::Result<Self> {
        let admin = if let Some(key_file) = &config.key_file {
            load_key_pair(key_file)
        } else if let Some(k) = &config.signing_key {
            key_pair_from_env(k)
        } else {
            Err(anyhow::anyhow!("no key giving"))
        }?;
        let addr = if config.no_tls {
            format!("http://{}", config.server)
        } else {
            format!("https://{}", config.server)
        };
        let channel = Self::channel_from_address(&addr, config.no_tls)?;
        let image_client = ImageClient::new(addr);
        let m10_client = M10Client::new(admin, channel);

        Ok(Self {
            image_client,
            m10_client,
        })
    }
}
