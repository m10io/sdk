use clap::Parser;
use m10_sdk::{account::AccountId, prost::Message};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Parser, Debug, Serialize, Deserialize)]
pub(crate) struct CreateTokenOptions {
    #[clap(short, long)]
    account: AccountId,
    #[clap(short, long)]
    value: u64,
}

impl CreateTokenOptions {
    pub(super) async fn create(&self, config: &crate::Config) -> anyhow::Result<()> {
        let context = Context::new(config)?;
        let (tx_id, token) = context
            .m10_client
            .create_token(self.account, self.value, None, vec![])
            .await?;
        println!("txn-id: {tx_id}");
        let token = token.ok_or_else(|| anyhow::anyhow!("no token returned"))?;
        let mut token_buf = vec![];
        token.encode(&mut token_buf)?;
        let path = format!("./ot-{}.tok", hex::encode(tx_id.to_be_bytes().as_slice()));
        std::fs::write(path, token_buf)?;
        println!("{token:?}");
        Ok(())
    }
}
