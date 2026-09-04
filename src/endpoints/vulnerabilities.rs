use crate::client::FleetClient;
use crate::error::{FleetError, Result};
use crate::models::vulnerability::*;
use crate::paths;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetVulnerabilityResponse {
    vulnerability: VulnerabilityDetails,
}

pub struct VulnerabilitiesEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> VulnerabilitiesEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// List vulnerabilities with optional filters
    pub fn list(&self) -> ListVulnerabilitiesBuilder<'_> {
        ListVulnerabilitiesBuilder {
            client: self.client,
            query: ListVulnerabilitiesQuery::new(),
        }
    }

    /// Get details for a specific vulnerability by CVE
    pub async fn get(
        &self,
        cve: &str,
        fleet_id: Option<u64>,
    ) -> Result<Option<VulnerabilityDetails>> {
        validate_cve(cve)?;
        let base = paths::vulnerability(cve);
        let params = fleet_id
            .map(|id| vec![("fleet_id".to_string(), id.to_string())])
            .unwrap_or_default();
        let path = crate::http::append_query_params(&base, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        if response.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        }
        let response: GetVulnerabilityResponse = crate::http::handle_response(response).await?;
        Ok(Some(response.vulnerability))
    }
}

fn validate_cve(cve: &str) -> Result<()> {
    let mut parts = cve.split('-');
    let prefix = parts.next();
    let year = parts.next();
    let number = parts.next();
    let valid = prefix.is_some_and(|prefix| prefix.eq_ignore_ascii_case("cve"))
        && year
            .is_some_and(|year| year.len() == 4 && year.bytes().all(|byte| byte.is_ascii_digit()))
        && number.is_some_and(|number| {
            number.len() >= 4 && number.bytes().all(|byte| byte.is_ascii_digit())
        })
        && parts.next().is_none();
    if !valid {
        return Err(FleetError::Validation(
            "CVE must use the CVE-YYYY-NNNN format".into(),
        ));
    }
    Ok(())
}

pub struct ListVulnerabilitiesBuilder<'a> {
    client: &'a FleetClient,
    query: ListVulnerabilitiesQuery,
}

impl<'a> ListVulnerabilitiesBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: VulnerabilityOrderKey) -> Self {
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

    pub fn exploit(mut self, exploit: bool) -> Self {
        self.query = self.query.exploit(exploit);
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

    pub async fn send(self) -> Result<ListVulnerabilitiesResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(paths::VULNERABILITIES, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
