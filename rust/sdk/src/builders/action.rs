use crate::account::AccountId;
use crate::builders::{TxnFilter, DEFAULT_TXN_LIMIT};
use crate::types::Target;
use crate::WithContext;
use core::convert::From;
use m10_protos::prost::Message;
use m10_protos::sdk::list_actions_request::Filter;
use m10_protos::{sdk, Metadata};

pub struct ActionBuilder {
    name: String,
    from: AccountId,
    to: Target,
    payload: Vec<u8>,
}

impl ActionBuilder {
    pub fn for_account(name: impl Into<String>, from: AccountId, to: AccountId) -> Self {
        Self {
            name: name.into(),
            from,
            to: Target::Account(to),
            payload: vec![],
        }
    }

    pub fn for_all(name: impl Into<String>, from: AccountId) -> Self {
        Self {
            name: name.into(),
            from,
            to: Target::Any,
            payload: vec![],
        }
    }

    pub fn metadata(mut self, value: impl Metadata) -> Self {
        self.payload = value.any().encode_to_vec();
        self
    }

    pub fn payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }
}

impl From<ActionBuilder> for sdk::InvokeAction {
    fn from(action: ActionBuilder) -> Self {
        Self {
            from_account: action.from.to_vec(),
            name: action.name,
            target: Some(action.to.into()),
            payload: action.payload,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ActionsFilter {
    name: String,
    filter: Filter,
}

impl TxnFilter<ActionsFilter> {
    pub fn by_account(name: String, id: AccountId) -> Self {
        Self {
            filter: ActionsFilter {
                name,
                filter: Filter::AccountId(id.to_vec()),
            },
            min: 0,
            max: u64::MAX,
            limit: DEFAULT_TXN_LIMIT,
        }
    }

    pub fn by_context_id(name: String, context_id: Vec<u8>) -> Self {
        Self {
            filter: ActionsFilter {
                name,
                filter: Filter::ContextId(context_id),
            },
            min: 0,
            max: u64::MAX,
            limit: DEFAULT_TXN_LIMIT,
        }
    }
}

impl From<TxnFilter<ActionsFilter>> for sdk::ListActionsRequest {
    fn from(filter: TxnFilter<ActionsFilter>) -> Self {
        sdk::ListActionsRequest {
            name: filter.filter.name,
            filter: Some(filter.filter.filter),
            min_tx_id: filter.min,
            max_tx_id: filter.max,
            limit: filter.limit,
        }
    }
}

impl WithContext for ActionBuilder {}
