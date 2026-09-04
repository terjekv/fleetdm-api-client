use crate::client::FleetClient;
use crate::error::Result;
use crate::models::report::*;

/// Endpoint for report-related operations.
pub struct ReportsEndpoint {
    client: FleetClient,
}

impl ReportsEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// `GET /api/v1/fleet/reports`
    pub fn list(&self) -> ListReportsBuilder<'_> {
        ListReportsBuilder {
            client: &self.client,
            query: ListReportsQuery::new(),
        }
    }

    /// `GET /api/v1/fleet/reports/:id`
    pub async fn get(&self, report_id: u64) -> Result<GetReportResponse> {
        let path = crate::paths::report(report_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/reports`
    pub async fn create(&self, req: CreateReportRequest) -> Result<GetReportResponse> {
        req.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, crate::paths::REPORTS)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// `PATCH /api/v1/fleet/reports/:id`
    pub async fn update(
        &self,
        report_id: u64,
        req: UpdateReportRequest,
    ) -> Result<GetReportResponse> {
        req.validate()?;
        let path = crate::paths::report(report_id);
        let request = self.client.request(reqwest::Method::PATCH, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// `DELETE /api/v1/fleet/reports/id/:id`
    pub async fn delete(&self, report_id: u64) -> Result<()> {
        let path = crate::paths::report_delete(report_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// `DELETE /api/v1/fleet/reports/:name`, optionally scoped to a fleet.
    pub async fn delete_by_name(&self, name: &str, fleet_id: Option<u64>) -> Result<()> {
        if name.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "report name must not be empty".into(),
            ));
        }
        let path = crate::paths::report_name(name);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        let request = match fleet_id {
            Some(fleet_id) => request.json(&serde_json::json!({ "fleet_id": fleet_id })),
            None => request,
        };
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/reports/delete`
    pub async fn batch_delete(&self, ids: Vec<u64>) -> Result<DeleteReportsResponse> {
        if ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "report IDs must not be empty".into(),
            ));
        }
        let body = serde_json::json!({ "ids": ids });
        let request = self
            .client
            .request(reqwest::Method::POST, crate::paths::REPORTS_DELETE)?
            .json(&body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `POST /api/v1/fleet/reports/:id/run`
    pub async fn run_live(
        &self,
        report_id: u64,
        req: RunLiveReportRequest,
    ) -> Result<LiveReportResult> {
        if req.host_ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "host IDs must not be empty".into(),
            ));
        }
        let path = crate::paths::report_run(report_id);
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// `GET /api/v1/fleet/reports/:id/report`
    pub async fn report_data(&self, report_id: u64) -> Result<ReportDataResponse> {
        self.report_data_for_fleet(report_id, None).await
    }

    /// `GET /api/v1/fleet/reports/:id/report?fleet_id=...`
    pub async fn report_data_for_fleet(
        &self,
        report_id: u64,
        fleet_id: Option<u64>,
    ) -> Result<ReportDataResponse> {
        let path = crate::paths::report_data(report_id);
        let params = fleet_id
            .map(|id| vec![("fleet_id".to_string(), id.to_string())])
            .unwrap_or_default();
        let path = crate::http::append_query_params(&path, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `GET /api/v1/fleet/hosts/:id/reports`
    pub async fn list_host_reports(&self, host_id: u64) -> Result<serde_json::Value> {
        self.host_reports(host_id).send().await
    }

    /// Build a filtered `GET /api/v1/fleet/hosts/:id/reports` request.
    pub fn host_reports(&self, host_id: u64) -> ListHostReportsBuilder<'_> {
        ListHostReportsBuilder {
            client: &self.client,
            host_id,
            query: ListHostReportsQuery::default(),
        }
    }

    async fn list_host_reports_with_query(
        &self,
        host_id: u64,
        query: ListHostReportsQuery,
    ) -> Result<serde_json::Value> {
        let path = crate::paths::host_reports(host_id);
        let path = crate::http::append_query_params(&path, &query.to_query_params());
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// `GET /api/v1/fleet/hosts/:id/reports/:report_id`
    pub async fn host_report(&self, host_id: u64, report_id: u64) -> Result<serde_json::Value> {
        let path = crate::paths::host_report(host_id, report_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list reports queries.
pub struct ListReportsBuilder<'a> {
    client: &'a FleetClient,
    query: ListReportsQuery,
}

impl<'a> ListReportsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }
    pub fn order_key(mut self, key: crate::models::report::ReportOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }
    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
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
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.query = self.query.platform(platform);
        self
    }
    pub fn merge_inherited(mut self, merge_inherited: bool) -> Self {
        self.query = self.query.merge_inherited(merge_inherited);
        self
    }
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.query = self.query.after(after);
        self
    }

    pub async fn send(self) -> Result<ListReportsResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::REPORTS, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

pub struct ListHostReportsBuilder<'a> {
    client: &'a FleetClient,
    host_id: u64,
    query: ListHostReportsQuery,
}

impl<'a> ListHostReportsBuilder<'a> {
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query.query = Some(query.into());
        self
    }
    pub fn exclude_no_results(mut self, exclude: bool) -> Self {
        self.query.exclude_no_results = Some(exclude);
        self
    }
    pub fn page(mut self, page: u32) -> Self {
        self.query.page = Some(page);
        self
    }
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query.per_page = Some(per_page);
        self
    }
    pub fn order_key(mut self, order_key: impl Into<String>) -> Self {
        self.query.order_key = Some(order_key.into());
        self
    }
    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query.order_direction = Some(direction);
        self
    }
    pub async fn send(self) -> Result<serde_json::Value> {
        self.query.validate()?;
        let endpoint = ReportsEndpoint::new(self.client.clone());
        endpoint
            .list_host_reports_with_query(self.host_id, self.query)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_reports_query_builder() {
        let query = ListReportsQuery::new()
            .page(2)
            .per_page(50)
            .fleet_id(123)
            .platform("darwin");
        let params = query.to_query_params();
        assert!(params.contains(&("page".to_string(), "2".to_string())));
        assert!(params.contains(&("per_page".to_string(), "50".to_string())));
        assert!(params.contains(&("fleet_id".to_string(), "123".to_string())));
        assert!(params.contains(&("platform".to_string(), "darwin".to_string())));
    }
}
