use crate::document_id::DocumentId;
use bytes::Bytes;
use m10_protos::{
    prost::FieldMask,
    sdk::operation::{Operation as Op, UpdateDocument},
    sdk::Operation,
    Pack,
};

pub mod account_sets;
pub mod accounts;
pub mod banks;
pub mod resource_id;
pub mod role_bindings;
pub mod roles;

pub use resource_id::ResourceId;

/// A struct for building a document update operation
///
/// # Example
///
/// ```rust
/// use m10_sdk::DocumentUpdate;
/// use m10_sdk::sdk::Account;
///
/// let mut update: DocumentUpdate<Account> = DocumentUpdate::new(uuid::Uuid::new_v4());
/// update.name("Test".to_string());
/// let operation = update.operation();
/// ```
pub struct DocumentUpdate<D> {
    pub(crate) document: D,
    pub(crate) mask: FieldMask,
    pub(crate) merge_repeated: bool,
}

impl<D: Pack> DocumentUpdate<D> {
    /// Creates a new document update operation with the passed id
    pub fn new(id: impl DocumentId) -> Self {
        let mut document = D::default();
        document.set_id(id.into_vec());
        Self {
            document,
            mask: FieldMask::default(),
            merge_repeated: false,
        }
    }

    /// Returns the ID of the document being updated
    pub fn id(&self) -> &[u8] {
        self.document.id()
    }

    /// Builds the document update, and returns the associated [`Operation`]
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
