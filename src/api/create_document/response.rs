use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(rename = "open_id")]
    pub open_id: i64,
    pub token: String,
    pub status: String,
    pub name: String,
    #[serde(rename = "original_file")]
    pub original_file: String,
    #[serde(rename = "signed_file")]
    pub signed_file: Value,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_update_at")]
    pub last_update_at: String,
    pub signers: Vec<Signer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signer {
    pub token: String,
    #[serde(rename = "sign_url")]
    pub sign_url: String,
    pub status: String,
    pub name: String,
    pub email: String,
    #[serde(rename = "phone_country")]
    pub phone_country: String,
    #[serde(rename = "phone_number")]
    pub phone_number: String,
    #[serde(rename = "times_viewed")]
    pub times_viewed: i64,
    #[serde(rename = "last_view_at")]
    pub last_view_at: Value,
    #[serde(rename = "signed_at")]
    pub signed_at: Value,
}
