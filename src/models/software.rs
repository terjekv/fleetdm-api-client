use crate::models::common::OrderDirection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::error::{FleetError, Result as FleetResult};
use crate::models::common::{FileUpload, PaginationMeta};

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

fn deserialize_empty_string_or_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    match value.as_deref() {
        None | Some("") => Ok(None),
        Some(value) => DateTime::parse_from_rfc3339(value)
            .map(|dt| Some(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SoftwareOrderKey {
    Name,
    HostsCount,
    VulnerabilitiesCount,
}

impl SoftwareOrderKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            SoftwareOrderKey::Name => "name",
            SoftwareOrderKey::HostsCount => "hosts_count",
            SoftwareOrderKey::VulnerabilitiesCount => "vulnerabilities_count",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Software {
    id: u64,
    name: String,
    version: String,
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser: Option<String>,
    generated_cpe: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    vulnerabilities: Vec<SoftwareVulnerability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts_count: Option<u32>,
    #[serde(
        default,
        deserialize_with = "deserialize_empty_string_or_datetime",
        skip_serializing_if = "Option::is_none"
    )]
    last_opened_at: Option<DateTime<Utc>>,
}

impl Software {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn bundle_identifier(&self) -> Option<&str> {
        self.bundle_identifier.as_deref()
    }
    pub fn vendor(&self) -> Option<&str> {
        self.vendor.as_deref()
    }
    pub fn browser(&self) -> Option<&str> {
        self.browser.as_deref()
    }
    pub fn generated_cpe(&self) -> &str {
        &self.generated_cpe
    }
    pub fn vulnerabilities(&self) -> &[SoftwareVulnerability] {
        &self.vulnerabilities
    }
    pub fn hosts_count(&self) -> Option<u32> {
        self.hosts_count
    }
    pub fn last_opened_at(&self) -> Option<DateTime<Utc>> {
        self.last_opened_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareVulnerability {
    cve: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details_link: Option<String>,
}

impl SoftwareVulnerability {
    pub fn cve(&self) -> &str {
        &self.cve
    }
    pub fn details_link(&self) -> Option<&str> {
        self.details_link.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSoftwareResponse {
    #[serde(default)]
    software: Vec<Software>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts_updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

/// A version summarized under a software title.
#[derive(Debug, Clone, Deserialize)]
pub struct SoftwareTitleVersion {
    id: u64,
    version: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    vulnerabilities: Vec<SoftwareVulnerability>,
    #[serde(default)]
    hosts_count: Option<u64>,
}

impl SoftwareTitleVersion {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn vulnerabilities(&self) -> &[SoftwareVulnerability] {
        &self.vulnerabilities
    }
    pub fn hosts_count(&self) -> Option<u64> {
        self.hosts_count
    }
}

/// A software title returned by Fleet's current title routes.
#[derive(Debug, Clone, Deserialize)]
pub struct SoftwareTitle {
    id: u64,
    name: String,
    #[serde(default)]
    icon_url: Option<String>,
    #[serde(default)]
    source: String,
    #[serde(default)]
    extension_for: String,
    #[serde(default)]
    browser: String,
    #[serde(default)]
    hosts_count: u64,
    #[serde(default)]
    versions_count: u64,
    #[serde(default)]
    versions: Vec<SoftwareTitleVersion>,
    #[serde(default)]
    counts_updated_at: Option<String>,
    #[serde(default)]
    software_package: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default)]
    app_store_app: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default)]
    bundle_identifier: Option<String>,
    #[serde(default)]
    hash_sha256: Option<String>,
    #[serde(default)]
    display_name: String,
}

impl SoftwareTitle {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn extension_for(&self) -> &str {
        &self.extension_for
    }
    pub fn browser(&self) -> &str {
        &self.browser
    }
    pub fn hosts_count(&self) -> u64 {
        self.hosts_count
    }
    pub fn versions_count(&self) -> u64 {
        self.versions_count
    }
    pub fn versions(&self) -> &[SoftwareTitleVersion] {
        &self.versions
    }
    pub fn counts_updated_at(&self) -> Option<&str> {
        self.counts_updated_at.as_deref()
    }
    /// Installer metadata varies across package platforms and Fleet versions.
    pub fn software_package(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.software_package.as_ref()
    }
    /// Store metadata may include vendor-specific managed configuration.
    pub fn app_store_app(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.app_store_app.as_ref()
    }
    pub fn bundle_identifier(&self) -> Option<&str> {
        self.bundle_identifier.as_deref()
    }
    pub fn hash_sha256(&self) -> Option<&str> {
        self.hash_sha256.as_deref()
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSoftwareTitlesResponse {
    #[serde(default)]
    software_titles: Vec<SoftwareTitle>,
    #[serde(default)]
    count: u64,
    #[serde(default)]
    counts_updated_at: Option<String>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListSoftwareTitlesResponse {
    pub fn software_titles(&self) -> &[SoftwareTitle] {
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

#[derive(Debug, Clone, Deserialize)]
pub struct GetSoftwareTitleResponse {
    software_title: SoftwareTitle,
}

impl GetSoftwareTitleResponse {
    pub fn software_title(&self) -> &SoftwareTitle {
        &self.software_title
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OperatingSystemKernel {
    id: u64,
    version: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    vulnerabilities: Vec<String>,
    #[serde(default)]
    hosts_count: u64,
}

impl OperatingSystemKernel {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn vulnerabilities(&self) -> &[String] {
        &self.vulnerabilities
    }
    pub fn hosts_count(&self) -> u64 {
        self.hosts_count
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OperatingSystemVersion {
    os_version_id: u64,
    hosts_count: u64,
    name: String,
    name_only: String,
    version: String,
    platform: String,
    #[serde(default)]
    generated_cpes: Vec<String>,
    #[serde(default)]
    vulnerabilities_count: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    vulnerabilities: Vec<SoftwareVulnerability>,
    #[serde(default)]
    kernels: Vec<OperatingSystemKernel>,
}

impl OperatingSystemVersion {
    pub fn os_version_id(&self) -> u64 {
        self.os_version_id
    }
    pub fn hosts_count(&self) -> u64 {
        self.hosts_count
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn name_only(&self) -> &str {
        &self.name_only
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn generated_cpes(&self) -> &[String] {
        &self.generated_cpes
    }
    pub fn vulnerabilities_count(&self) -> Option<u64> {
        self.vulnerabilities_count
    }
    pub fn vulnerabilities(&self) -> &[SoftwareVulnerability] {
        &self.vulnerabilities
    }
    pub fn kernels(&self) -> &[OperatingSystemKernel] {
        &self.kernels
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListOperatingSystemVersionsResponse {
    #[serde(default)]
    os_versions: Vec<OperatingSystemVersion>,
    #[serde(default)]
    count: u64,
    #[serde(default)]
    counts_updated_at: Option<String>,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListOperatingSystemVersionsResponse {
    pub fn os_versions(&self) -> &[OperatingSystemVersion] {
        &self.os_versions
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

#[derive(Debug, Clone, Deserialize)]
pub struct GetOperatingSystemVersionResponse {
    #[serde(default)]
    counts_updated_at: Option<String>,
    os_version: OperatingSystemVersion,
}

impl GetOperatingSystemVersionResponse {
    pub fn counts_updated_at(&self) -> Option<&str> {
        self.counts_updated_at.as_deref()
    }
    pub fn os_version(&self) -> &OperatingSystemVersion {
        &self.os_version
    }
}

impl ListSoftwareResponse {
    pub fn software(&self) -> &[Software] {
        &self.software
    }
    pub fn count(&self) -> Option<u32> {
        self.count
    }
    pub fn counts_updated_at(&self) -> Option<DateTime<Utc>> {
        self.counts_updated_at
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

/// Optional fields shared by software-package upload and update requests.
#[derive(Clone, Default)]
pub struct SoftwarePackageOptions {
    pub fleet_id: Option<u64>,
    pub install_script: Option<String>,
    pub uninstall_script: Option<String>,
    pub pre_install_query: Option<String>,
    pub post_install_script: Option<String>,
    pub self_service: Option<bool>,
    pub automatic_install: Option<bool>,
    pub display_name: Option<String>,
    pub version: Option<String>,
    pub labels_include_all: Vec<String>,
    pub labels_include_any: Vec<String>,
    pub labels_exclude_any: Vec<String>,
    pub categories: Vec<String>,
}

impl SoftwarePackageOptions {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        let targeting_modes = [
            !self.labels_include_all.is_empty(),
            !self.labels_include_any.is_empty(),
            !self.labels_exclude_any.is_empty(),
        ]
        .into_iter()
        .filter(|set| *set)
        .count();
        if targeting_modes > 1 {
            return Err(FleetError::Validation(
                "only one labels_include_all, labels_include_any, or labels_exclude_any mode may be used"
                    .into(),
            ));
        }
        if [
            self.install_script.as_deref(),
            self.uninstall_script.as_deref(),
            self.pre_install_query.as_deref(),
            self.post_install_script.as_deref(),
            self.display_name.as_deref(),
            self.version.as_deref(),
        ]
        .into_iter()
        .flatten()
        .any(|value| value.trim().is_empty())
        {
            return Err(FleetError::Validation(
                "software package options must not be empty when provided".into(),
            ));
        }
        if self
            .labels_include_all
            .iter()
            .chain(&self.labels_include_any)
            .chain(&self.labels_exclude_any)
            .chain(&self.categories)
            .any(|value| value.trim().is_empty())
        {
            return Err(FleetError::Validation(
                "software labels and categories must not contain empty values".into(),
            ));
        }
        Ok(())
    }

    fn has_update(&self) -> bool {
        self.install_script.is_some()
            || self.uninstall_script.is_some()
            || self.pre_install_query.is_some()
            || self.post_install_script.is_some()
            || self.self_service.is_some()
            || self.automatic_install.is_some()
            || self.display_name.is_some()
            || self.version.is_some()
            || !self.labels_include_all.is_empty()
            || !self.labels_include_any.is_empty()
            || !self.labels_exclude_any.is_empty()
            || !self.categories.is_empty()
    }
}

/// Request for adding a custom software package.
#[derive(Clone)]
pub struct UploadSoftwarePackageRequest {
    pub software: FileUpload,
    pub options: SoftwarePackageOptions,
}

impl UploadSoftwarePackageRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        self.options.validate()?;
        let filename = self.software.filename().to_ascii_lowercase();
        if ![
            ".pkg", ".msi", ".exe", ".deb", ".rpm", ".tar.gz", ".ipa", ".sh", ".ps1",
        ]
        .iter()
        .any(|extension| filename.ends_with(extension))
        {
            return Err(FleetError::Validation(
                "unsupported software package extension".into(),
            ));
        }
        Ok(())
    }
}

/// Request for updating a custom software package.
#[derive(Clone)]
pub struct UpdateSoftwarePackageRequest {
    pub fleet_id: u64,
    pub software: Option<FileUpload>,
    pub options: SoftwarePackageOptions,
}

impl UpdateSoftwarePackageRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        self.options.validate()?;
        if self.options.fleet_id.is_some() {
            return Err(FleetError::Validation(
                "use UpdateSoftwarePackageRequest::fleet_id instead of options.fleet_id".into(),
            ));
        }
        if self.software.is_none() && !self.options.has_update() {
            return Err(FleetError::Validation(
                "software package update must change at least one field".into(),
            ));
        }
        if let Some(software) = &self.software {
            let filename = software.filename().to_ascii_lowercase();
            if ![
                ".pkg", ".msi", ".exe", ".deb", ".rpm", ".tar.gz", ".ipa", ".sh", ".ps1",
            ]
            .iter()
            .any(|extension| filename.ends_with(extension))
            {
                return Err(FleetError::Validation(
                    "unsupported software package extension".into(),
                ));
            }
        }
        Ok(())
    }
}

/// A custom package returned after an upload or update.
#[derive(Debug, Clone, Deserialize)]
pub struct SoftwarePackage {
    #[serde(default)]
    team_id: Option<u64>,
    #[serde(default)]
    fleet_id: Option<u64>,
    title_id: u64,
    name: String,
    platform: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    hash_sha256: Option<String>,
    #[serde(default)]
    installer_id: Option<u64>,
}

/// Response returned after adding or updating a package.
#[derive(Debug, Clone, Deserialize)]
pub struct SoftwarePackageResponse {
    #[serde(alias = "software_installer")]
    software_package: SoftwarePackage,
}

impl SoftwarePackageResponse {
    pub fn software_package(&self) -> &SoftwarePackage {
        &self.software_package
    }
}

impl SoftwarePackage {
    pub fn fleet_id(&self) -> Option<u64> {
        self.fleet_id.or(self.team_id)
    }
    pub fn title_id(&self) -> u64 {
        self.title_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    pub fn hash_sha256(&self) -> Option<&str> {
        self.hash_sha256.as_deref()
    }
    pub fn installer_id(&self) -> Option<u64> {
        self.installer_id
    }
}

/// Source for a software icon update.
#[derive(Debug, Clone)]
pub enum SoftwareIconSource {
    Png(FileUpload),
    Existing {
        hash_sha256: String,
        filename: String,
    },
}

/// Request for setting a software title icon.
#[derive(Debug, Clone)]
pub struct UpdateSoftwareIconRequest {
    pub fleet_id: u64,
    pub source: SoftwareIconSource,
}

impl UpdateSoftwareIconRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        match &self.source {
            SoftwareIconSource::Png(file)
                if !file.filename().to_ascii_lowercase().ends_with(".png") =>
            {
                Err(FleetError::Validation(
                    "software icon must be a PNG file".into(),
                ))
            }
            SoftwareIconSource::Existing {
                hash_sha256,
                filename,
            } if hash_sha256.trim().is_empty() || filename.trim().is_empty() => Err(
                FleetError::Validation("existing icon hash and filename must not be empty".into()),
            ),
            _ => Ok(()),
        }
    }
}

/// Response returned after setting a software icon.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSoftwareIconResponse {
    icon_url: String,
}

impl UpdateSoftwareIconResponse {
    pub fn icon_url(&self) -> &str {
        &self.icon_url
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppStoreApp {
    name: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    icon_url: Option<String>,
    #[serde(default)]
    latest_version: String,
    #[serde(deserialize_with = "deserialize_string_or_number")]
    app_store_id: String,
    platform: String,
}

fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(value) => Ok(value),
        serde_json::Value::Number(value) => Ok(value.to_string()),
        _ => Err(serde::de::Error::custom("expected a string or number")),
    }
}

impl AppStoreApp {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
    pub fn latest_version(&self) -> &str {
        &self.latest_version
    }
    pub fn app_store_id(&self) -> &str {
        &self.app_store_id
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAppStoreAppsResponse {
    #[serde(default)]
    app_store_apps: Vec<AppStoreApp>,
}

impl ListAppStoreAppsResponse {
    pub fn app_store_apps(&self) -> &[AppStoreApp] {
        &self.app_store_apps
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateAppStoreAppRequest {
    pub app_store_id: String,
    pub fleet_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensure: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_all: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_any: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_exclude_any: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Map<String, serde_json::Value>>,
}

impl CreateAppStoreAppRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        validate_app_targeting(
            &self.labels_include_all,
            &self.labels_include_any,
            &self.labels_exclude_any,
        )?;
        if self.app_store_id.trim().is_empty() {
            return Err(FleetError::Validation(
                "app_store_id must not be empty".into(),
            ));
        }
        if self
            .ensure
            .as_deref()
            .is_some_and(|ensure| ensure != "present")
        {
            return Err(FleetError::Validation(
                "ensure must be 'present' when provided".into(),
            ));
        }
        if self.platform.as_deref() == Some("android") && self.self_service.is_none() {
            return Err(FleetError::Validation(
                "self_service is required for Android apps".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAppStoreAppRequest {
    pub fleet_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_window_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_window_end: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_all: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_any: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_exclude_any: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Map<String, serde_json::Value>>,
}

impl UpdateAppStoreAppRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        validate_app_targeting(
            &self.labels_include_all,
            &self.labels_include_any,
            &self.labels_exclude_any,
        )?;
        if self.display_name.is_none()
            && self.self_service.is_none()
            && self.auto_update_enabled.is_none()
            && self.auto_update_window_start.is_none()
            && self.auto_update_window_end.is_none()
            && self.labels_include_all.is_empty()
            && self.labels_include_any.is_empty()
            && self.labels_exclude_any.is_empty()
            && self.categories.is_empty()
            && self.configuration.is_none()
        {
            return Err(FleetError::Validation(
                "app update must change at least one field".into(),
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
        if self.auto_update_window_start.is_some() != self.auto_update_window_end.is_some() {
            return Err(FleetError::Validation(
                "automatic update windows require both start and end times".into(),
            ));
        }
        if self.auto_update_enabled == Some(true)
            && (self.auto_update_window_start.is_none() || self.auto_update_window_end.is_none())
        {
            return Err(FleetError::Validation(
                "automatic updates require both update-window times".into(),
            ));
        }
        Ok(())
    }
}

fn validate_app_targeting(all: &[String], any: &[String], exclude: &[String]) -> FleetResult<()> {
    let modes = [!all.is_empty(), !any.is_empty(), !exclude.is_empty()]
        .into_iter()
        .filter(|used| *used)
        .count();
    if modes > 1 {
        return Err(FleetError::Validation(
            "only one label targeting mode may be used".into(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAppStoreAppResponse {
    software_title_id: u64,
    name: String,
}

impl CreateAppStoreAppResponse {
    pub fn software_title_id(&self) -> u64 {
        self.software_title_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAppStoreAppResponse {
    app_store_app: serde_json::Map<String, serde_json::Value>,
}

impl UpdateAppStoreAppResponse {
    /// Updated store metadata, including vendor-specific configuration fields.
    pub fn app_store_app(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.app_store_app
    }
}

#[derive(Clone, Deserialize)]
pub struct FleetMaintainedApp {
    id: u64,
    name: String,
    slug: String,
    platform: String,
    version: String,
    #[serde(default)]
    filename: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    install_script: Option<String>,
    #[serde(default)]
    uninstall_script: Option<String>,
    #[serde(default)]
    software_title_id: Option<u64>,
    #[serde(default)]
    categories: Vec<String>,
}

impl FleetMaintainedApp {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn slug(&self) -> &str {
        &self.slug
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    pub fn install_script(&self) -> Option<&str> {
        self.install_script.as_deref()
    }
    pub fn uninstall_script(&self) -> Option<&str> {
        self.uninstall_script.as_deref()
    }
    pub fn software_title_id(&self) -> Option<u64> {
        self.software_title_id
    }
    pub fn categories(&self) -> &[String] {
        &self.categories
    }
}

#[derive(Clone, Deserialize)]
pub struct ListFleetMaintainedAppsResponse {
    #[serde(default)]
    fleet_maintained_apps: Vec<FleetMaintainedApp>,
    #[serde(default)]
    count: u64,
    #[serde(default)]
    meta: Option<PaginationMeta>,
}

impl ListFleetMaintainedAppsResponse {
    pub fn fleet_maintained_apps(&self) -> &[FleetMaintainedApp] {
        &self.fleet_maintained_apps
    }
    pub fn count(&self) -> u64 {
        self.count
    }
    pub fn meta(&self) -> Option<&PaginationMeta> {
        self.meta.as_ref()
    }
}

#[derive(Clone, Deserialize)]
pub struct GetFleetMaintainedAppResponse {
    fleet_maintained_app: FleetMaintainedApp,
}

impl GetFleetMaintainedAppResponse {
    pub fn fleet_maintained_app(&self) -> &FleetMaintainedApp {
        &self.fleet_maintained_app
    }
}

#[derive(Clone, Serialize)]
pub struct CreateFleetMaintainedAppRequest {
    pub fleet_maintained_app_id: u64,
    pub fleet_id: u64,
    #[serde(flatten)]
    pub options: FleetMaintainedAppOptions,
}

#[derive(Clone, Default, Serialize)]
pub struct FleetMaintainedAppOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_install_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_install: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_all: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_include_any: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels_exclude_any: Vec<String>,
}

impl CreateFleetMaintainedAppRequest {
    pub(crate) fn validate(&self) -> FleetResult<()> {
        if self.fleet_maintained_app_id == 0 {
            return Err(FleetError::Validation(
                "fleet_maintained_app_id must be greater than zero".into(),
            ));
        }
        validate_app_targeting(
            &self.options.labels_include_all,
            &self.options.labels_include_any,
            &self.options.labels_exclude_any,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFleetMaintainedAppResponse {
    software_title_id: u64,
}

impl CreateFleetMaintainedAppResponse {
    pub fn software_title_id(&self) -> u64 {
        self.software_title_id
    }
}

/// Result of a custom-package or Fleet-maintained-app installation.
#[derive(Clone, Deserialize)]
pub struct SoftwareInstallResult {
    install_uuid: String,
    software_title: String,
    software_title_id: u64,
    software_package: String,
    host_id: u64,
    host_display_name: String,
    status: String,
    #[serde(default)]
    output: Option<String>,
    #[serde(default)]
    pre_install_query_output: Option<String>,
    #[serde(default)]
    post_install_script_output: Option<String>,
}

impl SoftwareInstallResult {
    pub fn install_uuid(&self) -> &str {
        &self.install_uuid
    }
    pub fn software_title(&self) -> &str {
        &self.software_title
    }
    pub fn software_title_id(&self) -> u64 {
        self.software_title_id
    }
    pub fn software_package(&self) -> &str {
        &self.software_package
    }
    pub fn host_id(&self) -> u64 {
        self.host_id
    }
    pub fn host_display_name(&self) -> &str {
        &self.host_display_name
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
    }
    pub fn pre_install_query_output(&self) -> Option<&str> {
        self.pre_install_query_output.as_deref()
    }
    pub fn post_install_script_output(&self) -> Option<&str> {
        self.post_install_script_output.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSoftwareResponse {
    software: Software,
}

impl GetSoftwareResponse {
    pub fn software(&self) -> &Software {
        &self.software
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListSoftwareQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_key: Option<SoftwareOrderKey>,
    pub order_direction: Option<OrderDirection>,
    pub team_id: Option<u64>,
    pub fleet_id: Option<u64>,
    pub query: Option<String>,
    pub vulnerable: Option<bool>,
}

impl ListSoftwareQuery {
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
    pub fn order_key(mut self, key: SoftwareOrderKey) -> Self {
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
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn vulnerable(mut self, vulnerable: bool) -> Self {
        self.vulnerable = Some(vulnerable);
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
        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
        }
        if let Some(vulnerable) = self.vulnerable {
            params.push(("vulnerable".to_string(), vulnerable.to_string()));
        }
        params
    }

    pub(crate) fn validate(&self) -> FleetResult<()> {
        crate::models::common::validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            false,
        )
    }
}
