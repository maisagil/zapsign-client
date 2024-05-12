use rustify::Wrapper;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

pub type Response = Vec<Document>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Document {
    pub open_id: i64,
    pub token: String,
    pub status: String,
    pub name: String,
    pub original_file: String,
    pub signed_file: Option<String>,
    pub created_at: String,
    pub last_update_at: String,
}

// Pagination wrapper.
#[derive(Debug, Deserialize)]
pub struct ResponseWrapper<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: T,
}

impl<T: DeserializeOwned + Send + Sync> Wrapper for ResponseWrapper<T> {
    type Value = T;
}
