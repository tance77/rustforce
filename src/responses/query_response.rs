use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse<T> {
    pub total_size: i32,
    pub done: bool,
    pub records: Vec<T>,
    pub next_records_url: Option<String>,
}
