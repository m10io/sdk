use std::str::FromStr;
use std::time::Duration;

use rusoto_credential::{ChainProvider, ProvideAwsCredentials};
use rusoto_signature::SignedRequest;
use sqlx_core::connection::Connection;
use sqlx_core::error::Error;
use sqlx_core::postgres::{PgConnectOptions, PgConnection};

pub use bb8;

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
        let username = percent_encoding::percent_decode_str(url.username())
            .decode_utf8()
            .map_err(|err| Error::Configuration(err.into()))?;
        if host.ends_with("rds.amazonaws.com") {
            // If RDS, exchange IAM credentials for auth token
            let region = host
                .split('.')
                .nth_back(3)
                .expect("region")
                .parse()
                .unwrap();
            let aws_credentials = ChainProvider::new()
                .credentials()
                .await
                .map_err(|err| Error::Configuration(err.into()))?;
            let mut request = SignedRequest::new("GET", "rds-db", &region, "/");
            request.set_hostname(Some(format!("{}:5432", host)));
            request.add_param("Action", "connect");
            request.add_param("DBUser", &username);
            let expires_in = Duration::from_secs(15 * 60);
            let presigned_url = request.generate_presigned_url(&aws_credentials, &expires_in, true);
            let auth_token = presigned_url.trim_start_matches("https://");
            let connect_options = PgConnectOptions::from_str(&self.url)?.password(auth_token);
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
