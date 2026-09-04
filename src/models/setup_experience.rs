//! Setup Experience (MDM Setup Assistant) models for device enrollment

use serde::{Deserialize, Serialize};

use crate::error::{FleetError, Result};
use crate::models::common::{FileUpload, OrderDirection, PaginationMeta, validate_list_options};

/// Metadata for the EULA currently stored by Fleet.
#[derive(Clone, Deserialize, Serialize)]
pub struct EulaMetadata {
    name: String,
    token: String,
    created_at: String,
}

impl EulaMetadata {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

/// Multipart request for a script that runs during macOS setup.
#[derive(Clone, Debug)]
pub struct SetupExperienceScriptRequest {
    pub script: FileUpload,
    pub fleet_id: Option<u64>,
}

/// Metadata for the configured setup-experience script.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupExperienceScript {
    id: u64,
    #[serde(default)]
    team_id: Option<u64>,
    #[serde(default)]
    fleet_id: Option<u64>,
    name: String,
    created_at: String,
    updated_at: String,
}

impl SetupExperienceScript {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn fleet_id(&self) -> Option<u64> {
        self.fleet_id.or(self.team_id)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

/// Software selected for automatic installation during setup.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateSetupExperienceSoftwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<u64>,
    pub software_title_ids: Vec<u64>,
}

/// Fields accepted by Fleet's setup-experience settings update.
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateSetupExperienceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_end_user_authentication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_end_user_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_all_software_macos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_all_software_windows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_release_device_manually: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_agent_install: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_managed_local_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_local_account_type: Option<String>,
}

impl UpdateSetupExperienceRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        if self
            .end_user_local_account_type
            .as_deref()
            .is_some_and(|kind| !matches!(kind, "admin" | "standard"))
        {
            return Err(FleetError::Validation(
                "end_user_local_account_type must be 'admin' or 'standard'".into(),
            ));
        }
        let has_update = self.enable_end_user_authentication.is_some()
            || self.lock_end_user_info.is_some()
            || self.require_all_software_macos.is_some()
            || self.require_all_software_windows.is_some()
            || self.enable_release_device_manually.is_some()
            || self.manual_agent_install.is_some()
            || self.enable_managed_local_account.is_some()
            || self.end_user_local_account_type.is_some();
        if !has_update {
            return Err(FleetError::Validation(
                "at least one setup-experience setting must be provided".into(),
            ));
        }
        Ok(())
    }
}

impl UpdateSetupExperienceSoftwareRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_platforms(self.platform.as_deref())?;
        if self.software_title_ids.is_empty() {
            return Err(FleetError::Validation(
                "software_title_ids must not be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListSetupExperienceSoftwareQuery {
    pub platforms: Option<String>,
    pub fleet_id: Option<u64>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<SetupExperienceSoftwareOrderKey>,
    pub order_direction: Option<OrderDirection>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetupExperienceSoftwareOrderKey {
    Name,
    HostsCount,
}

impl SetupExperienceSoftwareOrderKey {
    fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::HostsCount => "hosts_count",
        }
    }
}

impl ListSetupExperienceSoftwareQuery {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_platforms(self.platforms.as_deref())?;
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            false,
        )
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(platforms) = &self.platforms {
            params.push(("platform".into(), platforms.clone()));
        }
        if let Some(fleet_id) = self.fleet_id {
            params.push(("fleet_id".into(), fleet_id.to_string()));
        }
        if let Some(page) = self.page {
            params.push(("page".into(), page.to_string()));
        }
        if let Some(per_page) = self.per_page {
            params.push(("per_page".into(), per_page.to_string()));
        }
        if let Some(order_key) = self.order_key {
            params.push(("order_key".into(), order_key.as_str().into()));
        }
        if let Some(order_direction) = self.order_direction {
            params.push(("order_direction".into(), order_direction.to_string()));
        }
        params
    }
}

