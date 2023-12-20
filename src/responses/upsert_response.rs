use serde::{Deserialize, Serialize};
use crate::responses::create_response::CreateResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpsertResponse {
    create: Option<CreateResponse>,
}