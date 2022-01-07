use crate::document_id::DocumentId;
use bytes::Bytes;
use m10_sdk_protos::{
    arcadius2::operation::{Operation as Op, UpdateDocument},
    arcadius2::Operation,
    prost::FieldMask,
    Pack,
};

pub mod account_sets;
pub mod accounts;
pub mod role_bindings;
pub mod roles;

pub struct DocumentUpdate<D> {
    pub(crate) document: D,
    pub(crate) mask: FieldMask,
    pub(crate) merge_repeated: bool,
}

impl<D: Pack> DocumentUpdate<D> {
    pub fn new(id: impl DocumentId) -> Self {
        let mut document = D::default();
        document.set_id(id.into_vec());
        Self {
            document,
            mask: FieldMask::default(),
            merge_repeated: false,
        }
    }

    pub fn id(&self) -> &[u8] {
        self.document.id()
    }

    pub fn operation(&self) -> Operation {
        Operation {
            operation: Some(Op::UpdateDocument(UpdateDocument {
                collection: D::COLLECTION.to_string(),
                primary_key: Some(Bytes::copy_from_slice(self.document.id()).into()),
                document: self.document.pack(),
                field_mask: Some(self.mask.clone()),
                merge_repeated: self.merge_repeated,
            })),
        }
    }
}
