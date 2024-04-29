use super::response::Response;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};

#[derive(Builder, Default, Endpoint, Deserialize, Serialize)]
#[endpoint(path = "docs", response = "Response", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct Request {
    #[endpoint(query = "page")]
    page: i32,
}
