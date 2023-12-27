use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    pub id: String,
    pub success: bool,
}