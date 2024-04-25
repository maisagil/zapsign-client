use derive_builder::Builder;
use rustify::Wrapper;
use rustify_derive::Endpoint;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

pub type DocsResponse = Vec<Document>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Eq))]
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

// Defines an API endpoint at /test/path that takes no inputs and returns an
// empty response.
#[derive(Builder, Default, Endpoint, Serialize)]
#[endpoint(path = "docs", response = "DocsResponse", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct Docs {
    #[endpoint(query = "page")]
    page: i32,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PaginationWrapper<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: T,
}

impl<T: DeserializeOwned + Send + Sync> Wrapper for PaginationWrapper<T> {
    type Value = T;
}
