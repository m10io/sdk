use crate::account::AccountId;
use crate::error::{M10Error, M10Result};
use crate::types::TxId;
use crate::{EnhancedTransfer, EnhancedTransferStep, TransactionExt};
use core::convert::{From, TryFrom};
use core::result::Result;
use core::time::Duration;
use m10_protos::prost::Any;
use m10_protos::sdk::transaction_data::Data;
use m10_protos::{sdk, MetadataExt, MetadataType};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct TransferStep {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: u64,
    pub metadata: Vec<Any>,
}

#[cfg(feature = "format")]
impl std::fmt::Display for TransferStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            from,
            to,
            amount,
            metadata,
        } = self;
        write!(f, "Step{{ from={from} to={to} amount={amount} metadata=[",)?;
        for m in metadata {
            write!(f, "{},", m.type_url)?;
        }
        write!(f, "] }}")
    }
}

impl MetadataExt for TransferStep {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        self.metadata
            .iter()
            .find(|a| a.type_url == M::TYPE_URL)
            .map(|a| a.value.as_slice())
    }
}

impl TryFrom<sdk::TransferStep> for TransferStep {
    type Error = M10Error;

    fn try_from(step: sdk::TransferStep) -> Result<Self, Self::Error> {
        Ok(TransferStep {
            from: AccountId::try_from_be_slice(&step.from_account_id)?,
            to: AccountId::try_from_be_slice(&step.to_account_id)?,
            amount: step.amount,
            metadata: vec![],
        })
    }
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[derive(Clone, Debug, Copy)]
pub enum TransferStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}

impl From<sdk::finalized_transfer::TransferState> for TransferStatus {
    fn from(status: sdk::finalized_transfer::TransferState) -> Self {
        match status {
            sdk::finalized_transfer::TransferState::Pending => TransferStatus::Pending,
            sdk::finalized_transfer::TransferState::Accepted => TransferStatus::Accepted,
            sdk::finalized_transfer::TransferState::Rejected => TransferStatus::Rejected,
            sdk::finalized_transfer::TransferState::Expired => TransferStatus::Expired,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transfer {
    pub tx_id: TxId,
    pub context_id: Vec<u8>,
    pub timestamp: SystemTime,
    pub steps: Vec<TransferStep>,
    pub success: bool,
    pub status: TransferStatus,
}

#[cfg(feature = "format")]
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            tx_id,
            context_id,
            timestamp,
            steps,
            success,
            status,
        } = self;
        write!(f, "Transfer{{ tx_id={tx_id} context_id={} timestamp={timestamp:?} success={success} status={status} steps=[", hex::encode(context_id))?;
        for step in steps {
            write!(f, "{step},")?;
        }
        write!(f, "] }}")
    }
}

impl TryFrom<sdk::FinalizedTransfer> for Transfer {
    type Error = M10Error;

    fn try_from(transfer: sdk::FinalizedTransfer) -> Result<Self, Self::Error> {
        Ok(Transfer {
            tx_id: transfer.tx_id,
            context_id: transfer.context_id,
            success: transfer.error.is_none(),
            timestamp: UNIX_EPOCH + Duration::from_micros(transfer.timestamp),
            steps: transfer
                .transfer_steps
                .into_iter()
                .map(TransferStep::try_from)
                .collect::<M10Result<_>>()?,
            status: TransferStatus::from(
                sdk::finalized_transfer::TransferState::from_i32(transfer.state)
                    .ok_or(M10Error::InvalidTransaction)?,
            ),
        })
    }
}

impl TryFrom<sdk::FinalizedTransaction> for Transfer {
    type Error = M10Error;

    fn try_from(txn: sdk::FinalizedTransaction) -> Result<Self, Self::Error> {
        let response = txn.response.as_ref().ok_or(M10Error::InvalidTransaction)?;
        let context_id = txn
            .request
            .as_ref()
            .ok_or(M10Error::InvalidTransaction)?
            .context_id
            .clone();
        let success = response.error.is_none();
        let tx_id = response.tx_id;
        let timestamp = UNIX_EPOCH + Duration::from_micros(response.timestamp);
        match txn.data().ok_or(M10Error::InvalidTransaction)? {
            Data::Transfer(transfer) => Ok(Self {
                tx_id,
                context_id,
                timestamp,
                steps: transfer
                    .transfer_steps
                    .iter()
                    .cloned()
                    .map(TransferStep::try_from)
                    .collect::<M10Result<_>>()?,
                success,
                status: TransferStatus::Accepted,
            }),
            Data::InitiateTransfer(transfer) => Ok(Self {
                tx_id,
                context_id,
                timestamp,
                steps: transfer
                    .transfer_steps
                    .iter()
                    .cloned()
                    .map(TransferStep::try_from)
                    .collect::<M10Result<_>>()?,
                success,
                status: TransferStatus::Pending,
            }),
            Data::CommitTransfer(commit) => {
                let status = if success {
                    TransferStatus::from(
                        sdk::finalized_transfer::TransferState::from_i32(commit.new_state)
                            .ok_or(M10Error::InvalidTransaction)?,
                    )
                } else {
                    TransferStatus::Pending
                };
                let transfer = response
                    .transfer_committed
                    .as_ref()
                    .ok_or(M10Error::InvalidTransaction)?;
                Ok(Self {
                    tx_id,
                    context_id,
                    timestamp,
                    steps: transfer
                        .transfer_steps
                        .iter()
                        .cloned()
                        .map(TransferStep::try_from)
                        .collect::<M10Result<_>>()?,
                    success,
                    status,
                })
            }
            _ => Err(M10Error::InvalidTransaction),
        }
    }
}

