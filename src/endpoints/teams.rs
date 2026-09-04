use crate::client::FleetClient;
use crate::error::Result;
use crate::models::team::*;

/// Endpoint for team-related operations
pub struct TeamsEndpoint {
    client: FleetClient,
}

impl TeamsEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List teams with optional filters
    pub fn list(&self) -> ListTeamsBuilder<'_> {
        ListTeamsBuilder {
            client: &self.client,
            query: ListTeamsQuery::new(),
        }
    }

    /// Get a specific team by ID
    pub async fn get(&self, team_id: u64) -> Result<GetTeamResponse> {
        let path = crate::paths::team(team_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a new team
    pub async fn create(&self, req: CreateTeamRequest) -> Result<GetTeamResponse> {
        req.validate()?;
        let path = crate::paths::TEAMS;
        let request = self.client.request(reqwest::Method::POST, path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Update an existing team
    pub async fn update(&self, team_id: u64, req: UpdateTeamRequest) -> Result<GetTeamResponse> {
        req.validate()?;
        let path = crate::paths::team(team_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Delete a team by ID
    pub async fn delete(&self, team_id: u64) -> Result<DeleteTeamResponse> {
        let path = crate::paths::team(team_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list teams queries
pub struct ListTeamsBuilder<'a> {
    client: &'a FleetClient,
    query: ListTeamsQuery,
}

impl<'a> ListTeamsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::team::TeamOrderKey) -> Self {
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

    /// Execute the list teams query
    pub async fn send(self) -> Result<ListTeamsResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::TEAMS, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_teams_query_builder() {
        let query = ListTeamsQuery::new()
            .page(1)
            .per_page(20)
            .query("Engineering");

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "20".to_string())));
        assert!(params.contains(&("query".to_string(), "Engineering".to_string())));
    }
}
