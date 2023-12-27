#[derive(Debug, Clone)]
pub struct AccessToken {
    pub token_type: String,
    pub value: String,
    pub issued_at: String,
}

impl AccessToken {
    pub fn default() -> AccessToken {
        AccessToken {
            token_type: "".to_string(),
            value: "".to_string(),
            issued_at: "".to_string(),
        }
    }
}
