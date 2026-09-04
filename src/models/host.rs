use crate::error::Result;
use crate::models::common::{OrderDirection, PaginationMeta, validate_list_options};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumString};

/// Host status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HostStatus {
    Online,
    Offline,
    New,
    Mia,
    Missing,
}

/// Valid order keys for host queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HostOrderKey {
    Hostname,
    ComputerName,
    HardwareModel,
    Status,
    OsVersion,
    CreatedAt,
    UpdatedAt,
}

impl HostOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            HostOrderKey::Hostname => "hostname",
            HostOrderKey::ComputerName => "computer_name",
            HostOrderKey::HardwareModel => "hardware_model",
            HostOrderKey::Status => "status",
            HostOrderKey::OsVersion => "os_version",
            HostOrderKey::CreatedAt => "created_at",
            HostOrderKey::UpdatedAt => "updated_at",
        }
    }
}

/// Structured operating system information returned by some host endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedOsVersion {
    name: String,
    version: String,
    platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_version: Option<String>,
}

/// Host OS version can be either a structured object or a plain version string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OsVersion {
    Detailed(DetailedOsVersion),
    String(String),
}

impl OsVersion {
    pub fn name(&self) -> &str {
        match self {
            OsVersion::Detailed(version) => &version.name,
            OsVersion::String(version) => version.as_str(),
        }
    }
    pub fn version(&self) -> &str {
        match self {
            OsVersion::Detailed(version) => &version.version,
            OsVersion::String(_) => "",
        }
    }
    pub fn platform(&self) -> &str {
        match self {
            OsVersion::Detailed(version) => &version.platform,
            OsVersion::String(_) => "",
        }
    }
    pub fn build(&self) -> Option<&str> {
        match self {
            OsVersion::Detailed(version) => version.build.as_deref(),
            OsVersion::String(_) => None,
        }
    }
    pub fn kernel_version(&self) -> Option<&str> {
        match self {
            OsVersion::Detailed(version) => version.kernel_version.as_deref(),
            OsVersion::String(_) => None,
        }
    }
    pub fn raw(&self) -> &str {
        match self {
            OsVersion::Detailed(version) => &version.version,
            OsVersion::String(version) => version.as_str(),
        }
    }
}

/// MDM enrollment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdmInfo {
    enrollment_status: Option<String>,
    server_url: Option<String>,
    name: Option<String>,
    id: Option<u64>,
}

impl MdmInfo {
    pub fn enrollment_status(&self) -> Option<&str> {
        self.enrollment_status.as_deref()
    }
    pub fn server_url(&self) -> Option<&str> {
        self.server_url.as_deref()
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn id(&self) -> Option<u64> {
        self.id
    }
}

/// Host device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    id: u64,
    uuid: String,
    hostname: String,
    display_name: String,
    computer_name: String,

    platform: String,
    osquery_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<OsVersion>,

    status: HostStatus,
    uptime: u64,
    memory: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_physical_cores: Option<u32>,

    primary_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_mac: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hardware_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hardware_serial: Option<String>,

    seen_time: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_enrolled_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mdm: Option<MdmInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<HostLabel>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    issues: Option<HostIssues>,
}

impl Host {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn uuid(&self) -> &str {
        &self.uuid
    }
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn computer_name(&self) -> &str {
        &self.computer_name
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn osquery_version(&self) -> &str {
        &self.osquery_version
    }
    pub fn os_version(&self) -> Option<&OsVersion> {
        self.os_version.as_ref()
    }
    pub fn status(&self) -> HostStatus {
        self.status
    }
    pub fn uptime(&self) -> u64 {
        self.uptime
    }
    pub fn memory(&self) -> u64 {
        self.memory
    }
    pub fn cpu_type(&self) -> Option<&str> {
        self.cpu_type.as_deref()
    }
    pub fn cpu_brand(&self) -> Option<&str> {
        self.cpu_brand.as_deref()
    }
    pub fn cpu_physical_cores(&self) -> Option<u32> {
        self.cpu_physical_cores
    }
    pub fn primary_ip(&self) -> &str {
        &self.primary_ip
    }
    pub fn primary_mac(&self) -> Option<&str> {
        self.primary_mac.as_deref()
    }
    pub fn hardware_model(&self) -> Option<&str> {
        self.hardware_model.as_deref()
    }
    pub fn hardware_serial(&self) -> Option<&str> {
        self.hardware_serial.as_deref()
    }
    pub fn seen_time(&self) -> DateTime<Utc> {
        self.seen_time
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn last_enrolled_at(&self) -> Option<DateTime<Utc>> {
        self.last_enrolled_at
    }
    pub fn team_id(&self) -> Option<u64> {
        self.fleet_id()
    }
    pub fn team_name(&self) -> Option<&str> {
        self.fleet_name()
    }
    pub fn fleet_id(&self) -> Option<u64> {
        self.fleet_id.or(self.team_id)
    }
    pub fn fleet_name(&self) -> Option<&str> {
        self.fleet_name.as_deref().or(self.team_name.as_deref())
    }
    pub fn mdm(&self) -> Option<&MdmInfo> {
        self.mdm.as_ref()
    }
    pub fn labels(&self) -> Option<&[HostLabel]> {
        self.labels.as_deref()
    }
    pub fn issues(&self) -> Option<&HostIssues> {
        self.issues.as_ref()
    }
}

/// Label associated with a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostLabel {
    id: u64,
    name: String,
    description: String,
    label_type: String,
}

impl HostLabel {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn label_type(&self) -> &str {
        &self.label_type
    }
}

/// Issues detected on a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostIssues {
    total_issues_count: u32,
    failing_policies_count: u32,
}

