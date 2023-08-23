use std::fs;

use m10_sdk::{account::AccountId, prost::Message, sdk, Signer};

use crate::{context::Context, utils};

pub(crate) fn verify_offline(file: &str) -> anyhow::Result<()> {
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

pub(crate) fn verify_redeemable(master: &str, file: &str) -> anyhow::Result<()> {
    let master_token_file = fs::read(master)?;
    let sdk::OfflineToken { data, .. } = sdk::OfflineToken::decode(master_token_file.as_slice())?;
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

pub(crate) async fn issue_token(
    files: &[String],
    values: &[u64],
    to: &str,
    config: &crate::Config,
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
    let key = if let Some(key_file) = &config.key_file {
        utils::load_key_pair(key_file)
    } else if let Some(k) = &config.signing_key {
        utils::key_pair_from_env(k)
    } else {
        Err(anyhow::anyhow!("no key giving"))
    }?;
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

pub(crate) async fn redeem_token(
    file: &str,
    account: AccountId,
    config: &crate::Config,
) -> anyhow::Result<()> {
    let token_file = fs::read(file)?;
    let token = sdk::RedeemableToken::decode(token_file.as_slice())?;
    let context = Context::new(config)?;
    context
        .m10_client
        .redeem_token(token, account, vec![])
        .await?;
    Ok(())
}
