use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub search_records: Vec<SearchRecord>,
    //    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchRecord {
    #[serde(rename = "Id")]
    pub id: String,
    pub attributes: SObjectAttribute,
}


#[derive(serde::Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SObjectAttribute {
    #[serde(rename = "type")]
    pub sobject_type: String,
    pub url: String,
}