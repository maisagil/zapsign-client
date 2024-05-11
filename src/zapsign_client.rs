use std::num::ParseIntError;
use thiserror::Error;

use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderValue,
};
pub use rustify::Endpoint;
use rustify::{errors::ClientError, MiddleWare};

use crate::api::create_document::{request::RequestBuilder, response::Response};

pub struct ZapsignProvider {
    client: rustify::Client,
    auth_key: String,
}

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error(transparent)]
    Client(#[from] ClientError),
}

impl ZapsignProvider {
    pub fn new(auth_key: &str, base_url: &str) -> Self {
        let http = reqwest::Client::new();
        Self {
            client: rustify::Client::new(base_url, http),
            auth_key: auth_key.to_string(),
        }
    }

    pub async fn create_document(
        &self,
        doc_builder: &mut RequestBuilder,
    ) -> Result<Response, ProviderError> {
        let endpoint = doc_builder.build().unwrap();
        let auth_middleware = ZapsignMiddleware::new("v1".to_string(), self.auth_key.to_string());

        let endpoint_result = endpoint
            .with_middleware(&auth_middleware)
            .exec(&self.client)
            .await?;

        let result: Response = endpoint_result.parse()?;

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct ZapsignMiddleware {
    pub version: String,
    pub auth: String,
}

impl ZapsignMiddleware {
    pub fn new(version: String, auth: String) -> Self {
        Self { version, auth }
    }
}

impl MiddleWare for ZapsignMiddleware {
    fn request<E: rustify::Endpoint>(
        &self,
        _: &E,
        req: &mut http::request::Request<Vec<u8>>,
    ) -> Result<(), rustify::errors::ClientError> {
        let auth_as_header: HeaderValue = format!("{} {}", "Bearer", self.auth).parse().unwrap();
        req.headers_mut().append(AUTHORIZATION, auth_as_header);

        let ctype_as_header = HeaderValue::from_str("application/json");

        req.headers_mut()
            .append(CONTENT_TYPE, ctype_as_header.unwrap());

        let url = url::Url::parse(req.uri().to_string().as_str()).unwrap();
        let url_c = url.clone();

        dbg!(url_c.as_str());

        Ok(())
    }

    fn response<E: rustify::Endpoint>(
        &self,
        endpoint: &E,
        resp: &mut http::response::Response<Vec<u8>>,
    ) -> Result<(), rustify::errors::ClientError> {
        Ok(())
    }
}
