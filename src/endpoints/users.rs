use crate::client::FleetClient;
use crate::error::Result;
use crate::models::user::*;

/// Endpoint for user-related operations
pub struct UsersEndpoint {
    client: FleetClient,
}

impl UsersEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List users with optional filters
    pub fn list(&self) -> ListUsersBuilder<'_> {
        ListUsersBuilder {
            client: &self.client,
            query: ListUsersQuery::new(),
        }
    }

    /// Get a specific user by ID
    pub async fn get(&self, user_id: u64) -> Result<GetUserResponse> {
        let path = crate::paths::user(user_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a new user
    pub async fn create(&self, req: CreateUserRequest) -> Result<GetUserResponse> {
        req.validate()?;
        let path = crate::paths::USERS_ADMIN;
        let request = self.client.request(reqwest::Method::POST, path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Update an existing user
    pub async fn update(&self, user_id: u64, req: UpdateUserRequest) -> Result<GetUserResponse> {
        req.validate()?;
        let path = crate::paths::user(user_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Delete a user by ID
    pub async fn delete(&self, user_id: u64) -> Result<DeleteUserResponse> {
        let path = crate::paths::user(user_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list users queries
pub struct ListUsersBuilder<'a> {
    client: &'a FleetClient,
    query: ListUsersQuery,
}

impl<'a> ListUsersBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::user::UserOrderKey) -> Self {
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

    pub fn team_id(mut self, team_id: u64) -> Self {
        self.query = self.query.team_id(team_id);
        self
    }

    pub fn fleet_id(mut self, fleet_id: u64) -> Self {
        self.query = self.query.fleet_id(fleet_id);
        self
    }

    /// Execute the list users query
    pub async fn send(self) -> Result<ListUsersResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::USERS, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_users_query_builder() {
        let query = ListUsersQuery::new().page(1).per_page(50).team_id(10);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "50".to_string())));
        assert!(params.contains(&("team_id".to_string(), "10".to_string())));
    }
}
