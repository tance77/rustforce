use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkApiV1CreateResponse {
    pub apex_processing_time: Option<f64>,
    pub api_active_processing_time: Option<f64>,
    pub api_version: Option<f64>,
    pub concurrency_mode: Option<String>,
    pub content_type: Option<String>,
    pub created_by_id: Option<String>,
    pub created_date: Option<String>,
    pub id: Option<String>,
    pub number_batches_completed: Option<i64>,
    pub number_batches_failed: Option<i64>,
    pub number_batches_in_progress: Option<i64>,
    pub number_batches_queued: Option<i64>,
    pub number_batches_total: Option<i64>,
    pub number_records_failed: Option<i64>,
    pub number_records_processed: Option<i64>,
    pub number_retries: Option<i64>,
    pub object: Option<String>,
    pub operation: Option<String>,
    pub state: Option<String>,
    pub system_modstamp: Option<String>,
    pub total_processing_time: Option<f64>,
}