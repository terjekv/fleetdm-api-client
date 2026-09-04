use crate::client::FleetClient;
use crate::error::Result;
use crate::models::carve::{GetCarveBlockResponse, GetCarveResponse, ListCarvesResponse};
use crate::paths;

pub struct CarvesEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> CarvesEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// List all carves
    pub async fn list(&self) -> Result<ListCarvesResponse> {
        let request = self.client.request(reqwest::Method::GET, paths::CARVES)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get a specific carve by ID
    pub async fn get(&self, id: u64) -> Result<GetCarveResponse> {
        let path = paths::carve(id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get a specific carve block
    pub async fn get_block(&self, carve_id: u64, block_id: u64) -> Result<GetCarveBlockResponse> {
        let path = paths::carve_block(carve_id, block_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
