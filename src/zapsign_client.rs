use std::str::FromStr;

use http::HeaderValue;
pub use rustify::Endpoint;
use rustify::{clients::reqwest::Client as HTTPClient, MiddleWare};

use crate::api::create_document::{self, request::RequestBuilder, response::Response};

const BASE_URL: &str = std::env!("BASE_URL");
pub struct ZapsignProvider {
    client: rustify::Client,
}

impl ZapsignProvider {
    pub fn new(auth_key: &str) -> Self {
        let http = reqwest::Client::new();
        Self {
            client: rustify::Client::new(BASE_URL, http),
        }
    }

    pub async fn create_document(&self, doc_builder: RequestBuilder) -> Result<Response, String> {
        let endpoint = doc_builder.build().unwrap();
        let result: create_document::response::Response =
            endpoint.exec(&self.client).await.unwrap().parse().unwrap();

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
        let auth_as_header = HeaderValue::from_str(&format!("{}{}", "Bearer ", self.auth));
        req.headers_mut()
            .append("Authorization: ", auth_as_header.unwrap());

        let url = url::Url::parse(req.uri().to_string().as_str()).unwrap();
        let mut url_c = url.clone();
        let mut segs: Vec<&str> = url.path_segments().unwrap().collect();
        segs.insert(0, &self.version);
        url_c.set_path(format!("{}{}", self.version, url_c.path()).as_str());
        *req.uri_mut() = http::Uri::from_str(url_c.as_str()).unwrap();

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
