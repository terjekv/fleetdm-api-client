use crate::client::FleetClient;
use crate::error::Result;
use crate::models::target::{SearchTargetsRequest, SearchTargetsResponse};
use crate::paths;

pub struct TargetsEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> TargetsEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Search for query targets (hosts, labels, teams)
    pub async fn search(&self, request: &SearchTargetsRequest) -> Result<SearchTargetsResponse> {
        let request_builder = self.client.request(reqwest::Method::POST, paths::TARGETS)?;
        crate::http::send_request(request_builder.json(request), self.client.retry_policy()).await
    }
}
