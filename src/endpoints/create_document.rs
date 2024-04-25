use crate::api::create_document;
use rustify::{client::Client, errors::ClientError};

pub async fn create_document(
    client: &impl Client,
    doc_builder: &mut create_document::request::RequestBuilder,
) -> Result<(), ClientError> {
    let endpoint = doc_builder.build().unwrap();

    Ok(())
}
