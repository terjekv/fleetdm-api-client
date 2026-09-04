use crate::client::FleetClient;
use crate::error::Result;
use crate::models::policy::*;

/// Endpoint for policy-related operations
pub struct PoliciesEndpoint {
    client: FleetClient,
}

impl PoliciesEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List policies with optional filters
    pub fn list(&self) -> ListPoliciesBuilder<'_> {
        ListPoliciesBuilder {
            client: &self.client,
            query: ListPoliciesQuery::new(),
        }
    }

    /// Get a specific policy by ID
    pub async fn get(&self, policy_id: u64) -> Result<GetPolicyResponse> {
        let path = crate::paths::policy(policy_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a new policy
    pub async fn create(&self, req: CreatePolicyRequest) -> Result<GetPolicyResponse> {
        req.validate()?;
        let path = crate::paths::POLICIES;
        let request = self.client.request(reqwest::Method::POST, path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Update an existing policy
    pub async fn update(
        &self,
        policy_id: u64,
        req: UpdatePolicyRequest,
    ) -> Result<GetPolicyResponse> {
        req.validate()?;
        let path = crate::paths::policy(policy_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Delete a policy by ID
    pub async fn delete(&self, policy_id: u64) -> Result<()> {
        let path = format!("{}/delete", crate::paths::POLICIES);
        let body = serde_json::json!({
            "ids": [policy_id]
        });
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_empty_request(request.json(&body), self.client.retry_policy()).await
    }

    // --- Team Policy Methods ---

    /// List policies for a specific team
    pub fn list_team(&self, team_id: u64) -> ListTeamPoliciesBuilder<'_> {
        ListTeamPoliciesBuilder {
            client: &self.client,
            team_id,
            query: ListPoliciesQuery::new(),
            merge_inherited: None,
        }
    }

    /// Get a team policy by ID
    pub async fn get_team(&self, team_id: u64, policy_id: u64) -> Result<GetPolicyResponse> {
        let path = crate::paths::team_policy(team_id, policy_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a policy for a team
    pub async fn create_team(
        &self,
        team_id: u64,
        req: CreatePolicyRequest,
    ) -> Result<GetPolicyResponse> {
        req.validate()?;
        let path = crate::paths::team_policies(team_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Update a team policy
    pub async fn update_team(
        &self,
        team_id: u64,
        policy_id: u64,
        req: UpdatePolicyRequest,
    ) -> Result<GetPolicyResponse> {
        req.validate()?;
        let path = crate::paths::team_policy(team_id, policy_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Delete team policies by IDs
    pub async fn delete_team(&self, team_id: u64, policy_ids: Vec<u64>) -> Result<()> {
        if policy_ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "policy IDs must not be empty".into(),
            ));
        }
        let path = crate::paths::team_policies_delete(team_id);
        let body = serde_json::json!({ "ids": policy_ids });
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_empty_request(request.json(&body), self.client.retry_policy()).await
    }
}

/// Builder for constructing list policies queries
pub struct ListPoliciesBuilder<'a> {
    client: &'a FleetClient,
    query: ListPoliciesQuery,
}

impl<'a> ListPoliciesBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::policy::PolicyOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }

    pub fn team_id(mut self, team_id: u64) -> Self {
        self.query = self.query.team_id(team_id);
        self
    }

    pub fn fleet_id(mut self, fleet_id: u64) -> Self {
        self.query = self.query.fleet_id(fleet_id);
        self
    }

    /// Execute the list policies query
    pub async fn send(self) -> Result<ListPoliciesResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::POLICIES, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for listing team policies
pub struct ListTeamPoliciesBuilder<'a> {
    client: &'a FleetClient,
    team_id: u64,
    query: ListPoliciesQuery,
    merge_inherited: Option<bool>,
}

impl<'a> ListTeamPoliciesBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::policy::PolicyOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }

    /// If true, inherited (global) policies are included in the response
    pub fn merge_inherited(mut self, merge: bool) -> Self {
        self.merge_inherited = Some(merge);
        self
    }

    /// Execute the list team policies query
    pub async fn send(self) -> Result<ListPoliciesResponse> {
        self.query.validate()?;
        let path = crate::paths::team_policies(self.team_id);
        let mut params = self.query.to_query_params();
        if let Some(merge) = self.merge_inherited {
            params.push(("merge_inherited".to_string(), merge.to_string()));
        }

        let path = crate::http::append_query_params(&path, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_policies_query_builder() {
        let query = ListPoliciesQuery::new().page(1).per_page(25).team_id(456);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "25".to_string())));
        assert!(params.contains(&("team_id".to_string(), "456".to_string())));
    }
}
