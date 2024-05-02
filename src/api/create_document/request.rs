use super::response::Response;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};

// Improves the building process
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Endpoint)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into), default)]
#[endpoint(
    path = "docs",
    method = "POST",
    builder = "true",
    response = "Response"
)]
pub struct Request {
    pub name: String,
    #[serde(rename = "url_pdf")]
    pub url_pdf: String,
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    pub signers: Vec<Signer>,
    pub lang: String,
    #[serde(rename = "disable_signer_emails")]
    pub disable_signer_emails: bool,
    #[serde(rename = "signed_file_only_finished")]
    pub signed_file_only_finished: bool,
    #[serde(rename = "brand_logo")]
    pub brand_logo: String,
    #[serde(rename = "brand_primary_color")]
    pub brand_primary_color: String,
    #[serde(rename = "brand_name")]
    pub brand_name: String,
    #[serde(rename = "folder_path")]
    pub folder_path: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "date_limit_to_sign")]
    pub date_limit_to_sign: Option<String>,
    #[serde(rename = "signature_order_active")]
    pub signature_order_active: bool,
    pub observers: Vec<String>,
    #[serde(rename = "reminder_every_n_days")]
    pub reminder_every_n_days: i64,
    #[serde(rename = "allow_refuse_signature")]
    pub allow_refuse_signature: bool,
    #[serde(rename = "disable_signers_get_original_file")]
    pub disable_signers_get_original_file: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(into), default)]
#[serde(rename_all = "camelCase")]
pub struct Signer {
    pub name: String,
    pub email: Option<String>,
    #[serde(rename = "auth_mode")]
    pub auth_mode: Option<String>,
    #[serde(rename = "send_automatic_email")]
    pub send_automatic_email: Option<bool>,
    #[serde(rename = "send_automatic_whatsapp")]
    pub send_automatic_whatsapp: Option<bool>,
    #[serde(rename = "order_group")]
    pub order_group: Option<String>,
    #[serde(rename = "custom_message")]
    pub custom_message: Option<String>,
    #[serde(rename = "phone_country")]
    pub phone_country: Option<String>,
    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,
    #[serde(rename = "lock_email")]
    pub lock_email: Option<bool>,
    #[serde(rename = "blank_email")]
    pub blank_email: Option<bool>,
    #[serde(rename = "hide_email")]
    pub hide_email: Option<bool>,
    #[serde(rename = "lock_phone")]
    pub lock_phone: Option<bool>,
    #[serde(rename = "blank_phone")]
    pub blank_phone: Option<bool>,
    #[serde(rename = "hide_phone")]
    pub hide_phone: Option<bool>,
    #[serde(rename = "lock_name")]
    pub lock_name: Option<bool>,
    #[serde(rename = "require_cpf")]
    pub require_cpf: Option<bool>,
    pub cpf: Option<String>,
    #[serde(rename = "require_selfie_photo")]
    pub require_selfie_photo: Option<bool>,
    #[serde(rename = "require_document_photo")]
    pub require_document_photo: Option<bool>,
    #[serde(rename = "selfie_validation_type")]
    pub selfie_validation_type: Option<String>,
    #[serde(rename = "selfie_photo_url")]
    pub selfie_photo_url: Option<String>,
    #[serde(rename = "document_photo_url")]
    pub document_photo_url: Option<String>,
    #[serde(rename = "document_verse_photo_url")]
    pub document_verse_photo_url: Option<String>,
    pub qualification: Option<String>,
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    #[serde(rename = "redirect_link")]
    pub redirect_link: Option<String>,
}
