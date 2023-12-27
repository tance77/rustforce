use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkApiStatusResponse {
    pub id: String,
    pub operation: String,
    pub object: String,
    pub created_by_id: String,
    pub created_date: String,
    pub system_modstamp: String,
    pub state: String,
    pub concurrency_mode: String,
    pub content_type: String,
    pub api_version: f64,
    pub job_type: Option<String>,
    pub line_ending: Option<String>,
    pub column_delimiter: Option<String>,
    pub number_records_processed: Option<i64>,
    pub number_records_failed: Option<i64>,
    pub retries: Option<i64>,
    pub total_processing_time: Option<i64>,
    pub api_active_processing_time: Option<i64>,
    pub apex_processing_time: Option<i64>,
}