use m10_protos::prost::Message;
use m10_protos::sdk::{
    Contract, CreateLedgerTransfer, CreateLedgerTransfers, CreateTransfer, TransferStep,
};
use std::time::{Duration, SystemTime};

pub const DEFAULT_CONTRACT_DURATION: Duration = Duration::from_secs(300);

/// A builder for [`Contract`]
///
/// A [`Contract`] in M10's system is an agreement amoung multiple parties to complete a series of transactions.
/// Each contract contains series of endorsements from each of the parties. Contracts are designed
/// to be executed across ledgers, for scenarios like FX swaps or other multi-currency transactions.
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
    /// Adds a transfer to the contract between two accounts on a particular ledger.
    pub fn transfer(
        mut self,
        ledger_id: &str,
        from_account_id: Vec<u8>,
        to_account_id: Vec<u8>,
        amount: u64,
        memo: Option<&str>,
    ) -> Self {
        let memo = memo.map(m10_protos::metadata::memo);
        self.transfers.push(CreateLedgerTransfer {
            ledger_id: ledger_id.to_string(),
            nonce: fastrand::u64(..),
            transfer: Some(CreateTransfer {
                transfer_steps: vec![TransferStep {
                    from_account_id,
                    to_account_id,
                    amount,
                    metadata: memo.into_iter().collect(),
                }],
            }),
        });
        self
    }

    /// Adds a timeout to the contract
    pub fn valid_for(mut self, duration: Duration) -> Self {
        self.valid_for = duration;
        self
    }

    /// Builds the [`Contract`]
    pub fn build(self) -> Result<Contract, m10_protos::prost::EncodeError> {
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
