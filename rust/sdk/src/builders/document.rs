use crate::DocumentUpdate;
use core::convert::From;
use m10_protos::prost::Message;
use m10_protos::Pack;
use m10_protos::{sdk, Collection};

#[derive(Default)]
pub struct DocumentBuilder {
    docs: Vec<sdk::Operation>,
}

impl DocumentBuilder {
    pub fn insert<P: Pack>(mut self, doc: P) -> Self {
        self.docs.push(sdk::Operation::insert(doc));
        self
    }

    pub fn insert_custom(mut self, collection: Collection, doc: impl Message) -> Self {
        self.docs.push(sdk::Operation {
            operation: Some(sdk::operation::Operation::InsertDocument(
                sdk::operation::InsertDocument {
                    collection: collection.to_string(),
                    document: doc.encode_to_vec(),
                },
            )),
        });
        self
    }

    pub fn delete<P: Pack>(mut self, doc: P) -> Self {
        self.docs
            .push(sdk::Operation::delete::<P>(doc.id().to_vec()));
        self
    }

    pub fn delete_custom(mut self, collection: Collection, id: Vec<u8>) -> Self {
        self.docs.push(sdk::Operation {
            operation: Some(sdk::operation::Operation::DeleteDocument(
                sdk::operation::DeleteDocument {
                    collection: collection.to_string(),
                    primary_key: Some(bytes::Bytes::from(id).into()),
                },
            )),
        });
        self
    }

    pub fn update<P: Pack>(mut self, update: DocumentUpdate<P>) -> Self {
        self.docs.push(update.operation());
        self
    }
}

impl From<DocumentBuilder> for sdk::DocumentOperations {
    fn from(doc: DocumentBuilder) -> Self {
        Self {
            operations: doc.docs,
        }
    }
}
