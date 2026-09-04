use serde::{Deserialize, Serialize};

use crate::models::common::PaginationMeta;

// Run MDM command
#[derive(Debug, Serialize)]
pub struct RunCommandRequest {
    pub command: String,
    pub host_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RunCommandResponse {
    pub command_uuid: String,
    pub request_type: String,
    pub status: String,
    pub host_ids: Vec<u64>,
}

// Get MDM command results
#[derive(Debug, Deserialize)]
pub struct CommandResultsResponse {
    pub results: Vec<CommandResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_uuid: String,
    pub request_type: String,
    pub status: String,
    pub hostname: String,
    pub host_id: u64,
    pub updated_at: String,
    pub result: Option<CommandResultDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResultDetail {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

// List MDM commands
#[derive(Debug, Deserialize)]
pub struct ListCommandsResponse {
    pub commands: Vec<Command>,
    #[serde(default)]
    pub meta: Option<PaginationMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub command_uuid: String,
    pub request_type: String,
    pub status: String,
    pub hostname: String,
    pub host_id: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[deprecated(note = "use PaginationMeta")]
pub type ListMeta = PaginationMeta;

impl RunCommandResponse {
    pub fn command_uuid(&self) -> &str {
        &self.command_uuid
    }

    pub fn request_type(&self) -> &str {
        &self.request_type
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn host_ids(&self) -> &[u64] {
        &self.host_ids
    }
}

impl CommandResult {
    pub fn command_uuid(&self) -> &str {
        &self.command_uuid
    }

    pub fn request_type(&self) -> &str {
        &self.request_type
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn host_id(&self) -> u64 {
        self.host_id
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn result(&self) -> Option<&CommandResultDetail> {
        self.result.as_ref()
    }
}

impl Command {
    pub fn command_uuid(&self) -> &str {
        &self.command_uuid
    }

    pub fn request_type(&self) -> &str {
        &self.request_type
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn host_id(&self) -> u64 {
        self.host_id
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl ListCommandsResponse {
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

impl CommandResultsResponse {
    pub fn results(&self) -> &[CommandResult] {
        &self.results
    }
}
