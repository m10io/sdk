use std::fs;

use clap::Subcommand;
use m10_sdk::{account, account::AccountId, prost::Message, sdk, Signer};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Clone, Debug, Subcommand, Serialize, Deserialize)]
pub(crate) enum Verify {
    #[command(alias = "o")]
    OfflineToken {
        #[arg(short, long)]
        file: String,
    },
    #[command(alias = "r")]
    RedeemableToken {
        #[arg(short, long)]
        master: String,
        #[arg(short, long)]
        file: String,
    },
}

impl Verify {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        match self {
            Verify::OfflineToken { file } => Self::verify_offline(file),
            Verify::RedeemableToken { master, file } => Self::verify_redeemable(master, file),
        }
    }

    fn verify_offline(file: &str) -> anyhow::Result<()> {
        let master_token_file = fs::read(file)?;
        let sdk::OfflineToken { data, signature } =
            sdk::OfflineToken::decode(master_token_file.as_slice())?;
        let data = data.ok_or_else(|| anyhow::anyhow!("missing data in master token"))?;
        let mut token_data_buf = vec![];
        data.encode(&mut token_data_buf)?;
        let signature =
            signature.ok_or_else(|| anyhow::anyhow!("missing signature in master token"))?;
        signature.verify(&token_data_buf)?;
        println!("token ok");
        Ok(())
    }

    fn verify_redeemable(master: &str, file: &str) -> anyhow::Result<()> {
        let master_token_file = fs::read(master)?;
        let sdk::OfflineToken { data, .. } =
            sdk::OfflineToken::decode(master_token_file.as_slice())?;
        let master_token = data.ok_or_else(|| anyhow::anyhow!("missing data in master token"))?;

        let token_file = fs::read(file)?;
        let sdk::RedeemableToken { data, signature } =
            sdk::RedeemableToken::decode(token_file.as_slice())?;
        let token = data.ok_or_else(|| anyhow::anyhow!("missing data in redeemable token"))?;
        let mut token_data_buf = vec![];
        token.encode(&mut token_data_buf)?;
        let signature =
            signature.ok_or_else(|| anyhow::anyhow!("missing signature in redeemable token"))?;
        signature.verify(&token_data_buf)?;

        let mut ok = true;
        if !token.inputs.iter().any(|i| i.input == master_token.id) {
            println!("token not derived from master token");
            ok = false;
        }
        if signature.public_key != master_token.address {
            println!("token not issues by owner of master token");
            ok = false;
        }
        if ok {
            println!("token ok");
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Subcommand, Serialize, Deserialize)]
pub(crate) enum Issue {
    #[command(alias = "t")]
    Token {
        #[arg(short, long)]
        from: Vec<String>,
        #[arg(short, long)]
        values: Vec<u64>,
        #[arg(short, long)]
        to: String,
    },
}

impl Issue {
    pub(crate) async fn run(&self, context: &Context) -> anyhow::Result<()> {
        match self {
            Issue::Token { from, values, to } => Self::issue(from, values, to, context).await,
        }
    }

    async fn issue(
        files: &[String],
        values: &[u64],
        to: &str,
        context: &Context,
    ) -> anyhow::Result<()> {
        let mut c = None;
        let inputs = files
            .iter()
            .zip(values)
            .map(|(file, value)| {
                let master_token_file = fs::read(file)?;
                let master_token = sdk::OfflineToken::decode(master_token_file.as_slice())?;
                let sdk::offline_token::Data { id, currency, .. } = master_token
                    .data
                    .ok_or_else(|| anyhow::anyhow!("missing data in master token"))?;
                let cr = c.get_or_insert(currency.clone());
                if cr != &currency {
                    return Err(anyhow::anyhow!("Tokens must have same currency"));
                }
                Ok(sdk::redeemable_token::TokenInput {
                    input: id,
                    value: *value,
                })
            })
            .collect::<anyhow::Result<_>>()?;
        let currency = c.ok_or(anyhow::anyhow!("no input tokens found"))?;
        let g = libxid::new_generator();
        let token_id = g.new_id()?;
        let data = sdk::redeemable_token::Data {
            address: base64::decode(to)?,
            currency,
            id: token_id.to_vec(),
            inputs,
        };
        let key = context.signer();
        let mut token_data_buf = vec![];
        data.encode(&mut token_data_buf)?;
        let token = sdk::RedeemableToken {
            data: Some(data),
            signature: Some(key.sign_payload(&token_data_buf).await?),
        };
        let mut token_buf = vec![];
        token.encode(&mut token_buf)?;
        let path = format!("./rt-{}.tok", token_id);
        fs::write(path, token_buf)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Subcommand, Serialize, Deserialize)]
pub(crate) enum Redeem {
    #[command(alias = "t")]
    Token {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        account: account::AccountId,
    },
}

impl Redeem {
    pub(crate) async fn run(&self, context: &Context) -> anyhow::Result<()> {
        match self {
            Redeem::Token { token, account } => Self::redeem(token, *account, context).await,
        }
    }

    async fn redeem(file: &str, account: AccountId, context: &Context) -> anyhow::Result<()> {
        let token_file = fs::read(file)?;
        let token = sdk::RedeemableToken::decode(token_file.as_slice())?;
        context
            .ledger_client()
            .redeem_token(token, account, vec![])
            .await?;
        Ok(())
    }
}
