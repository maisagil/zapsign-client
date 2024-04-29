use rustify::Wrapper;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

pub type Response = Vec<Document>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "open_id")]
    pub open_id: i64,
    pub token: String,
    pub status: String,
    pub name: String,
    #[serde(rename = "original_file")]
    pub original_file: String,
    #[serde(rename = "signed_file")]
    pub signed_file: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_update_at")]
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