impl HostIssues {
    pub fn total_issues_count(&self) -> u32 {
        self.total_issues_count
    }
    pub fn failing_policies_count(&self) -> u32 {
        self.failing_policies_count
    }
}

/// Response for listing hosts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostsResponse {
    hosts: Vec<Host>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListHostsResponse {
    pub fn hosts(&self) -> &[Host] {
        &self.hosts
    }
    pub fn software(&self) -> Option<&[HashMap<String, serde_json::Value>]> {
        self.software.as_deref()
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

/// Response for getting a single host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHostResponse {
    host: Host,
}

impl GetHostResponse {
    pub fn host(&self) -> &Host {
        &self.host
    }
}

/// Response for deleting a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHostResponse {
    #[serde(default)]
    message: String,
}

impl DeleteHostResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Query parameters for listing hosts
#[derive(Debug, Default, Clone)]
pub struct ListHostsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<HostOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub status: Option<HostStatus>,
    pub team_id: Option<u64>,
    pub fleet_id: Option<u64>,
    pub query: Option<String>,
    pub mdm_id: Option<u64>,
    pub mdm_name: Option<String>,
    pub mdm_enrollment_status: Option<String>,
}

impl ListHostsQuery {
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

    pub fn order_key(mut self, key: HostOrderKey) -> Self {
        self.order_key = Some(key);
        self
    }

    pub fn order_direction(mut self, direction: OrderDirection) -> Self {
        self.order_direction = Some(direction);
        self
    }

    pub fn status(mut self, status: HostStatus) -> Self {
        self.status = Some(status);
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

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn mdm_id(mut self, mdm_id: u64) -> Self {
        self.mdm_id = Some(mdm_id);
        self
    }

    pub fn mdm_name(mut self, name: impl Into<String>) -> Self {
        self.mdm_name = Some(name.into());
        self
    }

    pub fn mdm_enrollment_status(mut self, status: impl Into<String>) -> Self {
        self.mdm_enrollment_status = Some(status.into());
        self
    }

    /// Convert to query string parameters
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
        if let Some(status) = self.status {
            let status_str = match status {
                HostStatus::Online => "online",
                HostStatus::Offline => "offline",
                HostStatus::New => "new",
                HostStatus::Mia => "mia",
                HostStatus::Missing => "missing",
            };
            params.push(("status".to_string(), status_str.to_string()));
        }
        if let Some(team_id) = self.team_id {
            params.push(("team_id".to_string(), team_id.to_string()));
        }
        if let Some(fleet_id) = self.fleet_id {
            params.push(("fleet_id".to_string(), fleet_id.to_string()));
        }
        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
        }
        if let Some(mdm_id) = self.mdm_id {
            params.push(("mdm_id".to_string(), mdm_id.to_string()));
        }
        if let Some(ref mdm_name) = self.mdm_name {
            params.push(("mdm_name".to_string(), mdm_name.clone()));
        }
        if let Some(ref mdm_enrollment_status) = self.mdm_enrollment_status {
            params.push((
                "mdm_enrollment_status".to_string(),
                mdm_enrollment_status.clone(),
            ));
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

/// Response for counting hosts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostCountResponse {
    count: u64,
}

impl HostCountResponse {
    pub fn count(&self) -> u64 {
        self.count
    }
}

/// Platform breakdown in host summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCount {
    platform: String,
    hosts_count: u64,
}

impl PlatformCount {
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn hosts_count(&self) -> u64 {
        self.hosts_count
    }
}

/// Response for host summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostSummaryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u64>,
    totals_hosts_count: u64,
    online_count: u64,
    offline_count: u64,
    mia_count: u64,
    missing_30_days_count: u64,
    new_count: u64,
    #[serde(default)]
    all_linux_count: u64,
    #[serde(default)]
    platforms: Vec<PlatformCount>,
}

impl HostSummaryResponse {
    pub fn team_id(&self) -> Option<u64> {
        self.team_id
    }
    pub fn totals_hosts_count(&self) -> u64 {
        self.totals_hosts_count
    }
    pub fn online_count(&self) -> u64 {
        self.online_count
    }
    pub fn offline_count(&self) -> u64 {
        self.offline_count
    }
    pub fn mia_count(&self) -> u64 {
        self.mia_count
    }
    pub fn missing_30_days_count(&self) -> u64 {
        self.missing_30_days_count
    }
    pub fn new_count(&self) -> u64 {
        self.new_count
    }
    pub fn all_linux_count(&self) -> u64 {
        self.all_linux_count
    }
    pub fn platforms(&self) -> &[PlatformCount] {
        &self.platforms
    }
}

// MDM Action Responses

/// Response from locking a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockHostResponse {
    host_id: u64,
}

impl LockHostResponse {
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
}

/// Request to unlock a host
#[derive(Clone, Serialize, Deserialize)]
pub struct UnlockHostRequest {
    pub pin: String,
}

/// Response from unlocking a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockHostResponse {
    host_id: u64,
}

impl UnlockHostResponse {
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
}

/// Response from wiping a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeHostResponse {
    host_id: u64,
}

impl WipeHostResponse {
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mia_status_uses_fleets_wire_value() {
        assert_eq!(serde_json::to_string(&HostStatus::Mia).unwrap(), "\"mia\"");
        assert_eq!(
            serde_json::from_str::<HostStatus>("\"mia\"").unwrap(),
            HostStatus::Mia
        );
    }
}
