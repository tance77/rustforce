use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub id: String,
    pub issued_at: String,
    pub access_token: String,
    pub instance_url: String,
    pub signature: String,
    pub token_type: Option<String>,
}

impl Default for TokenResponse {
    fn default() -> Self {
        TokenResponse {
            id: "".to_string(),
            issued_at: "".to_string(),
            access_token: "".to_string(),
            instance_url: "".to_string(),
            signature: "".to_string(),
            token_type: None,
        }
    }
}
