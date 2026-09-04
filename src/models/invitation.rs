use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::common::PaginationMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRole {
    id: u64,
    role: String,
}

impl TeamRole {
    pub fn new(id: u64, role: impl Into<String>) -> Self {
        Self {
            id,
            role: role.into(),
        }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn role(&self) -> &str {
        &self.role
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    id: u64,
    email: String,
    name: String,
    #[serde(default)]
    sso_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_role: Option<String>,
    #[serde(default)]
    teams: Vec<TeamRole>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Invitation {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn sso_enabled(&self) -> bool {
        self.sso_enabled
    }
    pub fn global_role(&self) -> Option<&str> {
        self.global_role.as_deref()
    }
    pub fn teams(&self) -> &[TeamRole] {
        &self.teams
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateInvitationRequest {
    pub email: String,
    pub name: String,
    pub sso_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_role: Option<String>,
    #[serde(rename = "fleets", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamRole>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateInvitationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_role: Option<String>,
    #[serde(rename = "fleets", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<TeamRole>>,
}

impl CreateInvitationRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        validate_identity(&self.name, &self.email)?;
        validate_roles(self.global_role.as_deref(), self.teams.as_deref())
    }
}

impl UpdateInvitationRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.email.is_none()
            && self.name.is_none()
            && self.sso_enabled.is_none()
            && self.global_role.is_none()
            && self.teams.is_none()
        {
            return Err(crate::FleetError::Validation(
                "at least one invitation field must be provided".into(),
            ));
        }
        if self
            .name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "invitation name must not be empty".into(),
            ));
        }
        if let Some(email) = &self.email {
            validate_email(email)?;
        }
        validate_roles(self.global_role.as_deref(), self.teams.as_deref())
    }
}

fn validate_identity(name: &str, email: &str) -> crate::Result<()> {
    if name.trim().is_empty() {
        return Err(crate::FleetError::Validation(
            "invitation name must not be empty".into(),
        ));
    }
    validate_email(email)
}

fn validate_email(email: &str) -> crate::Result<()> {
    let email = email.trim();
    let mut parts = email.split('@');
    if email.contains(char::is_whitespace)
        || parts.next().is_none_or(str::is_empty)
        || parts.next().is_none_or(|domain| !domain.contains('.'))
        || parts.next().is_some()
    {
        return Err(crate::FleetError::Validation(
            "invitation email must contain one '@', a domain, and no whitespace".into(),
        ));
    }
    Ok(())
}

fn validate_roles(global_role: Option<&str>, teams: Option<&[TeamRole]>) -> crate::Result<()> {
    const ROLES: [&str; 5] = ["admin", "maintainer", "observer", "observer_plus", "gitops"];
    if global_role.is_some_and(|role| !ROLES.contains(&role))
        || teams.is_some_and(|teams| {
            teams
                .iter()
                .any(|team| !ROLES.contains(&team.role.as_str()))
        })
    {
        return Err(crate::FleetError::Validation(format!(
            "invitation role must be one of: {}",
            ROLES.join(", ")
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvitationResponse {
    invite: Invitation,
}

impl CreateInvitationResponse {
    pub fn invite(&self) -> &Invitation {
        &self.invite
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInvitationResponse {
    invite: Invitation,
}

impl GetInvitationResponse {
    pub fn invite(&self) -> &Invitation {
        &self.invite
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListInvitationsResponse {
    #[serde(default)]
    invites: Vec<Invitation>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListInvitationsResponse {
    pub fn invites(&self) -> &[Invitation] {
        &self.invites
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateInvitationResponse {
    invite: Invitation,
}

impl UpdateInvitationResponse {
    pub fn invite(&self) -> &Invitation {
        &self.invite
    }
}
