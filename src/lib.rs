mod api;
mod endpoints;
mod zapsign_client;

pub use endpoints::*;

#[cfg(test)]
mod tests {
    use crate::api::create_document::request::{Signer, SignerBuilder};
    use crate::api::create_document::{self};
    use crate::api::list_documents::{
        request::Request as ListDocuments,
        response::ResponseWrapper as ListDocumentsResponseWrapper,
    };
    use crate::zapsign_client::ZapsignProvider;
    use rustify::{errors::ClientError, Client, Endpoint};

    const API_TOKEN: &str = std::env!("API_TOKEN");
    const BASE_URL: &str = std::env!("BASE_URL");
    #[tokio::test]
    async fn teste() {
        let provider = ZapsignProvider::new(API_TOKEN);

        let mut req_builder = create_document::request::RequestBuilder::default();

        let mut signer_vec: Vec<Signer> = vec![];

        signer_vec.push(
            SignerBuilder::default()
                .name("My First API Signer PDF".to_string())
                .build()
                .unwrap(),
        );

        let req = req_builder
            .name("TesteDoc")
            .url_pdf("https://www.zero2prod.com/assets/sample_zero2prod.pdf".to_string())
            .lang("pt-br")
            .disable_signer_emails(true)
            .signed_file_only_finished(true)
            .external_id(None)
            .signers(signer_vec);

        let res = provider.create_document(req).await;
        let _ = dbg!(&res);

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn it_works() {
        let endpoint = ListDocuments::builder().page(1).build().unwrap();
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

        let client = Client::new(BASE_URL, req_client); // Configures base address of http://api.com

        let result = endpoint.exec(&client).await; // Sends GET request to http://api.com/test/path
        match result {
            Ok(r) => match r.wrap::<ListDocumentsResponseWrapper<_>>() {
                Ok(d) => {
                    d.results.iter().for_each(|d| println!("{:#?}", d));
                }
                Err(ClientError::ResponseParseError { source, content }) => {
                    println!("{}: {}", source, content.unwrap_or("".into()))
                }
                r => println!("{:#?}", r),
            },
            Err(e) => match e {
                _ => println!("{:#?}", e),
            },
        };
    }
}
