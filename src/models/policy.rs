use crate::error::{FleetError, Result};
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PolicyOrderKey {
    Name,
    Critical,
    PassingHostCount,
    FailingHostCount,
    CreatedAt,
}

impl PolicyOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            PolicyOrderKey::Name => "name",
            PolicyOrderKey::Critical => "critical",
            PolicyOrderKey::PassingHostCount => "passing_host_count",
            PolicyOrderKey::FailingHostCount => "failing_host_count",
            PolicyOrderKey::CreatedAt => "created_at",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    id: u64,
    name: String,
    query: String,
    description: String,
    resolution: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
    critical: bool,
    #[serde(default)]
    conditional_access_enabled: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passing_host_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failing_host_count: Option<u32>,
}

impl Policy {
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
    pub fn resolution(&self) -> &str {
        &self.resolution
    }
    pub fn team_id(&self) -> Option<u64> {
        self.team_id
    }
    pub fn platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }
    pub fn critical(&self) -> bool {
        self.critical
    }
    pub fn conditional_access_enabled(&self) -> bool {
        self.conditional_access_enabled
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
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
    pub fn passing_host_count(&self) -> Option<u32> {
        self.passing_host_count
    }
    pub fn failing_host_count(&self) -> Option<u32> {
        self.failing_host_count
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub query: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_access_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_access_enabled: Option<bool>,
}

impl CreatePolicyRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.query.trim().is_empty() {
            return Err(FleetError::Validation(
                "policy name and query must not be empty".into(),
            ));
        }
        Ok(())
    }
}

impl UpdatePolicyRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.name.is_none()
            && self.query.is_none()
            && self.description.is_none()
            && self.resolution.is_none()
            && self.team_id.is_none()
            && self.platform.is_none()
            && self.critical.is_none()
            && self.conditional_access_enabled.is_none()
        {
            return Err(FleetError::Validation(
                "at least one policy field must be provided".into(),
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
            return Err(FleetError::Validation(
                "policy name and query must not be empty when provided".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conditional_access_is_preserved_in_policy_wire_types() {
        let policy: Policy = serde_json::from_value(serde_json::json!({
            "id": 1,
            "name": "Encrypted",
            "query": "SELECT 1",
            "description": "",
            "resolution": "",
            "critical": true,
            "conditional_access_enabled": true,
            "created_at": "2025-01-01T00:00:00Z",
            "updated_at": "2025-01-01T00:00:00Z"
        }))
        .unwrap();
        assert!(policy.conditional_access_enabled());

        let request = CreatePolicyRequest {
            name: "Encrypted".into(),
            query: "SELECT 1".into(),
            description: String::new(),
            resolution: None,
            team_id: None,
            platform: None,
            critical: None,
            conditional_access_enabled: Some(true),
        };
        assert_eq!(
            serde_json::to_value(request).unwrap()["conditional_access_enabled"],
            true
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPoliciesResponse {
    #[serde(default)]
    policies: Vec<Policy>,
    #[serde(default)]
    inherited_policies: Vec<Policy>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListPoliciesResponse {
    pub fn policies(&self) -> &[Policy] {
        &self.policies
    }
    pub fn inherited_policies(&self) -> &[Policy] {
        &self.inherited_policies
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPolicyResponse {
    policy: Policy,
}

impl GetPolicyResponse {
    pub fn policy(&self) -> &Policy {
        &self.policy
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyResponse {
    #[serde(default)]
    message: String,
}

impl DeletePolicyResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<PolicyOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub team_id: Option<u64>,
    pub fleet_id: Option<u64>,
}

impl ListPoliciesQuery {
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
    pub fn order_key(mut self, key: PolicyOrderKey) -> Self {
        self.order_key = Some(key);
        self
    }
    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
        self
    }
    pub fn team_id(mut self, team_id: u64) -> Self {
        self.team_id = Some(team_id);
        self.fleet_id = None;
        self
    }

    pub fn fleet_id(mut self, fleet_id: u64) -> Self {
        self.fleet_id = Some(fleet_id);
        self.team_id = None;
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
        if let Some(fleet_id) = self.fleet_id {
            params.push(("fleet_id".to_string(), fleet_id.to_string()));
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
