use crate::client::FleetClient;
use crate::error::Result;
use crate::http::handle_response_with_retry;
use crate::models::label::*;
use crate::retry::RetryPolicy;

/// Endpoint for label-related operations
pub struct LabelsEndpoint {
    client: FleetClient,
}

impl LabelsEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List labels with optional filters
    pub fn list(&self) -> ListLabelsBuilder<'_> {
        ListLabelsBuilder {
            client: &self.client,
            query: ListLabelsQuery::new(),
            retry_override: None,
        }
    }

    /// Get a specific label by ID
    pub async fn get(&self, label_id: u64) -> Result<GetLabelResponse> {
        let path = crate::paths::label(label_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a new label
    pub async fn create(&self, req: CreateLabelRequest) -> Result<GetLabelResponse> {
        req.validate()?;
        let path = crate::paths::LABELS;
        let request = self.client.request(reqwest::Method::POST, path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Update an existing label
    pub async fn update(&self, label_id: u64, req: UpdateLabelRequest) -> Result<GetLabelResponse> {
        req.validate()?;
        let path = crate::paths::label(label_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Delete a label by ID
    pub async fn delete(&self, label_id: u64) -> Result<DeleteLabelResponse> {
        let path = crate::paths::label_delete(label_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list labels queries
pub struct ListLabelsBuilder<'a> {
    client: &'a FleetClient,
    query: ListLabelsQuery,
    retry_override: Option<Option<RetryPolicy>>,
}

impl<'a> ListLabelsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::label::LabelOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }

    /// Override the retry policy for this specific request
    ///
    /// This allows you to customize retry behavior on a per-request basis,
    /// overriding the client's default retry policy.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::{FleetClient, RetryPolicy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// // Use aggressive retry for this specific request
    /// let labels = client.labels()
    ///     .list()
    ///     .with_retry(RetryPolicy::aggressive())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_retry(mut self, policy: RetryPolicy) -> Self {
        self.retry_override = Some(Some(policy));
        self
    }

    /// Disable retry for this specific request
    ///
    /// This will disable retries even if the client has a default retry policy configured.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// // Disable retry for this request
    /// let labels = client.labels()
    ///     .list()
    ///     .no_retry()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn no_retry(mut self) -> Self {
        self.retry_override = Some(None);
        self
    }

    /// Execute the list labels query
    pub async fn send(self) -> Result<ListLabelsResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::LABELS, &params);

        // Determine which retry policy to use
        let retry_policy = match self.retry_override {
            Some(Some(ref policy)) => Some(policy), // Explicit override
            Some(None) => None,                     // Explicitly disabled
            None => self.client.retry_policy(),     // Use client default
        };

        if let Some(policy) = retry_policy {
            // Use retry logic
            let client = self.client;
            let path_clone = path.clone();
            handle_response_with_retry(policy, || async {
                let req = client.request(reqwest::Method::GET, &path_clone)?;
                req.send().await.map_err(Into::into)
            })
            .await
        } else {
            // No retry logic
            let request = self.client.request(reqwest::Method::GET, &path)?;
            crate::http::send_request(request, self.client.retry_policy()).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_labels_query_builder() {
        let query = ListLabelsQuery::new().page(1).per_page(30);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "30".to_string())));
    }
}
