mod endpoints;

pub use endpoints::*;
pub use rustify::{errors::ClientError, Client, Endpoint};

#[cfg(test)]
mod tests {
    use super::{Client, Endpoint};
    use crate::get_docs::{Docs, Document, PaginationWrapper};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const API_TOKEN: &str = std::env!("API_TOKEN");
    // const BASE_URL: &str = std::env!("BASE_URL");

    #[tokio::test]
    async fn it_works() {
        let mock_server = MockServer::start().await;

        let mock_response = PaginationWrapper {
            results: vec![Document::default()],
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path("/docs"))
            .and(header("Authorization", API_TOKEN))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;

        let endpoint = Docs::builder().page(1).build().unwrap();
        // client with authentication token
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, API_TOKEN.parse().unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        let req_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let client = Client::new(&mock_server.uri(), req_client); // Configures base address of http://api.com

        let result = endpoint.exec(&client).await; // Sends GET request to http://api.com/test/path
        assert!(result.is_ok());
        let result = result.unwrap().wrap::<PaginationWrapper<_>>().unwrap();
        assert_eq!(mock_response, result);
        // match result {
        //     Ok(r) => match r.wrap::<PaginationWrapper<_>>() {
        //         Ok(d) => {
        //             d.results.iter().for_each(|d| println!("{:#?}", d));
        //         }
        //         Err(ClientError::ResponseParseError { source, content }) => {
        //             println!("{}: {}", source, content.unwrap_or("".into()))
        //         }
        //         r => println!("{:#?}", r),
        //     },
        //     Err(e) => match e {
        //         _ => println!("{:#?}", e),
        //     },
        // };
    }
}
