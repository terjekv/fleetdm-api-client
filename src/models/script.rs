use crate::error::Result as FleetResult;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ScriptOrderKey {
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

impl ScriptOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScriptOrderKey::Id => "id",
            ScriptOrderKey::Name => "name",
            ScriptOrderKey::CreatedAt => "created_at",
            ScriptOrderKey::UpdatedAt => "updated_at",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    id: u64,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u64>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Script {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn team_id(&self) -> Option<u64> {
        self.team_id
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScriptExecutionResult {
    execution_id: String,
    script_id: u64,
    host_id: u64,
    execution_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<u32>,
    created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    executed_at: Option<DateTime<Utc>>,
}

impl ScriptExecutionResult {
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }
    pub fn script_id(&self) -> u64 {
        self.script_id
    }
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
    pub fn execution_status(&self) -> &str {
        &self.execution_status
    }
    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }
    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
    }
    pub fn runtime(&self) -> Option<u32> {
        self.runtime
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn executed_at(&self) -> Option<DateTime<Utc>> {
        self.executed_at
    }
}

#[derive(Clone, Serialize)]
pub struct CreateScriptRequest {
    pub name: String,
    pub script_contents: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<u64>,
}

impl CreateScriptRequest {
    pub fn new(name: impl Into<String>, script_contents: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            script_contents: script_contents.into(),
            team_id: None,
        }
    }

    pub fn fleet_id(mut self, fleet_id: u64) -> Self {
        self.team_id = Some(fleet_id);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunScriptRequest {
    pub host_id: u64,
    pub script_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScriptsResponse {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    scripts: Vec<Script>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListScriptsResponse {
    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum GetScriptResponse {
    Wrapped { script: Script },
    Inline(Script),
}

impl GetScriptResponse {
    pub fn script(&self) -> &Script {
        match self {
            GetScriptResponse::Wrapped { script } => script,
            GetScriptResponse::Inline(script) => script,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScriptResponse {
    script_id: u64,
}

impl CreateScriptResponse {
    pub fn script_id(&self) -> u64 {
        self.script_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScriptResponse {
    #[serde(default)]
    message: String,
}

impl DeleteScriptResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListScriptsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<ScriptOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub team_id: Option<u64>,
    pub fleet_id: Option<u64>,
    pub after: Option<String>,
}

impl ListScriptsQuery {
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
    pub fn order_key(mut self, key: ScriptOrderKey) -> Self {
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
        if let Some(team_id) = self.team_id {
            params.push(("team_id".to_string(), team_id.to_string()));
        }
        if let Some(fleet_id) = self.fleet_id {
            params.push(("fleet_id".to_string(), fleet_id.to_string()));
        }
        if let Some(ref after) = self.after {
            params.push(("after".to_string(), after.clone()));
        }
        params
    }

    pub(crate) fn validate(&self) -> FleetResult<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            self.after.is_some(),
        )
    }
}
