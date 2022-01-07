use prost::Message;
use prost_types::Any;

use crate::sdk::metadata::*;
use crate::sdk::transaction::{CreateTransfer, TransferStep};
use crate::sdk::FinalizedTransfer;

pub fn memo(memo: &str) -> Any {
    Memo {
        plaintext: memo.to_string(),
    }
    .any()
}

pub trait Metadata: Message + Sized + Default {
    const TYPE_URL: &'static str;

    fn any(&self) -> Any {
        Any {
            type_url: Self::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

pub trait MetadataExt {
    fn metadata<M: Metadata>(&self) -> Option<M>;
    fn memo(&self) -> String {
        self.metadata::<Memo>().unwrap_or_default().plaintext
    }
}

impl MetadataExt for TransferStep {
    fn metadata<M: Metadata>(&self) -> Option<M> {
        self.metadata
            .iter()
            .filter(|a| a.type_url == M::TYPE_URL)
            .find_map(|a| M::decode(a.value.as_slice()).ok())
    }
}

impl MetadataExt for CreateTransfer {
    fn metadata<M: Metadata>(&self) -> Option<M> {
        self.transfer_steps.iter().find_map(|step| step.metadata())
    }
}

impl MetadataExt for FinalizedTransfer {
    fn metadata<M: Metadata>(&self) -> Option<M> {
        self.transfer_steps.iter().find_map(|step| step.metadata())
    }
}

impl Metadata for Attachment {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Attachment";
}

impl Metadata for Memo {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Memo";
}

impl Metadata for Fee {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Fee";
}

impl Metadata for Withdraw {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Withdraw";
}

impl Metadata for Deposit {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Deposit";
}

impl Metadata for Contract {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Contract";
}
