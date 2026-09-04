use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Serialize)]
pub struct TranslateRequest {
    pub payload: TranslatePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TranslatePayload {
    Label { payload: LabelPayload },
    Query { payload: QueryPayload },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPayload {
    pub label_id: u64,
    pub label_membership: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPayload {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslateResponse {
    translated_query: String,
}

impl TranslateResponse {
    pub fn translated_query(&self) -> &str {
        &self.translated_query
    }
}
