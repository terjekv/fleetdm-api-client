use base64::Engine;
use serde::{Deserialize, Serialize, Serializer};

use crate::error::{FleetError, Result};
use crate::models::common::{FileUpload, OrderDirection, PaginationMeta, validate_list_options};

const MAX_BATCH_BODY_SIZE: usize = 25 * 1024 * 1024;
const MAX_PROFILE_SIZE: usize = 1536 * 1024;

/// A configuration profile returned by Fleet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationProfile {
    profile_uuid: String,
    #[serde(default)]
    team_id: Option<u64>,
    #[serde(default)]
    fleet_id: Option<u64>,
    name: String,
    platform: String,
    #[serde(default)]
    identifier: Option<String>,
    created_at: String,
    updated_at: String,
    #[serde(default)]
    checksum: Option<String>,
    #[serde(default)]
    labels_include_all: Vec<ConfigurationProfileLabel>,
    #[serde(default)]
    labels_include_any: Vec<ConfigurationProfileLabel>,
    #[serde(default)]
    labels_exclude_any: Vec<ConfigurationProfileLabel>,
}

/// A label assignment returned with a configuration profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationProfileLabel {
    name: String,
    #[serde(default)]
    id: Option<u64>,
    #[serde(default)]
    broken: bool,
}

/// Multipart request used to upload a configuration profile.
#[derive(Debug, Clone)]
pub struct CreateConfigurationProfileRequest {
    pub profile: FileUpload,
    pub fleet_id: Option<u64>,
    pub labels_include_all: Vec<String>,
    pub labels_include_any: Vec<String>,
    pub labels_exclude_any: Vec<String>,
}

impl CreateConfigurationProfileRequest {
    /// Create a request targeting all hosts in the selected fleet.
    pub fn new(profile: FileUpload) -> Self {
        Self {
            profile,
            fleet_id: None,
            labels_include_all: Vec::new(),
            labels_include_any: Vec::new(),
            labels_exclude_any: Vec::new(),
        }
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.profile.bytes().len() > MAX_PROFILE_SIZE {
            return Err(FleetError::Validation(format!(
                "configuration profile exceeds Fleet's {MAX_PROFILE_SIZE}-byte upload limit"
            )));
        }
        let filename = self.profile.filename().to_ascii_lowercase();
        if ![".mobileconfig", ".json", ".xml"]
            .iter()
            .any(|extension| filename.ends_with(extension))
        {
            return Err(FleetError::Validation(
                "configuration profile must be a .mobileconfig, .json, or .xml file".into(),
            ));
        }
        if [
            !self.labels_include_all.is_empty(),
            !self.labels_include_any.is_empty(),
            !self.labels_exclude_any.is_empty(),
        ]
        .into_iter()
        .filter(|used| *used)
        .count()
            > 1
        {
            return Err(FleetError::Validation(
                "configuration profile label targeting modes are mutually exclusive".into(),
            ));
        }
        Ok(())
    }
}

/// One file and its optional label targeting in a batch profile update.
#[derive(Debug, Clone, Serialize)]
pub struct BatchConfigurationProfile {
    #[serde(serialize_with = "serialize_upload_as_base64")]
    pub profile: FileUpload,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_all: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_any: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_exclude_any: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl BatchConfigurationProfile {
    pub fn new(profile: FileUpload) -> Self {
        Self {
            profile,
            labels_include_all: Vec::new(),
            labels_include_any: Vec::new(),
            labels_exclude_any: Vec::new(),
            display_name: None,
        }
    }

    fn validate(&self) -> Result<()> {
        if [
            !self.labels_include_all.is_empty(),
            !self.labels_include_any.is_empty(),
            !self.labels_exclude_any.is_empty(),
        ]
        .into_iter()
        .filter(|used| *used)
        .count()
            > 1
        {
            return Err(FleetError::Validation(
                "configuration profile label targeting modes are mutually exclusive".into(),
            ));
        }
        if self
            .display_name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(FleetError::Validation(
                "display_name must not be empty when provided".into(),
            ));
        }
        Ok(())
    }
}

