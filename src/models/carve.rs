use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Carve {
    id: u64,
    created_at: String,
    host_id: u64,
    name: String,
    block_count: u32,
    block_size: u64,
    carve_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    carve_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_block: Option<u32>,
}

impl Carve {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn block_count(&self) -> u32 {
        self.block_count
    }
    pub fn block_size(&self) -> u64 {
        self.block_size
    }
    pub fn carve_size(&self) -> u64 {
        self.carve_size
    }
    pub fn carve_id(&self) -> Option<&str> {
        self.carve_id.as_deref()
    }
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
    pub fn expired(&self) -> Option<bool> {
        self.expired
    }
    pub fn max_block(&self) -> Option<u32> {
        self.max_block
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCarvesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    carves: Option<Vec<Carve>>,
}

impl ListCarvesResponse {
    pub fn carves(&self) -> Option<&[Carve]> {
        self.carves.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCarveResponse {
    carve: Carve,
}

impl GetCarveResponse {
    pub fn carve(&self) -> &Carve {
        &self.carve
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCarveBlockResponse {
    block_id: u64,
    data: String,
}

impl GetCarveBlockResponse {
    pub fn block_id(&self) -> u64 {
        self.block_id
    }
    pub fn data(&self) -> &str {
        &self.data
    }
}
