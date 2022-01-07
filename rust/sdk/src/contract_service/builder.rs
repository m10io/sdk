use crate::account::AccountId;
use m10_sdk_protos::prost::Message;
use m10_sdk_protos::sdk::{
    Contract, CreateLedgerTransfer, CreateLedgerTransfers, CreateTransfer, TransferStep,
};
use std::time::{Duration, SystemTime};

pub const DEFAULT_CONTRACT_DURATION: Duration = Duration::from_secs(300);

#[derive(Clone, Debug, Default)]
pub struct ContractBuilder {
    transfers: Vec<CreateLedgerTransfer>,
    valid_for: Duration,
}

impl From<CreateLedgerTransfers> for ContractBuilder {
    fn from(transfer_request: CreateLedgerTransfers) -> Self {
        let valid_for = if transfer_request.valid_until == 0 {
            DEFAULT_CONTRACT_DURATION
        } else {
            let valid_until = SystemTime::UNIX_EPOCH
                + std::time::Duration::from_micros(transfer_request.valid_until);
            valid_until
                .duration_since(SystemTime::now())
                .unwrap_or_else(|_| Duration::new(0, 0))
        };
        Self {
            transfers: transfer_request.transfers,
            valid_for,
        }
    }
}

impl From<Vec<CreateLedgerTransfer>> for ContractBuilder {
    fn from(transfers: Vec<CreateLedgerTransfer>) -> Self {
        Self {
            transfers,
            valid_for: DEFAULT_CONTRACT_DURATION,
        }
    }
}

impl ContractBuilder {
    pub fn transfer(
        mut self,
        ledger_id: &str,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: u64,
        memo: Option<&str>,
    ) -> Self {
        let memo = memo.map(m10_sdk_protos::memo);
        self.transfers.push(CreateLedgerTransfer {
            ledger_id: ledger_id.to_string(),
            nonce: fastrand::u64(1..u64::MAX),
            transfer: Some(CreateTransfer {
                transfer_steps: vec![TransferStep {
                    from_account_id: from_account_id.to_be_bytes().to_vec(),
                    to_account_id: to_account_id.to_be_bytes().to_vec(),
                    amount,
                    metadata: memo.into_iter().collect(),
                }],
            }),
        });
        self
    }

    pub fn valid_for(mut self, duration: Duration) -> Self {
        self.valid_for = duration;
        self
    }

    pub fn build(self) -> Result<Contract, m10_sdk_protos::prost::EncodeError> {
        let valid_until = (SystemTime::now() + self.valid_for)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        let transfer_reqs = CreateLedgerTransfers {
            transfers: self.transfers,
            valid_until,
        };
        let mut transactions = vec![];
        transfer_reqs.encode(&mut transactions)?;
        Ok(Contract {
            transactions,
            endorsements: Vec::new(),
        })
    }
}
