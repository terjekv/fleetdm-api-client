use crate::error::Result;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Valid order keys for user queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum UserOrderKey {
    Name,
    Email,
    CreatedAt,
}

impl UserOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserOrderKey::Name => "name",
            UserOrderKey::Email => "email",
            UserOrderKey::CreatedAt => "created_at",
        }
    }
}

/// A user in FleetDM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,

    #[serde(default)]
    enabled: bool,

    force_password_reset: bool,
    gravatar_url: String,
    sso_enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    api_only: Option<bool>,

    global_role: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    fleets: Option<Vec<UserTeam>>,

    #[serde(default, rename = "teams", skip_serializing_if = "Option::is_none")]
    legacy_teams: Option<Vec<UserTeam>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mfa_enabled: Option<bool>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn force_password_reset(&self) -> bool {
        self.force_password_reset
    }
    pub fn gravatar_url(&self) -> &str {
        &self.gravatar_url
    }
    pub fn sso_enabled(&self) -> bool {
        self.sso_enabled
    }
    pub fn api_only(&self) -> Option<bool> {
        self.api_only
    }
    pub fn global_role(&self) -> Option<&str> {
        self.global_role.as_deref()
    }
    pub fn teams(&self) -> Option<&[UserTeam]> {
        self.fleets.as_deref().or(self.legacy_teams.as_deref())
    }
    pub fn fleets(&self) -> Option<&[UserTeam]> {
        self.fleets.as_deref().or(self.legacy_teams.as_deref())
    }
    pub fn settings(&self) -> Option<&serde_json::Value> {
        self.settings.as_ref()
    }
    pub fn mfa_enabled(&self) -> Option<bool> {
        self.mfa_enabled
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// User's role within a team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTeam {
    id: u64,
    name: String,
    role: String,
}

impl UserTeam {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn role(&self) -> &str {
        &self.role
    }
}

/// Request to create a new user
#[derive(Clone, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_only: Option<bool>,

    #[serde(rename = "fleets", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<UserTeamRole>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTeamRole {
    pub id: u64,
    pub role: String,
}

/// Request to update a user
#[derive(Debug, Clone, Serialize)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_role: Option<String>,

    #[serde(rename = "fleets", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<UserTeamRole>>,
}

impl CreateUserRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        validate_name_and_email(&self.name, &self.email)?;
        if self
            .password
            .as_ref()
            .is_some_and(|password| password.is_empty())
        {
            return Err(crate::FleetError::Validation(
                "password must not be empty when provided".into(),
            ));
        }
        validate_roles(self.global_role.as_deref(), self.teams.as_deref())
    }
}

impl UpdateUserRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.is_none()
            && self.email.is_none()
            && self.enabled.is_none()
            && self.global_role.is_none()
            && self.teams.is_none()
        {
            return Err(crate::FleetError::Validation(
                "at least one user field must be provided".into(),
            ));
        }
        if self
            .name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "user name must not be empty when provided".into(),
            ));
        }
        if let Some(email) = &self.email {
            validate_email(email)?;
        }
        validate_roles(self.global_role.as_deref(), self.teams.as_deref())
    }
}

fn validate_name_and_email(name: &str, email: &str) -> crate::Result<()> {
    if name.trim().is_empty() {
        return Err(crate::FleetError::Validation(
            "user name must not be empty".into(),
        ));
    }
    validate_email(email)
}

fn validate_email(email: &str) -> crate::Result<()> {
    let email = email.trim();
    if email.is_empty() || email.contains(char::is_whitespace) {
        return Err(crate::FleetError::Validation(
            "email must be a non-empty address without whitespace".into(),
        ));
    }
    let mut parts = email.split('@');
    if parts.next().is_none_or(str::is_empty)
        || parts.next().is_none_or(|domain| !domain.contains('.'))
        || parts.next().is_some()
    {
        return Err(crate::FleetError::Validation(
            "email must contain one '@' and a domain".into(),
        ));
    }
    Ok(())
}

fn validate_roles(global_role: Option<&str>, fleets: Option<&[UserTeamRole]>) -> crate::Result<()> {
    const ROLES: [&str; 5] = ["admin", "maintainer", "observer", "observer_plus", "gitops"];
    if global_role.is_some_and(|role| !ROLES.contains(&role))
        || fleets.is_some_and(|fleets| {
            fleets
                .iter()
                .any(|fleet| !ROLES.contains(&fleet.role.as_str()))
        })
    {
        return Err(crate::FleetError::Validation(format!(
            "role must be one of: {}",
            ROLES.join(", ")
        )));
    }
    Ok(())
}

/// Response for listing users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    users: Vec<User>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListUsersResponse {
    pub fn users(&self) -> &[User] {
        &self.users
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

/// Response for getting a single user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    user: User,
}

impl GetUserResponse {
    pub fn user(&self) -> &User {
        &self.user
    }
}

/// Response for deleting a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    #[serde(default)]
    message: Option<String>,
}

impl DeleteUserResponse {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

/// Query parameters for listing users
#[derive(Debug, Default, Clone)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<UserOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub query: Option<String>,
    pub team_id: Option<u64>,
    pub fleet_id: Option<u64>,
}

impl ListUsersQuery {
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

    pub fn order_key(mut self, key: UserOrderKey) -> Self {
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
        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
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
