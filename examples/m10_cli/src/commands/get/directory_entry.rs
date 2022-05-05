use crate::context::Context;
use clap::{Parser, Subcommand};
use m10_sdk::directory::{directory_service_client::DirectoryServiceClient, GetObjectUrlRequest};
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) struct GetDirEntryOptions {
    /// Directory server address
    server: String,
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: GetDirEntrySubCommands,
}

impl GetDirEntryOptions {
    pub(super) async fn get(&self) -> anyhow::Result<()> {
        let addr = format!("https://{}", &self.server);
        let channel = Context::channel_from_address(&addr, false)?;
        let mut client = DirectoryServiceClient::new(channel);

        match &self.cmd {
            GetDirEntrySubCommands::ObjectUrl(options) => options.get_object_url(&mut client).await,
        }
    }
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum GetDirEntrySubCommands {
    ObjectUrl(GetObjectUrlOptions),
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct GetObjectUrlOptions {
    /// Object id
    #[clap(short, long)]
    object_id: String,
}

impl GetObjectUrlOptions {
    async fn get_object_url(
        &self,
        client: &mut DirectoryServiceClient<Channel>,
    ) -> anyhow::Result<()> {
        let request = GetObjectUrlRequest {
            object_id: self.object_id.clone(),
        };
        let response = client.get_object_url(request).await?.into_inner();
        println!("{:#?}", response);
        Ok(())
    }
}
