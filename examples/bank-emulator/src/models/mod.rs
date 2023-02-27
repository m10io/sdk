use serde::{Deserialize, Serialize};

pub use accounts::*;
pub use assets::*;
pub use contacts::*;
pub use currencies::*;
pub use fees::*;
pub use notification_preferences::*;
pub use payments::*;
#[allow(unused_imports)]
pub use transfers::*;

mod accounts;
mod assets;
mod contacts;
mod currencies;
mod fees;
mod notification_preferences;
mod payments;
mod transfers;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListResponse<I, D> {
    pub data: Vec<D>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<NextPageToken<I>>,
}

impl<I, D> ListResponse<I, D> {
    pub fn _new(data: Vec<D>) -> ListResponse<I, D> {
        ListResponse { data, next: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NextPageToken<I> {
    pub id: I,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Page<I> {
    pub id: Option<I>,
    pub limit: Option<i32>,
}

impl<I> Page<I> {
    pub fn get_limit(&self) -> i32 {
        self.limit.unwrap_or(10)
    }
}

impl<I> From<Page<I>> for Option<NextPageToken<I>> {
    fn from(value: Page<I>) -> Option<NextPageToken<I>> {
        let Page { id, .. } = value;
        id.map(|id| NextPageToken { id })
    }
}
