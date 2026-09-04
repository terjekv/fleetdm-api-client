use crate::client::FleetClient;
use crate::error::Result;
use crate::models::version::VersionResponse;

/// Endpoint for retrieving Fleet server version information
pub struct VersionEndpoint {
    client: FleetClient,
}

impl VersionEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// Get the Fleet server version
    pub async fn get(&self) -> Result<VersionResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, crate::paths::VERSION)?;

        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
