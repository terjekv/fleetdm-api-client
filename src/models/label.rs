use crate::error::Result;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum LabelOrderKey {
    Name,
    HostCount,
    CreatedAt,
}

impl LabelOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            LabelOrderKey::Name => "name",
            LabelOrderKey::HostCount => "host_count",
            LabelOrderKey::CreatedAt => "created_at",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum LabelType {
    Regular,
    Builtin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    id: u64,
    name: String,
    description: String,
    query: String,
    label_type: LabelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_membership_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_text: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Label {
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
    pub fn label_type(&self) -> LabelType {
        self.label_type
    }
    pub fn label_membership_type(&self) -> Option<&str> {
        self.label_membership_type.as_deref()
    }
    pub fn host_count(&self) -> Option<u32> {
        self.host_count
    }
    pub fn count(&self) -> Option<u32> {
        self.count
    }
    pub fn platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }
    pub fn author_id(&self) -> Option<u64> {
        self.author_id
    }
    pub fn display_text(&self) -> Option<&str> {
        self.display_text.as_deref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateLabelRequest {
    pub name: String,
    pub description: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateLabelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateLabelRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() || self.query.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "label name and query must not be empty".into(),
            ));
        }
        Ok(())
    }
}

impl UpdateLabelRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.is_none() && self.description.is_none() {
            return Err(crate::FleetError::Validation(
                "at least one label field must be provided".into(),
            ));
        }
        if self
            .name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "label name must not be empty when provided".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLabelsResponse {
    labels: Vec<Label>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListLabelsResponse {
    pub fn labels(&self) -> &[Label] {
        &self.labels
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLabelResponse {
    label: Label,
}

impl GetLabelResponse {
    pub fn label(&self) -> &Label {
        &self.label
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteLabelResponse {
    #[serde(default)]
    message: String,
}

impl DeleteLabelResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListLabelsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<LabelOrderKey>,
    pub order_direction: Option<OrderDirection>,
}

impl ListLabelsQuery {
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
    pub fn order_key(mut self, key: LabelOrderKey) -> Self {
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
