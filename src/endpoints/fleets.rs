use crate::client::FleetClient;
use crate::error::Result;
use crate::models::fleet::*;
use crate::models::policy::{
    CreatePolicyRequest, GetPolicyResponse, ListPoliciesResponse, UpdatePolicyRequest,
};

/// Endpoint for fleet-related operations.
pub struct FleetsEndpoint {
    client: FleetClient,
}

impl FleetsEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// `GET /api/v1/fleet/fleets`
    pub fn list(&self) -> ListFleetsBuilder<'_> {
        ListFleetsBuilder {
            client: &self.client,
            query: ListFleetsQuery::new(),
        }
    }

    /// `GET /api/v1/fleet/fleets/:id`
    pub async fn get(&self, fleet_id: u64) -> Result<GetFleetResponse> {
        let path = crate::paths::fleet(fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/fleets`
    pub async fn create(&self, req: CreateFleetRequest) -> Result<GetFleetResponse> {
        req.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, crate::paths::FLEETS)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// `PATCH /api/v1/fleet/fleets/:id`
    pub async fn update(&self, fleet_id: u64, req: UpdateFleetRequest) -> Result<GetFleetResponse> {
        req.validate()?;
        let path = crate::paths::fleet(fleet_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// `DELETE /api/v1/fleet/fleets/:id`
    pub async fn delete(&self, fleet_id: u64) -> Result<()> {
        let path = crate::paths::fleet(fleet_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// `GET /api/v1/fleet/fleets/:id/secrets`
    pub async fn secrets(&self, fleet_id: u64) -> Result<FleetSecretsResponse> {
        let path = crate::paths::fleet_secrets(fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `PATCH /api/v1/fleet/fleets/:id/secrets`
    pub async fn update_secrets(
        &self,
        fleet_id: u64,
        body: UpdateFleetSecretsRequest,
    ) -> Result<FleetSecretsResponse> {
        body.validate()?;
        let path = crate::paths::fleet_secrets(fleet_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&body), self.client.retry_policy()).await
    }

    /// `PATCH /api/v1/fleet/fleets/:id/users`
    pub async fn update_users(
        &self,
        fleet_id: u64,
        body: UpdateFleetUsersRequest,
    ) -> Result<GetFleetResponse> {
        body.validate()?;
        let path = crate::paths::fleet_users(fleet_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&body), self.client.retry_policy()).await
    }

    /// `GET /api/v1/fleet/fleets/:id/policies`
    pub async fn policies(&self, fleet_id: u64) -> Result<ListPoliciesResponse> {
        self.policies_with_options(fleet_id, None).await
    }

    pub async fn policies_with_options(
        &self,
        fleet_id: u64,
        merge_inherited: Option<bool>,
    ) -> Result<ListPoliciesResponse> {
        let path = crate::paths::fleet_policies(fleet_id);
        let params = merge_inherited
            .map(|merge| vec![("merge_inherited".into(), merge.to_string())])
            .unwrap_or_default();
        let path = crate::http::append_query_params(&path, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `GET /api/v1/fleet/fleets/:fleet_id/policies/:policy_id`
    pub async fn policy(&self, fleet_id: u64, policy_id: u64) -> Result<GetPolicyResponse> {
        let path = crate::paths::fleet_policy(fleet_id, policy_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/fleets/:id/policies`
    pub async fn create_policy(
        &self,
        fleet_id: u64,
        body: CreatePolicyRequest,
    ) -> Result<GetPolicyResponse> {
        body.validate()?;
        if body.team_id.is_some_and(|id| id != fleet_id) {
            return Err(crate::FleetError::Validation(
                "policy team_id must match the fleet path".into(),
            ));
        }
        let path = crate::paths::fleet_policies(fleet_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request.json(&body), self.client.retry_policy()).await
    }

    /// `PATCH /api/v1/fleet/fleets/:fleet_id/policies/:policy_id`
    pub async fn update_policy(
        &self,
        fleet_id: u64,
        policy_id: u64,
        body: UpdatePolicyRequest,
    ) -> Result<GetPolicyResponse> {
        body.validate()?;
        if body.team_id.is_some_and(|id| id != fleet_id) {
            return Err(crate::FleetError::Validation(
                "policy team_id must match the fleet path".into(),
            ));
        }
        let path = crate::paths::fleet_policy(fleet_id, policy_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&body), self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/fleets/:fleet_id/policies/delete`
    pub async fn delete_policies(
        &self,
        fleet_id: u64,
        ids: Vec<u64>,
    ) -> Result<DeletedFleetPoliciesResponse> {
        if ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "policy IDs must not be empty".into(),
            ));
        }
        let path = crate::paths::fleet_policies_delete(fleet_id);
        let body = serde_json::json!({ "ids": ids });
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request.json(&body), self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/fleets/:id/agent_options`
    pub async fn update_agent_options(
        &self,
        fleet_id: u64,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.update_agent_options_with_options(fleet_id, body, None, None)
            .await
    }

    pub async fn update_agent_options_with_options(
        &self,
        fleet_id: u64,
        body: serde_json::Value,
        force: Option<bool>,
        dry_run: Option<bool>,
    ) -> Result<serde_json::Value> {
        let path = crate::paths::fleet_agent_options(fleet_id);
        let mut params = Vec::new();
        if let Some(force) = force {
            params.push(("force".into(), force.to_string()));
        }
        if let Some(dry_run) = dry_run {
            params.push(("dry_run".into(), dry_run.to_string()));
        }
        let path = crate::http::append_query_params(&path, &params);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request.json(&body), self.client.retry_policy()).await
    }
}

/// Builder for constructing list fleets queries.
pub struct ListFleetsBuilder<'a> {
    client: &'a FleetClient,
    query: ListFleetsQuery,
}

impl<'a> ListFleetsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }
    pub fn order_key(mut self, key: crate::models::fleet::FleetOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }
    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = self.query.query(query);
        self
    }
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.query = self.query.after(after);
        self
    }

    pub async fn send(self) -> Result<ListFleetsResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::FLEETS, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_fleets_query_builder() {
        let query = ListFleetsQuery::new()
            .page(1)
            .per_page(20)
            .query("Engineering");
        let params = query.to_query_params();
        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "20".to_string())));
        assert!(params.contains(&("query".to_string(), "Engineering".to_string())));
    }
}
