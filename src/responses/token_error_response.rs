use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenErrorResponse {
    error: String,
    error_description: String,
}