use crate::types::TxId;

pub const DEFAULT_TXN_LIMIT: u64 = 20;

pub struct TxnFilter<T> {
    pub(crate) filter: T,
    pub(crate) min: TxId,
    pub(crate) max: TxId,
    pub(crate) limit: u64,
}

impl<T> TxnFilter<T> {
    pub fn min_tx(mut self, tx_id: TxId) -> Self {
        self.min = tx_id;
        self
    }

    pub fn max(mut self, tx_id: TxId) -> Self {
        self.max = tx_id;
        self
    }

    pub fn limit(mut self, count: u64) -> Self {
        self.limit = count;
        self
    }
}
