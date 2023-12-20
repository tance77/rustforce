use serde::{Deserialize, Serialize};
use crate::responses::describe_global_sobject_response::DescribeGlobalSObjectResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DescribeGlobalResponse {
    pub encoding: String,
    pub max_batch_size: u16,
    pub sobjects: Vec<DescribeGlobalSObjectResponse>,
}