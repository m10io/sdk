use clap::{Parser, Subcommand};
use m10_sdk::directory::{directory_service_client::DirectoryServiceClient, Alias, Ledger};
use serde::{Deserialize, Serialize};
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use uuid::Uuid;

use crate::context::Context;
use crate::utils::m10_config_path;
use tonic::service::interceptor::InterceptedService;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) struct CreateDirEntryOptions {
    /// Directory server address
    server: String,
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: CreateDirEntrySubCommands,
}

impl CreateDirEntryOptions {
    pub(super) async fn create(&self) -> anyhow::Result<()> {
        let addr = format!("https://{}", &self.server);
        let channel = Context::channel_from_address(&addr, false)?;
        let access_token = std::fs::read_to_string(m10_config_path().join("access.token"))?;
        let access_token = format!("Bearer {}", access_token.trim());
        let access_token = MetadataValue::from_str(access_token.as_str())?;
        let mut client =
            DirectoryServiceClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
                req.metadata_mut()
                    .insert("authorization", access_token.clone());
                Ok(req)
            });

        match &self.cmd {
            CreateDirEntrySubCommands::Ledger(options) => options.create(&mut client).await,
            CreateDirEntrySubCommands::Alias(options) => options.create(&mut client).await,
            CreateDirEntrySubCommands::ObjectUrl => Self::create_object_url(&mut client).await,
        }
    }

    async fn create_object_url<
        F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
    >(
        client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
    ) -> anyhow::Result<()> {
        let response = client.create_object_url(()).await?.into_inner();
        println!("{:#?}", response);
        Ok(())
    }
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) enum CreateDirEntrySubCommands {
    /// Create a new ledger entry in directory
    Ledger(CreateLedgerOptions),
    /// Create a new alias entry in directory
    Alias(CreateAliasOptions),
    /// Create an S3 object url
    ObjectUrl,
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateLedgerOptions {
    /// Operator id
    #[clap(short, long)]
    operator: String,
    /// Ledger url
    #[clap(short, long)]
    url: String,
}

impl CreateLedgerOptions {
    pub async fn create<
        F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
    >(
        &self,
        client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
    ) -> anyhow::Result<()> {
        let Self { operator, url } = self.to_owned();
        let request = Ledger { operator, url };
        client.create_ledger(request).await?;
        Ok(())
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct CreateAliasOptions {
    /// Alias handle (unique)
    #[clap(short = 'a', long)]
    handle: String,
    /// Display name
    #[clap(short, long)]
    display_name: String,
    /// Set alias type (handle, phone, email)
    #[clap(short = 't', long)]
    alias_type: String,
    /// Set 'account set' id the alias is linked to
    #[clap(short = 's', long)]
    account_set_id: Uuid,
    /// Operator id
    #[clap(short, long)]
    operator: String,
}

impl CreateAliasOptions {
    pub async fn create<
        F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
    >(
        &self,
        client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
    ) -> anyhow::Result<()> {
        let Self {
            handle,
            display_name,
            alias_type,
            account_set_id,
            operator,
        } = self.to_owned();
        let request = Alias {
            handle,
            display_name,
            alias_type: alias_type.parse()?,
            account_set_id: account_set_id.as_bytes().to_vec(),
            operator,
        };
        client.create_alias(request).await?;
        Ok(())
    }
}
