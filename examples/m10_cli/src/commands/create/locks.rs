use clap::Args;
use m10_sdk::{account::AccountId, sdk};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::Context;

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct CreateLockArgs {
    /// Lock identifier
    #[arg(long)]
    lock_id: String,
    /// Settlement cycle identifier
    #[arg(long)]
    cycle_id: String,
    /// Account to reserve funds from
    #[arg(long)]
    account_id: AccountId,
    /// Reserved amount
    #[arg(short, long)]
    amount: u64,
    /// Human-readable reason
    #[arg(long, default_value = "SETTLEMENT")]
    reason: String,
    /// Optional settlement batch reference
    #[arg(long)]
    settlement_batch_ref: Option<String>,
}

impl CreateLockArgs {
    pub(super) async fn create(&self, context: &Context) -> anyhow::Result<()> {
        let transaction = sdk::CreateLock {
            lock_id: parse_identifier_bytes(&self.lock_id, "lock_id")?,
            cycle_id: parse_identifier_bytes(&self.cycle_id, "cycle_id")?,
            account_id: self.account_id.to_vec(),
            amount: self.amount,
            reason: self.reason.clone(),
            settlement_batch_ref: optional_text_bytes(self.settlement_batch_ref.as_ref()),
        };

        submit_transaction(transaction, context, "created lock")
            .await
            .map(|_| ())
    }
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct ReleaseLockArgs {
    /// Lock identifier
    #[arg(long)]
    lock_id: String,
    /// Optional release reference
    #[arg(long)]
    release_ref: Option<String>,
}

impl ReleaseLockArgs {
    pub(super) async fn release(&self, context: &Context) -> anyhow::Result<()> {
        let transaction = sdk::ReleaseLock {
            lock_id: parse_identifier_bytes(&self.lock_id, "lock_id")?,
            release_ref: optional_text_bytes(self.release_ref.as_ref()),
        };

        submit_transaction(transaction, context, "released lock")
            .await
            .map(|_| ())
    }
}

#[derive(Clone, Args, Debug, Serialize, Deserialize)]
pub(crate) struct RedeemLocksForCycleArgs {
    /// Settlement cycle identifier
    #[arg(long)]
    cycle_id: String,
    /// Optional redemption reference
    #[arg(long)]
    redemption_tx_ref: Option<String>,
    /// One redemption step as "<lock_id>:<holder_account_id>:<issuance_account_id>:<amount>"
    #[arg(long = "step", required = true)]
    steps: Vec<String>,
}

impl RedeemLocksForCycleArgs {
    pub(super) async fn redeem(&self, context: &Context) -> anyhow::Result<()> {
        let steps = self
            .steps
            .iter()
            .map(|step| parse_redemption_step(step))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let transaction = sdk::RedeemLocksForCycle {
            cycle_id: parse_identifier_bytes(&self.cycle_id, "cycle_id")?,
            redemption_tx_ref: optional_text_bytes(self.redemption_tx_ref.as_ref()),
            steps,
        };

        submit_transaction(transaction, context, "redeemed locks for cycle")
            .await
            .map(|_| ())
    }
}

async fn submit_transaction<T>(data: T, context: &Context, verb: &str) -> anyhow::Result<u64>
where
    T: Into<sdk::transaction_data::Data>,
{
    let request =
        m10_sdk::signed_transaction(context.ledger_client(), data, context.context_id()).await?;
    let response = m10_sdk::create_transaction(context.ledger_client(), request).await?;
    eprintln!("{verb}:");
    println!("{}", response.tx_id);
    Ok(response.tx_id)
}

fn parse_redemption_step(value: &str) -> anyhow::Result<sdk::RedemptionStep> {
    let parts: Vec<&str> = value.split(':').map(str::trim).collect();
    if parts.len() != 4 {
        anyhow::bail!(
            "invalid --step '{}': expected '<lock_id>:<holder_account_id>:<issuance_account_id>:<amount>'",
            value
        );
    }

    let holder_account_id = parts[1].parse::<AccountId>().map_err(|err| {
        anyhow::anyhow!(
            "invalid holder account id '{}' in --step '{}': {}",
            parts[1],
            value,
            err
        )
    })?;
    let issuance_account_id = parts[2].parse::<AccountId>().map_err(|err| {
        anyhow::anyhow!(
            "invalid issuance account id '{}' in --step '{}': {}",
            parts[2],
            value,
            err
        )
    })?;
    let amount = parts[3].parse::<u64>().map_err(|err| {
        anyhow::anyhow!(
            "invalid amount '{}' in --step '{}': {}",
            parts[3],
            value,
            err
        )
    })?;

    Ok(sdk::RedemptionStep {
        lock_id: parse_identifier_bytes(parts[0], "step.lock_id")?,
        holder_account_id: holder_account_id.to_vec(),
        issuance_account_id: issuance_account_id.to_vec(),
        amount,
    })
}

fn parse_identifier_bytes(value: &str, field: &str) -> anyhow::Result<Vec<u8>> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        anyhow::bail!("{} cannot be empty", field);
    }

    if let Ok(uuid) = Uuid::parse_str(trimmed) {
        return Ok(uuid.as_bytes().to_vec());
    }

    if let Some(hex_value) = trimmed.strip_prefix("0x") {
        if hex_value.len() % 2 != 0 {
            anyhow::bail!("{} hex value must have an even number of characters", field);
        }
        return hex::decode(hex_value)
            .map_err(|err| anyhow::anyhow!("invalid {} hex value '{}': {}", field, value, err));
    }

    if let Some(base64_value) = trimmed.strip_prefix("b64:") {
        return base64::decode(base64_value)
            .map_err(|err| anyhow::anyhow!("invalid {} base64 value '{}': {}", field, value, err));
    }

    Ok(trimmed.as_bytes().to_vec())
}

fn optional_text_bytes(value: Option<&String>) -> Vec<u8> {
    value
        .map(|text| text.as_bytes().to_vec())
        .unwrap_or_default()
}
