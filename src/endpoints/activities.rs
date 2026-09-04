use crate::client::FleetClient;
use crate::error::Result;
use crate::models::activity::*;

/// Endpoint for activity-related operations
pub struct ActivitiesEndpoint {
    client: FleetClient,
}

impl ActivitiesEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List activities with optional filters
    pub fn list(&self) -> ListActivitiesBuilder<'_> {
        ListActivitiesBuilder {
            client: &self.client,
            query: ListActivitiesQuery::new(),
        }
    }
}

/// Builder for constructing list activities queries
pub struct ListActivitiesBuilder<'a> {
    client: &'a FleetClient,
    query: ListActivitiesQuery,
}

impl<'a> ListActivitiesBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::activity::ActivityOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }

    /// Execute the list activities query
    pub async fn send(self) -> Result<ListActivitiesResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::ACTIVITIES, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_activities_query_builder() {
        let query = ListActivitiesQuery::new().page(1).per_page(100);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "100".to_string())));
    }
}
