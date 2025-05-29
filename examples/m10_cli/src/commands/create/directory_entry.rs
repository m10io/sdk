use clap::Subcommand;
use m10_sdk::directory::{
    alias::Type, directory_service_client::DirectoryServiceClient, Alias, Ledger,
};
use serde::{Deserialize, Serialize};
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel,
};
use uuid::Uuid;

use crate::context::Context;
use crate::utils::m10_config_path;

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CreateDirEntry {
    /// Create a new alias entry in directory
    #[command(alias = "a")]
    Alias {
        /// Alias handle (unique)
        #[arg(short = 'a', long)]
        handle: String,
        /// Display name
        #[arg(short, long)]
        display_name: String,
        /// Set alias type (handle, phone, email)
        #[arg(short = 't', long)]
        alias_type: String,
        /// Set 'account set' id the alias is linked to
        #[arg(short = 'i', long)]
        account_set_id: Uuid,
        /// Operator id
        #[arg(short, long)]
        operator: String,
    },
    /// Create a new ledger entry in directory
    #[cfg_attr(feature = "customers", command(alias = "l", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "l"))]
    Ledger {
        /// Operator id
        #[arg(short, long)]
        operator: String,
        /// Ledger url
        #[arg(short, long)]
        url: String,
    },
    /// Create an S3 object url
    #[command(alias = "o")]
    ObjectUrl,
}

impl CreateDirEntry {
    pub(super) async fn create(self, context: &Context) -> anyhow::Result<()> {
        let channel = context.channel()?;
        let access_token = std::fs::read_to_string(m10_config_path().join("access.token"))?;
        let access_token = format!("Bearer {}", access_token.trim());
        let access_token = MetadataValue::try_from(access_token.as_str())?;
        let mut client =
            DirectoryServiceClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
                req.metadata_mut()
                    .insert("authorization", access_token.clone());
                Ok(req)
            });

        match self {
            CreateDirEntry::Ledger { operator, url } => {
                create_ledger_entry(operator, url, &mut client).await
            }
            CreateDirEntry::Alias {
                handle,
                display_name,
                alias_type,
                account_set_id,
                operator,
            } => {
                create_alias(
                    handle,
                    display_name,
                    alias_type,
                    account_set_id,
                    operator,
                    &mut client,
                )
                .await
            }
            CreateDirEntry::ObjectUrl => Self::create_object_url(&mut client).await,
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

pub async fn create_ledger_entry<
    F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
>(
    operator: String,
    url: String,
    client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
) -> anyhow::Result<()> {
    let request = Ledger { operator, url };
    client.create_ledger(request).await?;
    Ok(())
}

pub async fn create_alias<
    F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
>(
    handle: String,
    display_name: String,
    alias_type: String,
    account_set_id: Uuid,
    operator: String,
    client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
) -> anyhow::Result<()> {
    let alias_enum = Type::from_str_name(&alias_type.to_uppercase())
        .ok_or_else(|| anyhow::anyhow!("Invalid alias type"))?;

    let request = Alias {
        handle,
        display_name,
        alias_type: alias_enum as i32,
        account_set_id: account_set_id.as_bytes().to_vec(),
        operator,
    };
    client.create_alias(request).await?;
    Ok(())
}
