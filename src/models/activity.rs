use crate::error::Result;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Valid order keys for activity queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ActivityOrderKey {
    CreatedAt,
    Type,
}

impl ActivityOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityOrderKey::CreatedAt => "created_at",
            ActivityOrderKey::Type => "type",
        }
    }
}

/// An activity log entry in FleetDM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    id: u64,
    actor_id: Option<u64>,
    actor_full_name: Option<String>,
    actor_email: Option<String>,

    #[serde(alias = "activity_type")]
    r#type: String,

    details: serde_json::Value,
    created_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    actor_gravatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    actor_api_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_initiated: Option<bool>,
}

impl Activity {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn actor_id(&self) -> Option<u64> {
        self.actor_id
    }
    pub fn actor_full_name(&self) -> Option<&str> {
        self.actor_full_name.as_deref()
    }
    pub fn actor_email(&self) -> Option<&str> {
        self.actor_email.as_deref()
    }
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
    pub fn details(&self) -> &serde_json::Value {
        &self.details
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn actor_gravatar(&self) -> Option<&str> {
        self.actor_gravatar.as_deref()
    }
    pub fn actor_api_only(&self) -> Option<bool> {
        self.actor_api_only
    }
    pub fn fleet_initiated(&self) -> Option<bool> {
        self.fleet_initiated
    }
}

/// Response for listing activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivitiesResponse {
    activities: Vec<Activity>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListActivitiesResponse {
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

/// Query parameters for listing activities
#[derive(Debug, Default, Clone)]
pub struct ListActivitiesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<ActivityOrderKey>,
    pub order_direction: Option<OrderDirection>,
}

impl ListActivitiesQuery {
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

    pub fn order_key(mut self, key: ActivityOrderKey) -> Self {
        self.order_key = Some(key);
        self
    }

    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
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
