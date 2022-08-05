use m10_protos::prost::bytes::Bytes;
use uuid::Uuid;

/// A trait representing any type that can be used as a DocumentId
pub trait DocumentId {
    fn into_bytes(self) -> Bytes;
    fn into_vec(self) -> Vec<u8>;
}

impl DocumentId for Uuid {
    fn into_bytes(self) -> Bytes {
        Bytes::copy_from_slice(self.as_bytes())
    }

    fn into_vec(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl DocumentId for Vec<u8> {
    fn into_bytes(self) -> Bytes {
        Bytes::from(self)
    }

    fn into_vec(self) -> Vec<u8> {
        self
    }
}

impl DocumentId for Bytes {
    fn into_bytes(self) -> Bytes {
        self
    }

    fn into_vec(self) -> Vec<u8> {
        self.as_ref().to_vec()
    }
}
