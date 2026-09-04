use crate::client::FleetClient;
use crate::error::Result;
use crate::http::handle_bytes_response;
use crate::paths;

/// Conditional access integration endpoints.
pub struct ConditionalAccessEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> ConditionalAccessEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// GET /api/v1/fleet/conditional_access/idp/signing_cert
    pub async fn okta_signing_certificate(&self) -> Result<Vec<u8>> {
        let request = self.client.request(
            reqwest::Method::GET,
            paths::CONDITIONAL_ACCESS_IDP_SIGNING_CERT,
        )?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        handle_bytes_response(response).await
    }

    /// GET /api/v1/fleet/conditional_access/idp/apple/profile
    pub async fn okta_apple_profile(&self) -> Result<Vec<u8>> {
        let request = self.client.request(
            reqwest::Method::GET,
            paths::CONDITIONAL_ACCESS_IDP_APPLE_PROFILE,
        )?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        handle_bytes_response(response).await
    }

    /// DELETE /api/v1/conditional-access/microsoft
    pub async fn disconnect_microsoft_entra(&self) -> Result<()> {
        let request = self
            .client
            .request(reqwest::Method::DELETE, paths::CONDITIONAL_ACCESS_MICROSOFT)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }
}
