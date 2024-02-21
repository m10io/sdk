use std::str::FromStr;
use std::time::Duration;

use aws_config::Region;
use aws_credential_types::provider::ProvideCredentials;
use aws_sigv4::{
    http_request::{sign, SignableBody, SignableRequest, SigningSettings},
    sign::v4,
};

use std::time::SystemTime;

use sqlx_core::connection::Connection;
use sqlx_core::postgres::{PgConnectOptions, PgConnection};

pub use bb8;
pub use sqlx_core::error::Error;

#[derive(Debug, Clone)]
pub struct RdsManager {
    url: String,
}

impl RdsManager {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn pool(&self) -> Result<bb8::Pool<Self>, Error> {
        bb8::Pool::builder()
            .max_size(128)
            .idle_timeout(Some(Duration::from_secs(60)))
            .build(self.clone())
            .await
    }
}

#[async_trait::async_trait]
impl bb8::ManageConnection for RdsManager {
    type Connection = PgConnection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let url: url::Url = self
            .url
            .parse()
            .map_err(|err: url::ParseError| Error::Configuration(err.into()))?;
        let host = url
            .host_str()
            .ok_or_else(|| Error::Configuration("invalid host".into()))?;
        let host = percent_encoding::percent_decode_str(host)
            .decode_utf8()
            .map_err(|err| Error::Configuration(err.into()))?;
        let port = "5432";
        let username = percent_encoding::percent_decode_str(url.username())
            .decode_utf8()
            .map_err(|err| Error::Configuration(err.into()))?;
        if host.ends_with("rds.amazonaws.com") {
            // If RDS, exchange IAM credentials for auth token
            let region = host.split('.').nth_back(3).expect("region").to_string();

            let auth_token = generate_rds_iam_token(Region::new(region), &host, port, &username)
                .await
                .map_err(|_| Error::Configuration("unable to sign".into()))?;

            let connect_options = PgConnectOptions::from_str(&self.url)?.password(&auth_token);
            PgConnection::connect_with(&connect_options).await
        } else {
            PgConnection::connect(&self.url).await
        }
    }

    async fn is_valid(
        &self,
        conn: &mut bb8::PooledConnection<'_, Self>,
    ) -> Result<(), Self::Error> {
        conn.ping().await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

async fn generate_rds_iam_token(
    region: Region,
    db_hostname: &str,
    port: &str,
    db_username: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = aws_config::from_env().region(region).load().await;

    let credentials = config
        .credentials_provider()
        .expect("no credentials provider found")
        .provide_credentials()
        .await
        .expect("unable to load credentials");
    let identity = credentials.into();
    let region = config.region().unwrap().to_string();

    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(Duration::from_secs(15 * 60));
    signing_settings.signature_location = aws_sigv4::http_request::SignatureLocation::QueryParams;

    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(&region)
        .name("rds-db")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()?;

    let url = format!(
        "https://{db_hostname}:{port}/?Action=connect&DBUser={db_user}",
        db_hostname = db_hostname,
        port = port,
        db_user = db_username
    );

    let signable_request =
        SignableRequest::new("GET", &url, std::iter::empty(), SignableBody::Bytes(&[]))
            .expect("signable request");

    let (signing_instructions, _signature) =
        sign(signable_request, &signing_params.into())?.into_parts();

    let mut url = url::Url::parse(&url).unwrap();
    for (name, value) in signing_instructions.params() {
        url.query_pairs_mut().append_pair(name, value);
    }

    let response = url.to_string().split_off("https://".len());

    Ok(response)
}
