use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSessionResponse {
    #[serde(default)]
    message: String,
}

impl DeleteSessionResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}