fn serialize_upload_as_base64<S>(
    upload: &FileUpload,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(upload.bytes()))
}

/// Request for replacing the active configuration profiles for a fleet.
#[derive(Debug, Clone)]
pub struct BatchConfigurationProfilesRequest {
    pub fleet_id: Option<u64>,
    pub fleet_name: Option<String>,
    pub dry_run: bool,
    pub configuration_profiles: Vec<BatchConfigurationProfile>,
}

impl BatchConfigurationProfilesRequest {
    pub fn new(configuration_profiles: Vec<BatchConfigurationProfile>) -> Self {
        Self {
            fleet_id: None,
            fleet_name: None,
            dry_run: false,
            configuration_profiles,
        }
    }

    pub(crate) fn validate_and_body(&self) -> Result<Vec<u8>> {
        if self.fleet_id.is_some() && self.fleet_name.is_some() {
            return Err(FleetError::Validation(
                "fleet_id and fleet_name are mutually exclusive".into(),
            ));
        }
        if self
            .fleet_name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        {
            return Err(FleetError::Validation(
                "fleet_name must not be empty when provided".into(),
            ));
        }
        for profile in &self.configuration_profiles {
            profile.validate()?;
        }
        let body = serde_json::to_vec(&serde_json::json!({
            "configuration_profiles": self.configuration_profiles,
        }))?;
        if body.len() > MAX_BATCH_BODY_SIZE {
            return Err(FleetError::Validation(format!(
                "configuration profile batch exceeds Fleet's {MAX_BATCH_BODY_SIZE}-byte request limit"
            )));
        }
        Ok(body)
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(fleet_id) = self.fleet_id {
            params.push(("fleet_id".into(), fleet_id.to_string()));
        }
        if let Some(fleet_name) = &self.fleet_name {
            params.push(("fleet_name".into(), fleet_name.clone()));
        }
        if self.dry_run {
            params.push(("dry_run".into(), "true".into()));
        }
        params
    }
}

/// Request body accepted by Fleet's batch profile resend route.
#[derive(Debug, Clone, Serialize)]
pub struct ResendConfigurationProfilesRequest {
    pub profile_uuid: String,
    pub filters: ResendConfigurationProfileFilters,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResendConfigurationProfileFilters {
    pub profile_status: ResendConfigurationProfileStatus,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResendConfigurationProfileStatus {
    Failed,
}

impl ResendConfigurationProfilesRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.profile_uuid.trim().is_empty() {
            return Err(FleetError::Validation(
                "profile_uuid must not be empty".into(),
            ));
        }
        Ok(())
    }
}

/// Response returned after uploading a configuration profile.
#[derive(Debug, Deserialize)]
pub struct CreateConfigurationProfileResponse {
    profile_uuid: String,
}

/// Paginated configuration-profile list.
#[derive(Debug, Deserialize)]
pub struct ListConfigurationProfilesResponse {
    #[serde(default)]
    profiles: Vec<ConfigurationProfile>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationProfileOrderKey {
    Name,
    CreatedAt,
    UploadedAt,
}

impl ConfigurationProfileOrderKey {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::CreatedAt => "created_at",
            Self::UploadedAt => "uploaded_at",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListConfigurationProfilesQuery {
    pub fleet_id: Option<u64>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<ConfigurationProfileOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub after: Option<String>,
}

impl ListConfigurationProfilesQuery {
    pub(crate) fn validate(&self) -> Result<()> {
        validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            self.after.is_some(),
        )
    }

    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
        if let Some(after) = &self.after {
            params.push(("after".into(), after.clone()));
        }
        params
    }
}

/// Metadata returned for one configuration profile.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetConfigurationProfileResponse {
    profile: ConfigurationProfile,
}

/// Aggregate configuration-profile status counts.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigurationProfileStatus {
    #[serde(default)]
    verified: u64,
    #[serde(default)]
    verifying: u64,
    #[serde(default)]
    failed: u64,
    #[serde(default)]
    pending: u64,
}

