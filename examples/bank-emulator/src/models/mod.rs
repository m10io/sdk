use serde::{Deserialize, Serialize};

pub use accounts::*;
pub use assets::*;
pub use contacts::*;
pub use fees::*;
pub use notification_preferences::*;
pub use payments::*;

mod accounts;
mod assets;
mod contacts;
mod fees;
mod notification_preferences;
mod payments;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
