use crate::client::FleetClient;
use crate::error::Result;
use crate::models::host::{
    DeleteHostResponse, GetHostResponse, HostCountResponse, HostSummaryResponse, ListHostsQuery,
    ListHostsResponse,
};

/// Endpoint for host-related operations
pub struct HostsEndpoint {
    client: FleetClient,
}

impl HostsEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List hosts with optional filters
    ///
    /// # Example
    /// ```no_run
    /// use fleetdm_api_client::{FleetClient, models::HostStatus};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_token("your-token")?
    ///     .build();
    ///     
    /// let hosts = client.hosts()
    ///     .list()
    ///     .status(HostStatus::Online)
    ///     .per_page(50)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListHostsBuilder<'_> {
        ListHostsBuilder {
            client: &self.client,
            query: ListHostsQuery::new(),
        }
    }

    /// Get a specific host by ID
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// let host = client.hosts().get(123).await?;
    /// println!("Host: {}", host.host().hostname());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, host_id: u64) -> Result<GetHostResponse> {
        let path = crate::paths::host(host_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;

        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Delete a host by ID
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// let result = client.hosts().delete(123).await?;
    /// println!("Deleted: {}", result.message());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, host_id: u64) -> Result<DeleteHostResponse> {
        let path = crate::paths::host(host_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;

        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Request a refetch of host data
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// client.hosts().refetch(123).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refetch(&self, host_id: u64) -> Result<()> {
        let path = crate::paths::host_refetch(host_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Lock a device (MDM command)
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// let response = client.hosts().lock(123).await?;
    /// println!("Device locked: {}", response.host_id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn lock(&self, host_id: u64) -> Result<crate::models::host::LockHostResponse> {
        let path = crate::paths::host_lock(host_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Unlock a device (MDM command)
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// use fleetdm_api_client::models::host::UnlockHostRequest;
    /// let response = client.hosts().unlock(123, UnlockHostRequest { pin: "123456".to_string() }).await?;
    /// println!("Device unlocked: {}", response.host_id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unlock(
        &self,
        host_id: u64,
        request: crate::models::host::UnlockHostRequest,
    ) -> Result<crate::models::host::UnlockHostResponse> {
        let path = crate::paths::host_unlock(host_id);
        let req = self
            .client
            .request(reqwest::Method::POST, &path)?
            .json(&request);
        crate::http::send_request(req, self.client.retry_policy()).await
    }

    /// Wipe a device (MDM command)
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FleetClient::builder("https://fleet.example.com")?.with_token("token")?.build();
    /// let response = client.hosts().wipe(123).await?;
    /// println!("Device wipe initiated: {}", response.host_id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wipe(&self, host_id: u64) -> Result<crate::models::host::WipeHostResponse> {
        let path = crate::paths::host_wipe(host_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get a host by identifier (hostname, UUID, or hardware serial)
    pub async fn get_by_identifier(&self, identifier: &str) -> Result<GetHostResponse> {
        let path = crate::paths::host_identifier(identifier);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get host count, optionally filtered with the same parameters as list
    pub fn count(&self) -> HostCountBuilder<'_> {
        HostCountBuilder {
            client: &self.client,
            query: ListHostsQuery::new(),
        }
    }

    /// Get host summary (totals by status and platform)
    pub async fn summary(
        &self,
        fleet_id: Option<u64>,
        platform: Option<&str>,
    ) -> Result<HostSummaryResponse> {
        let mut params = vec![];
        if let Some(id) = fleet_id {
            params.push(("fleet_id".to_string(), id.to_string()));
        }
        if let Some(p) = platform {
            params.push(("platform".to_string(), p.to_string()));
        }
        let path = crate::http::append_query_params(crate::paths::HOST_SUMMARY, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Batch delete hosts by IDs
    pub async fn batch_delete(&self, ids: Vec<u64>) -> Result<()> {
        if ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "host IDs must not be empty".into(),
            ));
        }
        let body = serde_json::json!({ "ids": ids });
        let request = self
            .client
            .request(reqwest::Method::POST, crate::paths::HOSTS_DELETE)?
            .json(&body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Transfer hosts to a team by host IDs
    pub async fn transfer(&self, fleet_id: u64, host_ids: Vec<u64>) -> Result<()> {
        if host_ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "host IDs must not be empty".into(),
            ));
        }
        let body = serde_json::json!({ "fleet_id": fleet_id, "hosts": host_ids });
        let request = self
            .client
            .request(reqwest::Method::POST, crate::paths::HOSTS_TRANSFER)?
            .json(&body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list hosts queries
pub struct ListHostsBuilder<'a> {
    client: &'a FleetClient,
    query: ListHostsQuery,
}

impl<'a> ListHostsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::host::HostOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }

    pub fn status(mut self, status: crate::models::host::HostStatus) -> Self {
        self.query = self.query.status(status);
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

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = self.query.query(query);
        self
    }

    pub fn mdm_id(mut self, mdm_id: u64) -> Self {
        self.query = self.query.mdm_id(mdm_id);
        self
    }

    pub fn mdm_name(mut self, name: impl Into<String>) -> Self {
        self.query = self.query.mdm_name(name);
        self
    }

    pub fn mdm_enrollment_status(mut self, status: impl Into<String>) -> Self {
        self.query = self.query.mdm_enrollment_status(status);
        self
    }

    /// Execute the list hosts query
    pub async fn send(self) -> Result<ListHostsResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::HOSTS, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing host count queries
pub struct HostCountBuilder<'a> {
    client: &'a FleetClient,
    query: ListHostsQuery,
}

impl<'a> HostCountBuilder<'a> {
    pub fn status(mut self, status: crate::models::host::HostStatus) -> Self {
        self.query = self.query.status(status);
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

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = self.query.query(query);
        self
    }

    /// Execute the host count query
    pub async fn send(self) -> Result<HostCountResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::HOSTS_COUNT, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_hosts_query_builder() {
        use crate::models::common::OrderDirection;
        use crate::models::host::{HostOrderKey, HostStatus};

        let query = ListHostsQuery::new()
            .page(2)
            .per_page(50)
            .status(HostStatus::Online)
            .order_key(HostOrderKey::Hostname)
            .order_direction(OrderDirection::Asc)
            .team_id(123);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "2".to_string())));
        assert!(params.contains(&("per_page".to_string(), "50".to_string())));
        assert!(params.contains(&("status".to_string(), "online".to_string())));
        assert!(params.contains(&("order_key".to_string(), "hostname".to_string())));
        assert!(params.contains(&("order_direction".to_string(), "asc".to_string())));
        assert!(params.contains(&("team_id".to_string(), "123".to_string())));
    }
}
