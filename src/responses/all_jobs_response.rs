use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllJobsStatusResponse {
    pub records: Vec<JobInfo>,
    pub done: bool,
    pub next_records_url: Option<String>,
}

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobInfo {
    pub id: String,
    pub job_type: String,
    pub api_version: f64,
    pub created_date: String,
    pub column_delimiter: Option<String>,
    pub concurrency_mode: String,
    pub content_type: String,
    pub line_ending: Option<String>,
    pub object: String,
    pub operation: String,
    pub state: String,
    pub system_modstamp: String
}