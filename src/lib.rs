mod api;
mod zapsign_client;

pub use api::*;
pub use zapsign_client::*;

#[cfg(test)]
mod tests {
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::{
        api::create_document::request::{Request, Signer, SignerBuilder},
        *,
    };

    const API_TOKEN: &str = "API_TOKEN";
    #[tokio::test]
    async fn create_document_works() {
        let mock_server = MockServer::start().await;
        let mock_response = serde_json::json!(
        {
            "open_id": 5,
            "token": "eb9c367a-e62f-4992-8360-b0219deaeecc",
            "status": "pending",
            "name": "oi",
            "original_file": "https://zapsign.s3.amazonaws.com/pdf/62c2b027-d8fc-4392-xas75-f3c46c3cfc7a/d33336-4182-8c8b-ded5287e4c0f.pdf",
            "signed_file": null,
            "created_at": "2020-04-16T03:33:46.241747Z",
            "last_update_at": "2020-04-16T03:33:46.241775Z",
            "signers": [
                {
                    "token": "921c115d-4a6e-445d-bdca-03fadedbbc0b",
                    "sign_url": "https://app.zapsign.com.br/verificar/921c115d-4a6e-445d-bdca-03fadedbbc0b",
                    "status": "new",
                    "name": "João da Silva",
                    "email": "",
                    "phone_country": "",
                    "phone_number": "",
                    "times_viewed": 0,
                    "last_view_at": null,
                    "signed_at": null
                },
                {
                    "token": "07fb0a0a-4b7d-49a5-bd7b-4958265c4e46",
                    "sign_url": "https://app.zapsign.com.br/verificar/07fb0a0a-4b7d-49a5-bd7b-4958265c4e46",
                    "status": "new",
                    "name": "Fulano Siclano",
                    "email": "",
                    "phone_country": "",
                    "phone_number": "",
                    "times_viewed": 0,
                    "last_view_at": null,
                    "signed_at": null
                }
            ]
        }

                );
        Mock::given(method("POST"))
            .and(path("/docs/"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            // Mounting the mock on the mock server - it's now effective!
            .mount(&mock_server)
            .await;
        let provider = Provider::new(API_TOKEN, &mock_server.uri());

        let mut req_builder = Request::builder();

        let signer_vec: Vec<Signer> = vec![SignerBuilder::default()
            .name("My First API Signer PDF".to_string())
            .build()
            .unwrap()];

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

    // #[tokio::test]
    // async fn it_works() {
    //     let endpoint = ListDocuments::builder().page(1).build().unwrap();
    //     // client with authentication token
    //     let mut headers = reqwest::header::HeaderMap::new();
    //     headers.insert(reqwest::header::AUTHORIZATION, API_TOKEN.parse().unwrap());
    //     headers.insert(
    //         reqwest::header::CONTENT_TYPE,
    //         "application/json".parse().unwrap(),
    //     );
    //     let req_client = reqwest::Client::builder()
    //         .default_headers(headers)
    //         .build()
    //         .unwrap();
    //
    //     let client = Client::new(BASE_URL, req_client); // Configures base address of http://api.com
    //
    //     let result = endpoint.exec(&client).await; // Sends GET request to http://api.com/test/path
    //     match result {
    //         Ok(r) => match r.wrap::<ListDocumentsResponseWrapper<_>>() {
    //             Ok(d) => {
    //                 d.results.iter().for_each(|d| println!("{:#?}", d));
    //             }
    //             Err(ClientError::ResponseParseError { source, content }) => {
    //                 println!("{}: {}", source, content.unwrap_or("".into()))
    //             }
    //             r => println!("{:#?}", r),
    //         },
    //         Err(e) => match e {
    //             _ => println!("{:#?}", e),
    //         },
    //     };
    // }
}
