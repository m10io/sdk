use clap::{Parser, Subcommand};
use m10_sdk::directory::{
    alias::Type as AliasType, directory_service_client::DirectoryServiceClient, Alias,
    SearchAliasesRequest,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use uuid::Uuid;

use crate::context::Context;
use crate::utils::m10_config_path;
use tonic::service::interceptor::InterceptedService;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) struct FindDirEntryOptions {
    /// Directory server address
    server: String,
    #[clap(subcommand)]
    #[serde(flatten)]
    cmd: FindDirEntrySubCommands,
}

impl FindDirEntryOptions {
    pub(super) async fn find(&self) -> anyhow::Result<()> {
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
            FindDirEntrySubCommands::Ledger(options) => options.find(&mut client).await,
            FindDirEntrySubCommands::Alias(options) => options.find(&mut client).await,
        }
    }
}

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[clap(about)]
pub(crate) enum FindDirEntrySubCommands {
    /// Find ledger entries
    Ledger(FindLedgerOptions),
    //// Find alias entries
    Alias(FindAliasOptions),
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct FindLedgerOptions {}

impl FindLedgerOptions {
    pub async fn find<F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>>(
        &self,
        client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
    ) -> anyhow::Result<()> {
        let Self {} = self.to_owned();
        let response = client.list_ledgers(()).await?.into_inner();
        for ledger in response.ledgers {
            println!("{:#?}", ledger);
        }
        Ok(())
    }
}

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
#[clap(about)]
pub(crate) struct FindAliasOptions {
    /// By alias handle
    #[clap(short = 'a', long)]
    handle: String,
    #[clap(short, long)]
    /// By subject id
    subject: Option<String>,
    /// Next page token as returned by previous query
    #[clap(short, long)]
    page_token: Option<String>,
}

impl FindAliasOptions {
    pub async fn find<F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>>(
        &self,
        client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
    ) -> anyhow::Result<()> {
        let Self {
            handle,
            subject,
            page_token,
        } = self.to_owned();
        let msg = SearchAliasesRequest {
            handle_prefix: handle,
            subject: subject.unwrap_or_default(),
            page_token: page_token.unwrap_or_default(),
            page_size: 20,
        };
        let response = client.search_aliases(msg).await?.into_inner();
        for alias in response.aliases {
            println!("{:#?}", PrettyAlias::try_from(alias)?);
        }
        println!("next: {}", response.next_page_token);
        Ok(())
    }
}

#[derive(Debug)]
struct PrettyAlias {
    _handle: String,
    _alias_type: AliasType,
    _account_set_id: Uuid,
    _operator: String,
}

impl TryFrom<Alias> for PrettyAlias {
    type Error = anyhow::Error;

    fn try_from(other: Alias) -> Result<PrettyAlias, Self::Error> {
        let Alias {
            handle,
            alias_type,
            account_set_id,
            operator,
            ..
        } = other;
        let alias_type = AliasType::from_i32(alias_type).unwrap_or_default();
        Ok(PrettyAlias {
            _handle: handle,
            _alias_type: alias_type,
            _account_set_id: Uuid::from_slice(&account_set_id)?,
            _operator: operator,
        })
    }
}
