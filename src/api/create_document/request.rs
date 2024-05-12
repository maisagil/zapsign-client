use super::response::Response;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};

// Improves the building process
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Endpoint)]
#[builder(setter(into), default)]
#[endpoint(
    // zapsign requires the / at the end.
    path = "docs/",
    method = "POST",
    builder = "true",
    response = "Response"
)]
pub struct Request {
    pub name: String,
    pub url_pdf: String,
    pub external_id: Option<String>,
    pub signers: Vec<Signer>,
    pub lang: String,
    pub disable_signer_emails: bool,
    pub signed_file_only_finished: bool,
    pub brand_logo: String,
    pub brand_primary_color: String,
    pub brand_name: String,
    pub folder_path: String,
    pub created_by: String,
    pub date_limit_to_sign: Option<String>,
    pub signature_order_active: bool,
    pub observers: Vec<String>,
    pub reminder_every_n_days: i64,
    pub allow_refuse_signature: bool,
    pub disable_signers_get_original_file: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(into), default)]
pub struct Signer {
    pub name: String,
    pub email: Option<String>,
    pub auth_mode: Option<String>,
    pub send_automatic_email: Option<bool>,
    pub send_automatic_whatsapp: Option<bool>,
    pub order_group: Option<String>,
    pub custom_message: Option<String>,
    pub phone_country: Option<String>,
    pub phone_number: Option<String>,
    pub lock_email: Option<bool>,
    pub blank_email: Option<bool>,
    pub hide_email: Option<bool>,
    pub lock_phone: Option<bool>,
    pub blank_phone: Option<bool>,
    pub hide_phone: Option<bool>,
    pub lock_name: Option<bool>,
    pub require_cpf: Option<bool>,
    pub cpf: Option<String>,
    pub require_selfie_photo: Option<bool>,
    pub require_document_photo: Option<bool>,
    pub selfie_validation_type: Option<String>,
    pub selfie_photo_url: Option<String>,
    pub document_photo_url: Option<String>,
    pub document_verse_photo_url: Option<String>,
    pub qualification: Option<String>,
    pub external_id: Option<String>,
    pub redirect_link: Option<String>,
}



