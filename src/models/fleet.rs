use crate::error::Result;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FleetOrderKey {
    Name,
    CreatedAt,
}

impl FleetOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            FleetOrderKey::Name => "name",
            FleetOrderKey::CreatedAt => "created_at",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Fleet {
    id: u64,
    name: String,
    #[serde(default)]
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Vec<FleetSecret>>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

impl std::fmt::Debug for Fleet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Fleet")
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

impl Fleet {
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
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }
    pub fn user_count(&self) -> Option<u32> {
        self.user_count
    }
    pub fn host_count(&self) -> Option<u32> {
        self.host_count
    }
    pub fn secrets(&self) -> Option<&[FleetSecret]> {
        self.secrets.as_deref()
    }
    pub fn extra(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.extra
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FleetSecret {
    secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
}

impl std::fmt::Debug for FleetSecret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FleetSecret")
            .field("secret", &"[REDACTED]")
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl FleetSecret {
    pub fn secret(&self) -> &str {
        &self.secret
    }
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }
}

#[derive(Clone, Serialize)]
pub struct FleetSecretInput {
    pub secret: String,
}

#[derive(Clone, Serialize)]
pub struct UpdateFleetSecretsRequest {
    pub secrets: Vec<FleetSecretInput>,
}

impl UpdateFleetSecretsRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self
            .secrets
            .iter()
            .any(|secret| secret.secret.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "enroll secrets must not be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FleetSecretsResponse {
    #[serde(default)]
    secrets: Vec<FleetSecret>,
}

impl FleetSecretsResponse {
    pub fn secrets(&self) -> &[FleetSecret] {
        &self.secrets
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FleetUserRole {
    pub id: u64,
    pub role: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateFleetUsersRequest {
    pub users: Vec<FleetUserRole>,
}

impl UpdateFleetUsersRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        const ROLES: [&str; 5] = ["admin", "maintainer", "observer", "observer_plus", "gitops"];
        if self.users.is_empty() {
            return Err(crate::FleetError::Validation(
                "users must not be empty".into(),
            ));
        }
        if self
            .users
            .iter()
            .any(|user| !ROLES.contains(&user.role.as_str()))
        {
            return Err(crate::FleetError::Validation(format!(
                "fleet role must be one of: {}",
                ROLES.join(", ")
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeletedFleetPoliciesResponse {
    deleted: u64,
}

impl DeletedFleetPoliciesResponse {
    pub fn deleted(&self) -> u64 {
        self.deleted
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateFleetRequest {
    pub name: String,
}

#[derive(Clone, Serialize, Default)]
pub struct UpdateFleetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdm: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_expiry_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<serde_json::Value>,
}

impl CreateFleetRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "fleet name must not be empty".into(),
            ));
        }
        Ok(())
    }
}

impl UpdateFleetRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.is_none()
            && self.host_ids.is_none()
            && self.user_ids.is_none()
            && self.webhook_settings.is_none()
            && self.integrations.is_none()
            && self.mdm.is_none()
            && self.host_expiry_settings.is_none()
            && self.features.is_none()
        {
            return Err(crate::FleetError::Validation(
                "at least one fleet field must be provided".into(),
            ));
        }
        if self
            .name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "fleet name must not be empty when provided".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFleetsResponse {
    fleets: Vec<Fleet>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListFleetsResponse {
    pub fn fleets(&self) -> &[Fleet] {
        &self.fleets
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFleetResponse {
    #[serde(alias = "team")]
    fleet: Fleet,
}

impl GetFleetResponse {
    pub fn fleet(&self) -> &Fleet {
        &self.fleet
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListFleetsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<FleetOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub query: Option<String>,
    pub after: Option<String>,
}

impl ListFleetsQuery {
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
    pub fn order_key(mut self, key: FleetOrderKey) -> Self {
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
        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
        }
        if let Some(ref after) = self.after {
            params.push(("after".to_string(), after.clone()));
        }
        params
    }

    pub(crate) fn validate(&self) -> Result<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            self.after.is_some(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fleet_debug_redacts_structured_and_open_ended_secrets() {
        let fleet: Fleet = serde_json::from_value(serde_json::json!({
            "id": 1,
            "name": "Production",
            "description": "",
            "agent_options": {"enroll_secret": "agent-secret"},
            "secrets": [{"secret": "fleet-secret"}],
            "vendor_api_key": "vendor-secret"
        }))
        .unwrap();
        let debug = format!("{fleet:?}");
        assert!(!debug.contains("agent-secret"));
        assert!(!debug.contains("fleet-secret"));
        assert!(!debug.contains("vendor-secret"));
        assert!(debug.contains("[REDACTED]"));
    }
}
