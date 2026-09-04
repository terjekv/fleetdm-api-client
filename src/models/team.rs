use crate::error::Result;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TeamOrderKey {
    Name,
    CreatedAt,
}

impl TeamOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            TeamOrderKey::Name => "name",
            TeamOrderKey::CreatedAt => "created_at",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    id: u64,
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_options: Option<serde_json::Value>,
    created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Vec<TeamSecret>>,
}

impl std::fmt::Debug for Team {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Team")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("description", &self.description)
            .field("created_at", &self.created_at)
            .field("user_count", &self.user_count)
            .field("host_count", &self.host_count)
            .field("secrets", &self.secrets)
            .finish_non_exhaustive()
    }
}

impl Team {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn agent_options(&self) -> Option<&serde_json::Value> {
        self.agent_options.as_ref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn user_count(&self) -> Option<u32> {
        self.user_count
    }
    pub fn host_count(&self) -> Option<u32> {
        self.host_count
    }
    pub fn secrets(&self) -> Option<&[TeamSecret]> {
        self.secrets.as_deref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamSecret {
    secret: String,
    created_at: DateTime<Utc>,
}

impl std::fmt::Debug for TeamSecret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TeamSecret")
            .field("secret", &"[REDACTED]")
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl TeamSecret {
    pub fn secret(&self) -> &str {
        &self.secret
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTeamRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct UpdateTeamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_options: Option<serde_json::Value>,
}

impl CreateTeamRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "team name must not be empty".into(),
            ));
        }
        Ok(())
    }
}

impl UpdateTeamRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.is_none() && self.description.is_none() && self.agent_options.is_none() {
            return Err(crate::FleetError::Validation(
                "at least one team field must be provided".into(),
            ));
        }
        if self
            .name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "team name must not be empty when provided".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTeamsResponse {
    teams: Vec<Team>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListTeamsResponse {
    pub fn teams(&self) -> &[Team] {
        &self.teams
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamResponse {
    team: Team,
}

impl GetTeamResponse {
    pub fn team(&self) -> &Team {
        &self.team
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTeamResponse {
    #[serde(default)]
    message: String,
}

impl DeleteTeamResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListTeamsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<TeamOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub query: Option<String>,
}

impl ListTeamsQuery {
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
    pub fn order_key(mut self, key: TeamOrderKey) -> Self {
        self.order_key = Some(key);
        self
    }
    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
        self
    }
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
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
        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
        }
        params
    }

    pub(crate) fn validate(&self) -> Result<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            false,
        )
    }
}
