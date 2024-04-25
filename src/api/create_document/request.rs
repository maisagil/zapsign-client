use derive_builder::Builder;
use rustify_derive::Endpoint;

#[derive(Builder, Endpoint)]
#[endpoint(path = "docs/", method = "POST", builder = "true")]
#[builder(setter(into))] // Improves the building process
pub struct Request {
    #[endpoint(body)]
    pub name: String,
    #[endpoint(body)]
    pub url_pdf: Option<String>,
    #[endpoint(body)]
    pub base64_pdf: Option<String>,
    #[endpoint(body)]
    pub signers: Vec<String>,
    #[endpoint(body)]
    url_docx: Option<String>,
    #[endpoint(body)]
    base64_docx: Option<String>,
    #[endpoint(body)]
    lang: String,
    #[endpoint(body)]
    disable_signer_emails: bool,
    #[endpoint(body)]
    signed_file_only_finished: bool,
    #[endpoint(body)]
    brand_logo: String,
    #[endpoint(body)]
    brand_primary_color: String,
    #[endpoint(body)]
    brand_name: String,
    #[endpoint(body)]
    external_id: String,
    #[endpoint(body)]
    folder_path: String,
    #[endpoint(body)]
    created_by: String,
    #[endpoint(body)]
    date_limit_to_sign: String,
    #[endpoint(body)]
    signature_order_active: bool,
    #[endpoint(body)]
    observers: Vec<String>,
    #[endpoint(body)]
    reminder_every_n_days: i32,
    #[endpoint(body)]
    allow_refuse_signature: bool,
    #[endpoint(body)]
    disable_signers_get_original_file: bool,
}
