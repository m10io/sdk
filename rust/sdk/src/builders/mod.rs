mod account;
mod action;
mod document;
mod page;
mod transfer;
mod txn_filter;

pub use account::*;
pub use action::*;
pub use document::*;
pub use page::*;
pub use transfer::*;
pub use txn_filter::*;

pub struct Ctx<T> {
    pub value: T,
    pub context_id: Vec<u8>,
}

pub trait WithContext
where
    Self: Sized,
{
    fn context_id(self, context_id: Vec<u8>) -> Ctx<Self> {
        Ctx {
            value: self,
            context_id,
        }
    }
}

impl<T: WithContext> From<T> for Ctx<T> {
    fn from(t: T) -> Self {
        Self {
            value: t,
            context_id: vec![],
        }
    }
}
