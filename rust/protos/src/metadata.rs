use prost::Message;
use prost_types::Any;

use crate::sdk::metadata::*;
use crate::sdk::transaction::{CreateTransfer, TransferStep};
use crate::sdk::FinalizedTransfer;

pub fn memo(memo: &str) -> Memo {
    Memo {
        plaintext: memo.to_string(),
    }
}

pub trait MetadataType {
    const TYPE_URL: &'static str;
}

pub trait Metadata: MetadataType {
    fn any(&self) -> Any;
}

impl<M: MetadataType + Message + Sized + Default> Metadata for M {
    fn any(&self) -> Any {
        Any {
            type_url: Self::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

pub trait MetadataExt {
    // Deprecated - please use [`protobuf`] instead
    fn metadata<M: Metadata + Message + Default>(&self) -> Option<M> {
        self.protobuf::<M>()
    }

    fn protobuf<M: Metadata + Message + Default>(&self) -> Option<M> {
        self.with_type::<M>().and_then(|b| M::decode(b).ok())
    }

    fn with_type<M: MetadataType>(&self) -> Option<&[u8]>;

    fn memo(&self) -> String {
        self.protobuf::<Memo>().unwrap_or_default().plaintext
    }
}

impl MetadataExt for Any {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        (self.type_url == M::TYPE_URL).then_some(self.value.as_slice())
    }
}

impl<T: MetadataExt> MetadataExt for Vec<T> {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        self.iter().find_map(MetadataExt::with_type::<M>)
    }
}

impl MetadataExt for TransferStep {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        self.metadata.with_type::<M>()
    }
}

impl MetadataExt for CreateTransfer {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        self.transfer_steps.with_type::<M>()
    }
}

impl MetadataExt for FinalizedTransfer {
    fn with_type<M: MetadataType>(&self) -> Option<&[u8]> {
        self.transfer_steps.with_type::<M>()
    }
}

impl MetadataType for Attachment {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Attachment";
}

impl MetadataType for Memo {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Memo";
}

impl MetadataType for Fee {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Fee";
}

impl MetadataType for Withdraw {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Withdraw";
}

impl MetadataType for Deposit {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Deposit";
}

impl MetadataType for Contract {
    const TYPE_URL: &'static str = "m10.sdk.metadata.Contract";
}

impl MetadataType for SelfTransfer {
    const TYPE_URL: &'static str = "m10.sdk.metadata.SelfTransfer";
}

impl MetadataType for RebalanceTransfer {
    const TYPE_URL: &'static str = "m10.sdk.metadata.RebalanceTransfer";
}
