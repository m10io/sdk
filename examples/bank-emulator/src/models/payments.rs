use chrono::{DateTime, NaiveDateTime, Utc};
use m10_sdk::{EnhancedTransfer, MetadataExt};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::NextPageToken;

#[derive(Serialize, Deserialize)]
pub struct TransferChain {
    id: u64,
    timestamp: String,
    amount: u64,
    instrument: String,
    memo: String,
    sender: AccountInfo,
    receiver: AccountInfo,
}

impl TransferChain {
    pub fn try_from_transfer(
        transfer: EnhancedTransfer,
        instrument: &str,
    ) -> Result<TransferChain, Error> {
        // TODO: don't assume single transfer step
        let EnhancedTransfer {
            mut transfer,
            mut enhanced_steps,
        } = transfer;
        let transfer_step = transfer.transfer_steps.swap_remove(0);
        let enhanced_step = enhanced_steps.swap_remove(0);

        Ok(TransferChain {
            id: transfer.tx_id,
            timestamp: DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::from_timestamp_opt(
                    (transfer.timestamp / 1_000_000) as i64,
                    ((transfer.timestamp % 1_000_000) * 1000) as u32,
                )
                .expect("expected valid timestamp"),
                Utc,
            )
            .to_string(),
            amount: transfer_step.amount,
            instrument: instrument.into(),
            memo: transfer_step.memo(),
            sender: AccountInfo {
                name: enhanced_step
                    .from
                    .map(|a| a.public_name)
                    .unwrap_or_else(|| "Unknown".to_string()),
                account_id: hex::encode(transfer_step.from_account_id),
                bank: enhanced_step
                    .from_bank
                    .map(|x| x.public_name)
                    .unwrap_or_default(),
            },
            receiver: AccountInfo {
                name: enhanced_step
                    .to
                    .map(|a| a.public_name)
                    .unwrap_or_else(|| "Unknown".to_string()),
                account_id: hex::encode(transfer_step.to_account_id),
                bank: enhanced_step
                    .to_bank
                    .map(|x| x.public_name)
                    .unwrap_or_default(),
            },
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Payment {
    pub id: u64,
    pub contract_id: Option<String>,
    pub transfers: Vec<TransferChain>,
}

impl From<&Payment> for NextPageToken<u64> {
    fn from(value: &Payment) -> NextPageToken<u64> {
        let Payment { id, .. } = value;
        NextPageToken { id: *id }
    }
}

#[derive(Serialize, Deserialize)]
struct AccountInfo {
    name: String,
    account_id: String,
    bank: String,
}
