use crate::account::AccountId;
use crate::builders::{TxnFilter, DEFAULT_TXN_LIMIT};
use crate::WithContext;
use core::convert::From;
use core::default::Default;
use m10_protos::prost::Any;
use m10_protos::sdk::list_transfer_request::Filter;
use m10_protos::{sdk, Metadata};

pub struct StepBuilder {
    from: AccountId,
    to: AccountId,
    amount: u64,
    metadata: Vec<Any>,
}

impl StepBuilder {
    pub fn new(from: AccountId, to: AccountId, amount: u64) -> Self {
        Self {
            from,
            to,
            amount,
            metadata: vec![],
        }
    }

    pub fn metadata(mut self, value: impl Metadata) -> Self {
        self.metadata.push(value.any());
        self
    }

    pub fn any_metadata(mut self, value: Any) -> Self {
        self.metadata.push(value);
        self
    }

    pub fn custom_metadata(
        mut self,
        type_url: impl Into<String>,
        payload: impl Into<Vec<u8>>,
    ) -> Self {
        self.metadata.push(Any {
            type_url: type_url.into(),
            value: payload.into(),
        });
        self
    }
}

impl From<StepBuilder> for sdk::TransferStep {
    fn from(step: StepBuilder) -> Self {
        Self {
            from_account_id: step.from.to_vec(),
            to_account_id: step.to.to_vec(),
            amount: step.amount,
            metadata: step.metadata,
        }
    }
}

#[derive(Default)]
pub struct TransferBuilder(Vec<StepBuilder>);

impl TransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(mut self, step: StepBuilder) -> Self {
        self.0.push(step);
        self
    }
}

impl From<TransferBuilder> for sdk::CreateTransfer {
    fn from(transfer: TransferBuilder) -> Self {
        Self {
            transfer_steps: transfer
                .0
                .into_iter()
                .map(sdk::TransferStep::from)
                .collect(),
        }
    }
}

pub struct TransferFilter {
    filter: Filter,
    include_child_accounts: bool,
}

impl TxnFilter<TransferFilter> {
    pub fn by_account(id: AccountId) -> Self {
        Self {
            filter: TransferFilter {
                filter: Filter::AccountId(id.to_vec()),
                include_child_accounts: false,
            },
            min: 0,
            max: u64::MAX,
            limit: DEFAULT_TXN_LIMIT,
        }
    }

    pub fn by_context_id(context_id: Vec<u8>) -> Self {
        Self {
            filter: TransferFilter {
                filter: Filter::ContextId(context_id),
                include_child_accounts: false,
            },
            min: 0,
            max: u64::MAX,
            limit: DEFAULT_TXN_LIMIT,
        }
    }

    pub fn include_child_accounts(mut self, enable: bool) -> Self {
        self.filter.include_child_accounts = enable;
        self
    }
}

impl From<TxnFilter<TransferFilter>> for sdk::ListTransferRequest {
    fn from(filter: TxnFilter<TransferFilter>) -> Self {
        sdk::ListTransferRequest {
            filter: Some(filter.filter.filter),
            min_tx_id: filter.min,
            max_tx_id: filter.max,
            include_child_accounts: filter.filter.include_child_accounts,
            limit: filter.limit,
        }
    }
}

impl WithContext for TransferBuilder {}