impl MetadataExt for Transfer {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        self.steps.iter().find_map(TransferStep::with_type::<M>)
    }
}

#[derive(Clone, Debug)]
pub struct ExpandedTransfer {
    pub tx_id: TxId,
    pub context_id: Vec<u8>,
    pub timestamp: SystemTime,
    pub steps: Vec<ExpandedTransferStep>,
    pub success: bool,
    pub status: TransferStatus,
}

#[cfg(feature = "format")]
impl std::fmt::Display for ExpandedTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            tx_id,
            context_id,
            timestamp,
            steps,
            success,
            status,
        } = self;
        write!(f, "Transfer{{ tx_id={tx_id} context_id={} timestamp={timestamp:?} success={success} status={status} steps=[", hex::encode(context_id))?;
        for step in steps {
            write!(f, "{step},")?;
        }
        write!(f, "] }}")
    }
}

#[cfg_attr(feature = "format", derive(parse_display::Display))]
#[cfg_attr(
    feature = "format",
    display("Account{{ id={id} name={public_name} image={profile_image_url} currency={code}({decimals}) }}")
)]
#[derive(Clone, Debug)]
pub struct ExpandedAccount {
    pub id: AccountId,
    pub public_name: String,
    pub profile_image_url: String,
    pub code: String,
    pub decimals: u32,
}

#[derive(Clone, Debug)]
pub struct ExpandedTransferStep {
    pub from: ExpandedAccount,
    pub to: ExpandedAccount,
    pub amount: u64,
    pub metadata: Vec<Any>,
}

#[cfg(feature = "format")]
impl std::fmt::Display for ExpandedTransferStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            from,
            to,
            amount,
            metadata,
        } = self;
        write!(f, "Step{{ from={from} to={to} amount={amount} metadata=[",)?;
        for m in metadata {
            write!(f, "{},", m.type_url)?;
        }
        write!(f, "] }}")
    }
}

impl TryFrom<(EnhancedTransferStep, sdk::TransferStep)> for ExpandedTransferStep {
    type Error = M10Error;

    fn try_from(
        (step, raw): (EnhancedTransferStep, sdk::TransferStep),
    ) -> Result<Self, Self::Error> {
        let from = step.from.ok_or(M10Error::InvalidTransaction)?;
        let to = step.to.ok_or(M10Error::InvalidTransaction)?;
        Ok(Self {
            from: ExpandedAccount {
                id: AccountId::try_from_be_slice(&from.account_id)?,
                public_name: from.public_name,
                profile_image_url: from.profile_image_url,
                code: from.code,
                decimals: from.decimal_places,
            },
            to: ExpandedAccount {
                id: AccountId::try_from_be_slice(&to.account_id)?,
                public_name: to.public_name,
                profile_image_url: to.profile_image_url,
                code: to.code,
                decimals: to.decimal_places,
            },
            amount: raw.amount,
            metadata: raw.metadata,
        })
    }
}

impl TryFrom<EnhancedTransfer> for ExpandedTransfer {
    type Error = M10Error;

    fn try_from(transfer: EnhancedTransfer) -> Result<Self, Self::Error> {
        Ok(Self {
            tx_id: transfer.transfer.tx_id,
            context_id: transfer.transfer.context_id,
            timestamp: UNIX_EPOCH + Duration::from_micros(transfer.transfer.timestamp),
            steps: transfer
                .enhanced_steps
                .into_iter()
                .zip(transfer.transfer.transfer_steps.into_iter())
                .map(ExpandedTransferStep::try_from)
                .collect::<M10Result<_>>()?,
            success: transfer.transfer.error.is_none(),
            status: TransferStatus::from(
                sdk::finalized_transfer::TransferState::from_i32(transfer.transfer.state)
                    .ok_or(M10Error::InvalidTransaction)?,
            ),
        })
    }
}