fn validate_platforms(platforms: Option<&str>) -> Result<()> {
    const SUPPORTED: [&str; 6] = ["macos", "windows", "linux", "ios", "ipados", "android"];
    if platforms.is_some_and(|platforms| {
        platforms.trim().is_empty()
            || platforms
                .split(',')
                .any(|platform| !SUPPORTED.contains(&platform.trim()))
    }) {
        return Err(FleetError::Validation(format!(
            "platform must contain only: {}",
            SUPPORTED.join(", ")
        )));
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetupExperienceSoftwareVersion {
    id: u64,
    version: String,
    #[serde(default)]
    vulnerabilities: Vec<String>,
}

impl SetupExperienceSoftwareVersion {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn vulnerabilities(&self) -> &[String] {
        &self.vulnerabilities
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetupExperienceSoftwarePackage {
    name: String,
    platform: String,
    version: String,
    #[serde(default)]
    self_service: bool,
    #[serde(default)]
    install_during_setup: bool,
}

impl SetupExperienceSoftwarePackage {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn self_service(&self) -> bool {
        self.self_service
    }
    pub fn install_during_setup(&self) -> bool {
        self.install_during_setup
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetupExperienceSoftwareTitle {
    id: u64,
    name: String,
    #[serde(default)]
    icon_url: Option<String>,
    #[serde(default)]
    software_package: Option<SetupExperienceSoftwarePackage>,
    #[serde(default)]
    app_store_app: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default)]
    versions_count: u64,
    #[serde(default)]
    source: String,
    #[serde(default)]
    hosts_count: u64,
    #[serde(default)]
    versions: Vec<SetupExperienceSoftwareVersion>,
}

impl SetupExperienceSoftwareTitle {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
    pub fn software_package(&self) -> Option<&SetupExperienceSoftwarePackage> {
        self.software_package.as_ref()
    }
    /// App Store metadata is intentionally open-ended across Fleet/Apple versions.
    pub fn app_store_app(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.app_store_app.as_ref()
    }
    pub fn versions_count(&self) -> u64 {
        self.versions_count
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn hosts_count(&self) -> u64 {
        self.hosts_count
    }
    pub fn versions(&self) -> &[SetupExperienceSoftwareVersion] {
        &self.versions
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListSetupExperienceSoftwareResponse {
    #[serde(default)]
    software_titles: Vec<SetupExperienceSoftwareTitle>,
    #[serde(default)]
    count: u64,
    #[serde(default)]
    counts_updated_at: Option<String>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListSetupExperienceSoftwareResponse {
    pub fn software_titles(&self) -> &[SetupExperienceSoftwareTitle] {
        &self.software_titles
    }
    pub fn count(&self) -> u64 {
        self.count
    }
    pub fn counts_updated_at(&self) -> Option<&str> {
        self.counts_updated_at.as_deref()
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

/// A custom automatic enrollment profile. The nested Apple profile is an
/// intentionally open-ended vendor payload.
#[derive(Clone, Debug, Deserialize)]
pub struct AutomaticEnrollmentProfile {
    #[serde(default)]
    team_id: Option<u64>,
    #[serde(default)]
    fleet_id: Option<u64>,
    name: String,
    uploaded_at: String,
    enrollment_profile: serde_json::Map<String, serde_json::Value>,
}

impl AutomaticEnrollmentProfile {
    pub fn fleet_id(&self) -> Option<u64> {
        self.fleet_id.or(self.team_id)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn uploaded_at(&self) -> &str {
        &self.uploaded_at
    }
    pub fn enrollment_profile(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.enrollment_profile
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UpdateAutomaticEnrollmentProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<u64>,
    pub name: String,
    pub enrollment_profile: serde_json::Map<String, serde_json::Value>,
}

impl UpdateAutomaticEnrollmentProfileRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(FleetError::Validation(
                "profile name must not be empty".into(),
            ));
        }
        if self.enrollment_profile.is_empty() {
            return Err(FleetError::Validation(
                "enrollment_profile must not be empty".into(),
            ));
        }
        Ok(())
    }
}