impl ConfigurationProfileStatus {
    pub fn verified(&self) -> u64 {
        self.verified
    }
    pub fn verifying(&self) -> u64 {
        self.verifying
    }
    pub fn failed(&self) -> u64 {
        self.failed
    }
    pub fn pending(&self) -> u64 {
        self.pending
    }
}

/// Disk-encryption update request.
#[derive(Debug, Serialize)]
pub struct UpdateDiskEncryptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<u64>,
    pub enable_disk_encryption: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_require_bitlocker_pin: Option<bool>,
}

impl UpdateDiskEncryptionRequest {
    pub(crate) fn validate(&self) -> Result<()> {
        if !self.enable_disk_encryption && self.windows_require_bitlocker_pin == Some(true) {
            return Err(FleetError::Validation(
                "windows_require_bitlocker_pin requires disk encryption to be enabled".into(),
            ));
        }
        Ok(())
    }
}

/// Per-platform counts in Fleet's disk-encryption summary.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DiskEncryptionPlatformCounts {
    #[serde(default)]
    macos: u64,
    #[serde(default)]
    windows: u64,
    #[serde(default)]
    linux: u64,
}

impl DiskEncryptionPlatformCounts {
    pub fn macos(&self) -> u64 {
        self.macos
    }
    pub fn windows(&self) -> u64 {
        self.windows
    }
    pub fn linux(&self) -> u64 {
        self.linux
    }
}

/// Aggregate disk-encryption enforcement status.
#[derive(Debug, Clone, Deserialize)]
pub struct DiskEncryptionSummary {
    #[serde(default)]
    verified: DiskEncryptionPlatformCounts,
    #[serde(default)]
    verifying: DiskEncryptionPlatformCounts,
    #[serde(default)]
    action_required: DiskEncryptionPlatformCounts,
    #[serde(default)]
    enforcing: DiskEncryptionPlatformCounts,
    #[serde(default)]
    failed: DiskEncryptionPlatformCounts,
    #[serde(default)]
    removing_enforcement: DiskEncryptionPlatformCounts,
}

impl DiskEncryptionSummary {
    pub fn verified(&self) -> &DiskEncryptionPlatformCounts {
        &self.verified
    }
    pub fn verifying(&self) -> &DiskEncryptionPlatformCounts {
        &self.verifying
    }
    pub fn action_required(&self) -> &DiskEncryptionPlatformCounts {
        &self.action_required
    }
    pub fn enforcing(&self) -> &DiskEncryptionPlatformCounts {
        &self.enforcing
    }
    pub fn failed(&self) -> &DiskEncryptionPlatformCounts {
        &self.failed
    }
    pub fn removing_enforcement(&self) -> &DiskEncryptionPlatformCounts {
        &self.removing_enforcement
    }
}

impl ConfigurationProfile {
    pub fn profile_uuid(&self) -> &str {
        &self.profile_uuid
    }

    /// Current Fleet identifier, falling back to the legacy team identifier.
    pub fn fleet_id(&self) -> Option<u64> {
        self.fleet_id.or(self.team_id)
    }

    #[deprecated(note = "use fleet_id()")]
    pub fn team_id(&self) -> Option<u64> {
        self.fleet_id()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn platform(&self) -> &str {
        &self.platform
    }

    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    pub fn labels_include_all(&self) -> &[ConfigurationProfileLabel] {
        &self.labels_include_all
    }

    pub fn labels_include_any(&self) -> &[ConfigurationProfileLabel] {
        &self.labels_include_any
    }

    pub fn labels_exclude_any(&self) -> &[ConfigurationProfileLabel] {
        &self.labels_exclude_any
    }
}

impl ConfigurationProfileLabel {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn broken(&self) -> bool {
        self.broken
    }
}

impl CreateConfigurationProfileResponse {
    pub fn profile_uuid(&self) -> &str {
        &self.profile_uuid
    }
}

impl ListConfigurationProfilesResponse {
    pub fn profiles(&self) -> &[ConfigurationProfile] {
        &self.profiles
    }

    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

impl GetConfigurationProfileResponse {
    pub fn profile(&self) -> &ConfigurationProfile {
        &self.profile
    }
}
