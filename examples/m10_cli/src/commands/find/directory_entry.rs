use std::convert::TryFrom;

use clap::Subcommand;
use m10_sdk::directory::{
    alias::Type as AliasType, directory_service_client::DirectoryServiceClient, Alias,
    SearchAliasesRequest,
};
use serde::{Deserialize, Serialize};
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel,
};
use uuid::Uuid;

use crate::{context::Context, utils::m10_config_path};

#[derive(Clone, Subcommand, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DirEntry {
    //// Find alias entries
    #[command(alias = "a")]
    Alias {
        /// By alias handle
        #[arg(short = 'a', long)]
        handle: String,
        #[arg(long)]
        /// By subject id
        #[arg(long)]
        subject: Option<String>,
        /// Next page token as returned by previous query
        #[arg(long, aliases = ["token", "pt"])]
        page_token: Option<String>,
    },
    /// Find ledger entries
    #[command(alias = "l")]
    Ledger,
}

impl DirEntry {
    pub(super) async fn find(self, context: &Context) -> anyhow::Result<()> {
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
            DirEntry::Alias {
                handle,
                subject,
                page_token,
            } => find_alias(handle, subject, page_token, &mut client).await?,
            DirEntry::Ledger => {
                let response = client.list_ledgers(()).await?.into_inner();
                for ledger in response.ledgers {
                    println!("{:#?}", ledger);
                }
            }
        }
        Ok(())
    }
}

async fn find_alias<F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>>(
    handle: String,
    subject: Option<String>,
    page_token: Option<String>,
    client: &mut DirectoryServiceClient<InterceptedService<Channel, F>>,
) -> anyhow::Result<()> {
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
        let alias_type = AliasType::try_from(alias_type).unwrap_or_default();
        Ok(PrettyAlias {
            _handle: handle,
            _alias_type: alias_type,
            _account_set_id: Uuid::from_slice(&account_set_id)?,
            _operator: operator,
        })
    }
}
