use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ReportOrderKey {
    Name,
    CreatedAt,
    UpdatedAt,
}

impl ReportOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReportOrderKey::Name => "name",
            ReportOrderKey::CreatedAt => "created_at",
            ReportOrderKey::UpdatedAt => "updated_at",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    id: u64,
    name: String,
    #[serde(default)]
    query: String,
    #[serde(default)]
    description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fleet_id: Option<u64>,
    #[serde(default, rename = "team_id", skip_serializing_if = "Option::is_none")]
    legacy_team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<DateTime<Utc>>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

impl Report {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn fleet_id(&self) -> Option<u64> {
        self.fleet_id.or(self.legacy_team_id)
    }
    pub fn interval(&self) -> Option<u32> {
        self.interval
    }
    pub fn platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }
    pub fn extra(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.extra
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateReportRequest {
    pub name: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observer_can_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_include_any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_include_all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_osquery_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automations_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard_data: Option<bool>,
}

impl CreateReportRequest {
    pub fn new(name: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            query: query.into(),
            description: None,
            fleet_id: None,
            interval: None,
            platform: None,
            observer_can_run: None,
            labels_include_any: None,
            labels_include_all: None,
            min_osquery_version: None,
            automations_enabled: None,
            logging: None,
            discard_data: None,
        }
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if self.name.trim().is_empty() || self.query.trim().is_empty() {
            return Err(crate::error::FleetError::Validation(
                "report name and query must not be empty".into(),
            ));
        }
        if self.labels_include_any.is_some() && self.labels_include_all.is_some() {
            return Err(crate::error::FleetError::Validation(
                "labels_include_any and labels_include_all are mutually exclusive".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observer_can_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_include_any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_include_all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_osquery_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automations_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard_data: Option<bool>,
}

impl UpdateReportRequest {
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.name.is_none()
            && self.query.is_none()
            && self.description.is_none()
            && self.interval.is_none()
            && self.platform.is_none()
            && self.observer_can_run.is_none()
            && self.labels_include_any.is_none()
            && self.labels_include_all.is_none()
            && self.min_osquery_version.is_none()
            && self.automations_enabled.is_none()
            && self.logging.is_none()
            && self.discard_data.is_none()
        {
            return Err(crate::error::FleetError::Validation(
                "at least one report field must be provided".into(),
            ));
        }
        if self
            .name
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
            || self
                .query
                .as_ref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(crate::error::FleetError::Validation(
                "report name and query must not be empty when provided".into(),
            ));
        }
        if self.labels_include_any.is_some() && self.labels_include_all.is_some() {
            return Err(crate::error::FleetError::Validation(
                "labels_include_any and labels_include_all are mutually exclusive".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunLiveReportRequest {
    pub host_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveReportResult {
    query_id: u64,
    report_id: u64,
    targeted_host_count: u64,
    responded_host_count: u64,
    results: Vec<LiveReportHostResult>,
}

impl LiveReportResult {
    pub fn query_id(&self) -> u64 {
        self.query_id
    }
    pub fn report_id(&self) -> u64 {
        self.report_id
    }
    pub fn targeted_host_count(&self) -> u64 {
        self.targeted_host_count
    }
    pub fn responded_host_count(&self) -> u64 {
        self.responded_host_count
    }
    pub fn results(&self) -> &[LiveReportHostResult] {
        &self.results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveReportHostResult {
    host_id: u64,
    #[serde(default)]
    rows: Vec<serde_json::Value>,
    error: Option<String>,
}

impl LiveReportHostResult {
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
    pub fn rows(&self) -> &[serde_json::Value] {
        &self.rows
    }
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportsResponse {
    reports: Vec<Report>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListReportsResponse {
    pub fn reports(&self) -> &[Report] {
        &self.reports
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportResponse {
    report: Report,
}

impl GetReportResponse {
    pub fn report(&self) -> &Report {
        &self.report
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataResponse {
    #[serde(flatten)]
    body: serde_json::Map<String, serde_json::Value>,
}

impl ReportDataResponse {
    pub fn body(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.body
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteReportsResponse {
    deleted: u64,
}

impl DeleteReportsResponse {
    pub fn deleted(&self) -> u64 {
        self.deleted
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListReportsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<ReportOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub fleet_id: Option<u64>,
    pub query: Option<String>,
    pub platform: Option<String>,
    pub merge_inherited: Option<bool>,
    pub after: Option<String>,
}

impl ListReportsQuery {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }
    pub fn order_key(mut self, key: ReportOrderKey) -> Self {
        self.order_key = Some(key);
        self
    }
    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
        self
    }
    pub fn fleet_id(mut self, fleet_id: u64) -> Self {
        self.fleet_id = Some(fleet_id);
        self
    }
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
    pub fn merge_inherited(mut self, merge_inherited: bool) -> Self {
        self.merge_inherited = Some(merge_inherited);
        self
    }
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }
        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(order_key) = self.order_key {
            params.push(("order_key".to_string(), order_key.as_str().to_string()));
        }
        if let Some(order_direction) = self.order_direction {
            let direction = match order_direction {
                OrderDirection::Asc => "asc",
                OrderDirection::Desc => "desc",
            };
            params.push(("order_direction".to_string(), direction.to_string()));
        }
        if let Some(fleet_id) = self.fleet_id {
            params.push(("fleet_id".to_string(), fleet_id.to_string()));
        }
        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
        }
        if let Some(ref platform) = self.platform {
            params.push(("platform".to_string(), platform.clone()));
        }
        if let Some(merge_inherited) = self.merge_inherited {
            params.push(("merge_inherited".to_string(), merge_inherited.to_string()));
        }
        if let Some(ref after) = self.after {
            params.push(("after".to_string(), after.clone()));
        }
        params
    }

    pub(crate) fn validate(&self) -> crate::error::Result<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            self.after.is_some(),
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListHostReportsQuery {
    pub query: Option<String>,
    pub exclude_no_results: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<String>,
    pub order_direction: Option<OrderDirection>,
}

impl ListHostReportsQuery {
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref query) = self.query {
            params.push(("query".into(), query.clone()));
        }
        if let Some(exclude) = self.exclude_no_results {
            params.push(("exclude_no_results".into(), exclude.to_string()));
        }
        if let Some(page) = self.page {
            params.push(("page".into(), page.to_string()));
        }
        if let Some(per_page) = self.per_page {
            params.push(("per_page".into(), per_page.to_string()));
        }
        if let Some(ref order_key) = self.order_key {
            params.push(("order_key".into(), order_key.clone()));
        }
        if let Some(direction) = self.order_direction {
            params.push((
                "order_direction".into(),
                match direction {
                    OrderDirection::Asc => "asc",
                    OrderDirection::Desc => "desc",
                }
                .into(),
            ));
        }
        params
    }

    pub(crate) fn validate(&self) -> crate::error::Result<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            false,
        )
    }
}
