use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterResponse {
    pub client_id: String,
    pub client_secret: String,
    pub registration_access_token: String,
    pub registration_client_uri: String,
    pub client_id_issued_at: u64,
    pub client_secret_expires_at: u64,
    pub token_endpoint_auth_method: String,
    pub redirect_uris: Option<Vec<String>>,
    pub response_types: Option<Vec<String>>,
    pub grant_types: Option<Vec<String>>,
    pub application_type: Option<String>,
    pub client_name: Option<String>,
    pub logo_uri: Option<String>,
    pub subject_type: Option<String>
}