use m10_protos::prost::{DecodeError, Message};
use m10_protos::sdk::{Contract, CreateLedgerTransfer, CreateLedgerTransfers};
use serde::Serialize;
use serde_with::{hex::Hex, serde_as};

pub type ContractId = Vec<u8>;

#[serde_as]
#[derive(Clone, Debug, Serialize)]
pub struct TransferInfo {
    pub ledger_id: String,
    #[serde_as(as = "Hex")]
    pub from_account_id: Vec<u8>,
    #[serde_as(as = "Hex")]
    pub to_account_id: Vec<u8>,
    pub amount: u64,
    pub nonce: u64,
}

pub trait FinalizedContractExt {
    /// Calculates the contract ID
    fn id(&self) -> ContractId;

    /// Extracts a list of the proposed transfers
    fn transfer_info(&self) -> Result<Vec<TransferInfo>, DecodeError>;
}

impl FinalizedContractExt for Contract {
    fn id(&self) -> ContractId {
        ring::digest::digest(&ring::digest::SHA256, &self.transactions)
            .as_ref()
            .to_vec()
    }

    fn transfer_info(&self) -> Result<Vec<TransferInfo>, DecodeError> {
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
                                from_account_id: step.from_account_id,
                                to_account_id: step.to_account_id,
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
