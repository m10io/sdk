use crate::account::AccountId;
use m10_sdk_protos::prost::Message;
use m10_sdk_protos::sdk::{Contract, CreateLedgerTransfer, CreateLedgerTransfers};
use serde::Serialize;
use thiserror::Error;

pub type ContractId = Vec<u8>;

#[derive(Clone, Debug, Serialize)]
pub struct TransferInfo {
    pub ledger_id: String,
    #[serde(serialize_with = "account_id_as_hex")]
    pub from_account_id: AccountId,
    #[serde(serialize_with = "account_id_as_hex")]
    pub to_account_id: AccountId,
    pub amount: u64,
    pub nonce: u64,
}

pub trait FinalizedContractExt {
    /// Calculates the contract ID
    fn id(&self) -> ContractId;

    /// Extracts a list of the proposed transfers
    fn transfer_info(&self) -> Result<Vec<TransferInfo>, ContractError>;
}

impl FinalizedContractExt for Contract {
    fn id(&self) -> ContractId {
        ring::digest::digest(&ring::digest::SHA256, &self.transactions)
            .as_ref()
            .to_vec()
    }

    fn transfer_info(&self) -> Result<Vec<TransferInfo>, ContractError> {
        let CreateLedgerTransfers { transfers, .. } =
            CreateLedgerTransfers::decode(self.transactions.as_ref())?;
        transfers
            .into_iter()
            .flat_map(
                |CreateLedgerTransfer {
                     ref ledger_id,
                     nonce,
                     transfer,
                 }| {
                    transfer
                        .map(|tf| tf.transfer_steps)
                        .unwrap_or_default()
                        .into_iter()
                        .map(move |step| {
                            Ok(TransferInfo {
                                ledger_id: ledger_id.clone(),
                                from_account_id: AccountId::try_from_be_slice(
                                    &step.from_account_id,
                                )?,
                                to_account_id: AccountId::try_from_be_slice(&step.to_account_id)?,
                                amount: step.amount,
                                nonce,
                            })
                        })
                        .collect::<Vec<_>>()
                },
            )
            .collect::<Result<Vec<_>, _>>()
    }
}

fn account_id_as_hex<S: serde::Serializer>(
    account_id: &AccountId,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&hex::encode(&account_id.to_be_bytes()))
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("account id {0}")]
    AccountId(#[from] crate::account::AccountIdError),
    #[error("prost decode {0}")]
    ProstDecode(#[from] m10_sdk_protos::prost::DecodeError),
}
