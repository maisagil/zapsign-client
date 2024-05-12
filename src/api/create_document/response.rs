use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub open_id: i64,
    pub token: String,
    pub status: String,
    pub name: String,
    pub original_file: String,
    pub signed_file: Value,
    pub created_at: String,
    pub last_update_at: String,
    pub signers: Vec<Signer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signer {
    pub token: String,
    pub sign_url: String,
    pub status: String,
    pub name: String,
    pub email: String,
    pub phone_country: String,
    pub phone_number: String,
    pub times_viewed: i64,
    pub last_view_at: Value,
    pub signed_at: Value,
}
