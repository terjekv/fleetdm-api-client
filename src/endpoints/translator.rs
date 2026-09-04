use crate::client::FleetClient;
use crate::error::Result;
use crate::models::translator::{TranslateRequest, TranslateResponse};
use crate::paths;

pub struct TranslatorEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> TranslatorEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Translate a query or other content
    pub async fn translate(&self, request: &TranslateRequest) -> Result<TranslateResponse> {
        let request_builder = self
            .client
            .request(reqwest::Method::POST, paths::TRANSLATOR)?;
        crate::http::send_request(request_builder.json(request), self.client.retry_policy()).await
    }
}
