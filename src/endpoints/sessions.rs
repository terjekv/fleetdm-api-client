use crate::client::FleetClient;
use crate::error::Result;
use crate::models::session::DeleteSessionResponse;
use crate::paths;

pub struct SessionsEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> SessionsEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Delete a session by ID
    pub async fn delete(&self, id: u64) -> Result<DeleteSessionResponse> {
        let path = paths::session(id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
