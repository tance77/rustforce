use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobStatusResponse {
    pub apex_processing_time: Option<u64>, // Use Option if the field can be absent
    pub api_active_processing_time: Option<u64>, // Use Option if the field can be absent
    pub api_version: Option<f64>,
    pub assignment_rule_id: Option<String>, // Use Option if the field can be absent
    pub column_delimiter: Option<String>,
    pub concurrency_mode: String,
    pub content_type: String,
    pub content_url: Option<String>,
    pub created_by_id: String,
    pub created_date: String, // Use chrono::NaiveDateTime if you want to handle dates and times
    pub error_message: Option<String>, // Use Option if the field can be null
    pub external_id_field_name: Option<String>, // Use Option if the field can be absent
    pub id: String,
    pub job_type: Option<String>,
    pub line_ending: String,
    pub number_records_failed: Option<u64>,
    pub number_records_processed: Option<u64>,
    pub object: String,
    pub operation: String,
    pub retries: Option<u64>, // Use Option if the field can be absent
    pub state: String,
    pub system_modstamp: String, // Use chrono::NaiveDateTime if you want to handle dates and times
    pub total_processing_time: Option<u64>, // Use Option if the field can be absent
}