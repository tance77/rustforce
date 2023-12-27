use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkApiStateChangeResponse {
    pub id: String,
    pub operation: String,
    pub object: String,
    pub created_by_id: String,
    pub created_date: String,
    pub system_modstamp: String,
    pub state: String,
    pub concurrency_mode: String,
    pub content_type: String,
    pub api_version: f64
}