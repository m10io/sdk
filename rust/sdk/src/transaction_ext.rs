use m10_protos::sdk;
use m10_protos::sdk::transaction_data::Data;

pub trait TransactionExt {
    fn data(&self) -> Option<&Data>;
    fn into_data(self) -> Option<Data>;
}

impl TransactionExt for sdk::FinalizedTransaction {
    #[inline]
    fn data(&self) -> Option<&Data> {
        self.request
            .as_ref()
            .and_then(|req| req.data.as_ref())
            .and_then(|d| d.data.as_ref())
    }

    #[inline]
    fn into_data(self) -> Option<Data> {
        self.request.and_then(|req| req.data).and_then(|d| d.data)
    }
}

impl TransactionExt for sdk::TransactionRequestPayload {
    fn data(&self) -> Option<&Data> {
        self.data.as_ref().and_then(|d| d.data.as_ref())
    }

    fn into_data(self) -> Option<Data> {
        self.data.and_then(|d| d.data)
    }
}
