use crate::error::{FleetError, Result as FleetResult};
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum QueryOrderKey {
    Name,
    CreatedAt,
    UpdatedAt,
}

impl QueryOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryOrderKey::Name => "name",
            QueryOrderKey::CreatedAt => "created_at",
            QueryOrderKey::UpdatedAt => "updated_at",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    id: u64,
    name: String,
    description: String,
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_osquery_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automations_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observer_can_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discard_data: Option<bool>,
}

impl Query {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn team_id(&self) -> Option<u64> {
        self.team_id
    }
    pub fn interval(&self) -> Option<u32> {
        self.interval
    }
    pub fn platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }
    pub fn min_osquery_version(&self) -> Option<&str> {
        self.min_osquery_version.as_deref()
    }
    pub fn automations_enabled(&self) -> Option<bool> {
        self.automations_enabled
    }
    pub fn logging(&self) -> Option<&str> {
        self.logging.as_deref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn saved(&self) -> Option<bool> {
        self.saved
    }
    pub fn author_id(&self) -> Option<u64> {
        self.author_id
    }
    pub fn author_name(&self) -> Option<&str> {
        self.author_name.as_deref()
    }
    pub fn author_email(&self) -> Option<&str> {
        self.author_email.as_deref()
    }
    pub fn observer_can_run(&self) -> Option<bool> {
        self.observer_can_run
    }
    pub fn discard_data(&self) -> Option<bool> {
        self.discard_data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignStatus {
    campaign_id: u64,
    status: String,
    totals: CampaignTotals,
}

impl CampaignStatus {
    pub fn campaign_id(&self) -> u64 {
        self.campaign_id
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn totals(&self) -> &CampaignTotals {
        &self.totals
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignTotals {
    count: u32,
    online: u32,
    offline: u32,
    missing_in_action: u32,
}

impl CampaignTotals {
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn online(&self) -> u32 {
        self.online
    }
    pub fn offline(&self) -> u32 {
        self.offline
    }
    pub fn missing_in_action(&self) -> u32 {
        self.missing_in_action
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResults {
    campaign_id: u64,
    status: String,
    totals: CampaignTotals,
    results: Vec<CampaignResultRow>,
}

impl CampaignResults {
    pub fn campaign_id(&self) -> u64 {
        self.campaign_id
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn totals(&self) -> &CampaignTotals {
        &self.totals
    }
    pub fn results(&self) -> &[CampaignResultRow] {
        &self.results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResultRow {
    host_id: u64,
    hostname: String,
    rows: Vec<serde_json::Value>,
    error: Option<String>,
}

impl CampaignResultRow {
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    pub fn rows(&self) -> &[serde_json::Value] {
        &self.rows
    }
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveQueryResult {
    campaign_id: u64,
}

impl LiveQueryResult {
    pub fn campaign_id(&self) -> u64 {
        self.campaign_id
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateQueryRequest {
    pub name: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_osquery_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observer_can_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard_data: Option<bool>,
}

impl CreateQueryRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        if self.name.trim().is_empty() || self.query.trim().is_empty() {
            return Err(FleetError::Validation(
                "query name and SQL must not be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunLiveQueryRequest {
    pub query: String,
    pub selected: SelectedTargets,
}

#[derive(Debug, Clone, Serialize)]
pub struct SelectedTargets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueriesResponse {
    queries: Vec<Query>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListQueriesResponse {
    pub fn queries(&self) -> &[Query] {
        &self.queries
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueryResponse {
    query: Query,
}

impl GetQueryResponse {
    pub fn query(&self) -> &Query {
        &self.query
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueryResponse {
    #[serde(default)]
    message: String,
}

impl DeleteQueryResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListQueriesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<QueryOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub team_id: Option<u64>,
}

impl ListQueriesQuery {
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
    pub fn order_key(mut self, key: QueryOrderKey) -> Self {
        self.order_key = Some(key);
        self
    }
    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
        self
    }
    pub fn team_id(mut self, team_id: u64) -> Self {
        self.team_id = Some(team_id);
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
        if let Some(team_id) = self.team_id {
            params.push(("team_id".to_string(), team_id.to_string()));
        }
        params
    }

    pub(crate) fn validate(&self) -> FleetResult<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            false,
        )
    }
}
