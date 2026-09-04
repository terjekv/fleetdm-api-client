use crate::client::FleetClient;
use crate::error::Result;
use crate::models::config::{EnrollSecretsResponse, GetConfigResponse, ModifyEnrollSecretsRequest};
use crate::paths;

pub struct ConfigEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> ConfigEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Get the Fleet configuration
    pub async fn get(&self) -> Result<GetConfigResponse> {
        let request = self.client.request(reqwest::Method::GET, paths::CONFIG)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Update the Fleet configuration
    pub async fn update(&self, config: &serde_json::Value) -> Result<GetConfigResponse> {
        if config.as_object().is_none_or(serde_json::Map::is_empty) {
            return Err(crate::FleetError::Validation(
                "configuration update must be a non-empty JSON object".into(),
            ));
        }
        let request = self.client.request(reqwest::Method::PATCH, paths::CONFIG)?;
        crate::http::send_request(request.json(config), self.client.retry_policy()).await
    }

    /// Get enroll secrets
    pub async fn get_enroll_secrets(&self) -> Result<EnrollSecretsResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::ENROLL_SECRETS_SPEC)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Modify enroll secrets
    pub async fn modify_enroll_secrets(
        &self,
        request_data: &ModifyEnrollSecretsRequest,
    ) -> Result<EnrollSecretsResponse> {
        request_data.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::ENROLL_SECRETS_SPEC)?;
        crate::http::send_request(request.json(request_data), self.client.retry_policy()).await
    }
}
