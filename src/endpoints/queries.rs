use crate::client::FleetClient;
use crate::error::Result;
use crate::models::query::*;

/// Endpoint for query-related operations
pub struct QueriesEndpoint {
    client: FleetClient,
}

impl QueriesEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List queries with optional filters
    pub fn list(&self) -> ListQueriesBuilder<'_> {
        ListQueriesBuilder {
            client: &self.client,
            query: ListQueriesQuery::new(),
        }
    }

    /// Get a specific query by ID
    pub async fn get(&self, query_id: u64) -> Result<GetQueryResponse> {
        let path = crate::paths::query(query_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a new query
    pub async fn create(&self, req: CreateQueryRequest) -> Result<GetQueryResponse> {
        req.validate()?;
        let path = crate::paths::QUERIES;
        let request = self.client.request(reqwest::Method::POST, path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Update an existing query
    pub async fn update(&self, query_id: u64, req: CreateQueryRequest) -> Result<GetQueryResponse> {
        req.validate()?;
        let path = crate::paths::query(query_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Delete a query by ID
    pub async fn delete(&self, query_id: u64) -> Result<DeleteQueryResponse> {
        let path = crate::paths::query_delete(query_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Run a live query by saved query ID
    pub async fn run_live(
        &self,
        query_id: u64,
        req: RunLiveQueryRequest,
    ) -> Result<LiveQueryResult> {
        let path = crate::paths::query_run(query_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Get a saved query report
    pub async fn campaign_status(&self, query_id: u64) -> Result<CampaignStatus> {
        let path = crate::paths::query_report(query_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Batch delete queries by IDs
    pub async fn batch_delete(&self, ids: Vec<u64>) -> Result<()> {
        if ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "query IDs must not be empty".into(),
            ));
        }
        let body = serde_json::json!({ "ids": ids });
        let request = self
            .client
            .request(reqwest::Method::POST, crate::paths::QUERIES_DELETE)?
            .json(&body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Get a saved query report
    pub async fn campaign_results(&self, query_id: u64) -> Result<CampaignResults> {
        let path = crate::paths::query_report(query_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list queries queries
pub struct ListQueriesBuilder<'a> {
    client: &'a FleetClient,
    query: ListQueriesQuery,
}

impl<'a> ListQueriesBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::query::QueryOrderKey) -> Self {
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

    /// Execute the list queries query
    pub async fn send(self) -> Result<ListQueriesResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::QUERIES, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_queries_query_builder() {
        use crate::models::common::OrderDirection;
        use crate::models::query::QueryOrderKey;

        let query = ListQueriesQuery::new()
            .page(2)
            .per_page(50)
            .order_key(QueryOrderKey::Name)
            .order_direction(OrderDirection::Desc)
            .team_id(123);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "2".to_string())));
        assert!(params.contains(&("per_page".to_string(), "50".to_string())));
        assert!(params.contains(&("order_key".to_string(), "name".to_string())));
        assert!(params.contains(&("order_direction".to_string(), "desc".to_string())));
        assert!(params.contains(&("team_id".to_string(), "123".to_string())));
    }

    #[test]
    fn test_create_query_request() {
        let req = CreateQueryRequest {
            name: "Test Query".to_string(),
            description: Some("A test query".to_string()),
            query: "SELECT * FROM processes".to_string(),
            team_id: Some(1),
            interval: Some(3600),
            platform: Some("darwin".to_string()),
            min_osquery_version: None,
            observer_can_run: Some(true),
            discard_data: Some(false),
        };

        assert_eq!(req.name, "Test Query");
        assert_eq!(req.interval, Some(3600));
    }
}
