use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    version: String,
    branch: String,
    revision: String,
    go_version: String,
    build_date: String,
    build_user: String,
}

impl VersionResponse {
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn branch(&self) -> &str {
        &self.branch
    }
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn go_version(&self) -> &str {
        &self.go_version
    }
    pub fn build_date(&self) -> &str {
        &self.build_date
    }
    pub fn build_user(&self) -> &str {
        &self.build_user
    }
}
