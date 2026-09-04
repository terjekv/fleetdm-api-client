use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    org_info: OrgInfo,
    server_settings: ServerSettings,
    smtp_settings: SmtpSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    sso_settings: Option<SsoSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_expiry_settings: Option<HostExpirySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<Features>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_settings: Option<WebhookSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integrations: Option<Integrations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdm: Option<MdmSettings>,
}

impl Config {
    pub fn org_info(&self) -> &OrgInfo {
        &self.org_info
    }
    pub fn server_settings(&self) -> &ServerSettings {
        &self.server_settings
    }
    pub fn smtp_settings(&self) -> &SmtpSettings {
        &self.smtp_settings
    }
    /// SSO settings, when Fleet includes them for the caller and edition.
    pub fn sso_settings(&self) -> Option<&SsoSettings> {
        self.sso_settings.as_ref()
    }
    pub fn host_expiry_settings(&self) -> Option<&HostExpirySettings> {
        self.host_expiry_settings.as_ref()
    }
    pub fn features(&self) -> Option<&Features> {
        self.features.as_ref()
    }
    pub fn agent_options(&self) -> Option<&serde_json::Value> {
        self.agent_options.as_ref()
    }
    pub fn webhook_settings(&self) -> Option<&WebhookSettings> {
        self.webhook_settings.as_ref()
    }
    pub fn integrations(&self) -> Option<&Integrations> {
        self.integrations.as_ref()
    }
    pub fn mdm(&self) -> Option<&MdmSettings> {
        self.mdm.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgInfo {
    org_name: String,
    org_logo_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_logo_url_light_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_url: Option<String>,
}

impl OrgInfo {
    pub fn org_name(&self) -> &str {
        &self.org_name
    }
    pub fn org_logo_url(&self) -> &str {
        &self.org_logo_url
    }
    pub fn org_logo_url_light_background(&self) -> Option<&str> {
        self.org_logo_url_light_background.as_deref()
    }
    pub fn contact_url(&self) -> Option<&str> {
        self.contact_url.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    server_url: String,
    live_query_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_analytics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deferred_save_host: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_reports_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scripts_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ai_features_disabled: Option<bool>,
}

impl ServerSettings {
    pub fn server_url(&self) -> &str {
        &self.server_url
    }
    pub fn live_query_disabled(&self) -> bool {
        self.live_query_disabled
    }
    pub fn enable_analytics(&self) -> Option<bool> {
        self.enable_analytics
    }
    pub fn deferred_save_host(&self) -> Option<bool> {
        self.deferred_save_host
    }
    pub fn query_reports_disabled(&self) -> Option<bool> {
        self.query_reports_disabled
    }
    pub fn scripts_disabled(&self) -> Option<bool> {
        self.scripts_disabled
    }
    pub fn ai_features_disabled(&self) -> Option<bool> {
        self.ai_features_disabled
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SmtpSettings {
    enable_smtp: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    configured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ssl_tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_ssl_certs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_start_tls: Option<bool>,
}

impl SmtpSettings {
    pub fn enable_smtp(&self) -> bool {
        self.enable_smtp
    }
    pub fn configured(&self) -> Option<bool> {
        self.configured
    }
    pub fn sender_address(&self) -> Option<&str> {
        self.sender_address.as_deref()
    }
    pub fn server(&self) -> Option<&str> {
        self.server.as_deref()
    }
    pub fn port(&self) -> Option<u16> {
        self.port
    }
    pub fn authentication_type(&self) -> Option<&str> {
        self.authentication_type.as_deref()
    }
    pub fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    pub fn enable_ssl_tls(&self) -> Option<bool> {
        self.enable_ssl_tls
    }
    pub fn authentication_method(&self) -> Option<&str> {
        self.authentication_method.as_deref()
    }
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }
    pub fn verify_ssl_certs(&self) -> Option<bool> {
        self.verify_ssl_certs
    }
    pub fn enable_start_tls(&self) -> Option<bool> {
        self.enable_start_tls
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_name: Option<String>,
    enable_sso: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sso_idp_login: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_jit_provisioning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_jit_role_sync: Option<bool>,
}

impl SsoSettings {
    pub fn entity_id(&self) -> Option<&str> {
        self.entity_id.as_deref()
    }
    pub fn issuer_uri(&self) -> Option<&str> {
        self.issuer_uri.as_deref()
    }
    pub fn idp_image_url(&self) -> Option<&str> {
        self.idp_image_url.as_deref()
    }
    pub fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
    pub fn metadata_url(&self) -> Option<&str> {
        self.metadata_url.as_deref()
    }
    pub fn idp_name(&self) -> Option<&str> {
        self.idp_name.as_deref()
    }
    pub fn enable_sso(&self) -> bool {
        self.enable_sso
    }
    pub fn enable_sso_idp_login(&self) -> Option<bool> {
        self.enable_sso_idp_login
    }
    pub fn enable_jit_provisioning(&self) -> Option<bool> {
        self.enable_jit_provisioning
    }
    pub fn enable_jit_role_sync(&self) -> Option<bool> {
        self.enable_jit_role_sync
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostExpirySettings {
    host_expiry_enabled: bool,
    host_expiry_window: u32,
}

impl HostExpirySettings {
    pub fn host_expiry_enabled(&self) -> bool {
        self.host_expiry_enabled
    }
    pub fn host_expiry_window(&self) -> u32 {
        self.host_expiry_window
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_host_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_software_inventory: Option<bool>,
}

impl Features {
    pub fn enable_host_users(&self) -> Option<bool> {
        self.enable_host_users
    }
    pub fn enable_software_inventory(&self) -> Option<bool> {
        self.enable_software_inventory
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_status_webhook: Option<WebhookConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failing_policies_webhook: Option<WebhookConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerabilities_webhook: Option<WebhookConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
}

impl WebhookSettings {
    pub fn host_status_webhook(&self) -> Option<&WebhookConfig> {
        self.host_status_webhook.as_ref()
    }
    pub fn failing_policies_webhook(&self) -> Option<&WebhookConfig> {
        self.failing_policies_webhook.as_ref()
    }
    pub fn vulnerabilities_webhook(&self) -> Option<&WebhookConfig> {
        self.vulnerabilities_webhook.as_ref()
    }
    pub fn interval(&self) -> Option<&str> {
        self.interval.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_host_status_webhook: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_count: Option<u32>,
}

impl WebhookConfig {
    pub fn enable_host_status_webhook(&self) -> Option<bool> {
        self.enable_host_status_webhook
    }
    pub fn destination_url(&self) -> Option<&str> {
        self.destination_url.as_deref()
    }
    pub fn host_percentage(&self) -> Option<f64> {
        self.host_percentage
    }
    pub fn days_count(&self) -> Option<u32> {
        self.days_count
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Integrations {
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Vec<JiraIntegration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zendesk: Option<Vec<ZendeskIntegration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_calendar: Option<Vec<GoogleCalendarIntegration>>,
}

impl Integrations {
    pub fn jira(&self) -> Option<&[JiraIntegration]> {
        self.jira.as_deref()
    }
    pub fn zendesk(&self) -> Option<&[ZendeskIntegration]> {
        self.zendesk.as_deref()
    }
    pub fn google_calendar(&self) -> Option<&[GoogleCalendarIntegration]> {
        self.google_calendar.as_deref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JiraIntegration {
    url: String,
    username: String,
    api_token: String,
    project_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_software_vulnerabilities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_failing_policies: Option<bool>,
}

impl JiraIntegration {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn api_token(&self) -> &str {
        &self.api_token
    }
    pub fn project_key(&self) -> &str {
        &self.project_key
    }
    pub fn enable_software_vulnerabilities(&self) -> Option<bool> {
        self.enable_software_vulnerabilities
    }
    pub fn enable_failing_policies(&self) -> Option<bool> {
        self.enable_failing_policies
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZendeskIntegration {
    url: String,
    email: String,
    api_token: String,
    group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_software_vulnerabilities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_failing_policies: Option<bool>,
}

impl ZendeskIntegration {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn api_token(&self) -> &str {
        &self.api_token
    }
    pub fn group_id(&self) -> i64 {
        self.group_id
    }
    pub fn enable_software_vulnerabilities(&self) -> Option<bool> {
        self.enable_software_vulnerabilities
    }
    pub fn enable_failing_policies(&self) -> Option<bool> {
        self.enable_failing_policies
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GoogleCalendarIntegration {
    domain: String,
    api_key_json: HashMap<String, serde_json::Value>,
}

impl GoogleCalendarIntegration {
    pub fn domain(&self) -> &str {
        &self.domain
    }
    pub fn api_key_json(&self) -> &HashMap<String, serde_json::Value> {
        &self.api_key_json
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdmSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_bm_default_team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_bm_terms_expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_bm_enabled_and_configured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_and_configured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_user_authentication: Option<EndUserAuthentication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macos_updates: Option<MacOsUpdates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_updates: Option<WindowsUpdates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macos_settings: Option<MacOsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_settings: Option<WindowsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macos_setup: Option<MacOsSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_enabled_and_configured: Option<bool>,
}

impl MdmSettings {
    pub fn apple_bm_default_team(&self) -> Option<&str> {
        self.apple_bm_default_team.as_deref()
    }
    pub fn apple_bm_terms_expired(&self) -> Option<bool> {
        self.apple_bm_terms_expired
    }
    pub fn apple_bm_enabled_and_configured(&self) -> Option<bool> {
        self.apple_bm_enabled_and_configured
    }
    pub fn enabled_and_configured(&self) -> Option<bool> {
        self.enabled_and_configured
    }
    pub fn end_user_authentication(&self) -> Option<&EndUserAuthentication> {
        self.end_user_authentication.as_ref()
    }
    pub fn macos_updates(&self) -> Option<&MacOsUpdates> {
        self.macos_updates.as_ref()
    }
    pub fn windows_updates(&self) -> Option<&WindowsUpdates> {
        self.windows_updates.as_ref()
    }
    pub fn macos_settings(&self) -> Option<&MacOsSettings> {
        self.macos_settings.as_ref()
    }
    pub fn windows_settings(&self) -> Option<&WindowsSettings> {
        self.windows_settings.as_ref()
    }
    pub fn macos_setup(&self) -> Option<&MacOsSetup> {
        self.macos_setup.as_ref()
    }
    pub fn windows_enabled_and_configured(&self) -> Option<bool> {
        self.windows_enabled_and_configured
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndUserAuthentication {
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_name: Option<String>,
}

impl EndUserAuthentication {
    pub fn entity_id(&self) -> Option<&str> {
        self.entity_id.as_deref()
    }
    pub fn issuer_uri(&self) -> Option<&str> {
        self.issuer_uri.as_deref()
    }
    pub fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
    pub fn metadata_url(&self) -> Option<&str> {
        self.metadata_url.as_deref()
    }
    pub fn idp_name(&self) -> Option<&str> {
        self.idp_name.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOsUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<String>,
}

impl MacOsUpdates {
    pub fn minimum_version(&self) -> Option<&str> {
        self.minimum_version.as_deref()
    }
    pub fn deadline(&self) -> Option<&str> {
        self.deadline.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grace_period_days: Option<u32>,
}

impl WindowsUpdates {
    pub fn deadline_days(&self) -> Option<u32> {
        self.deadline_days
    }
    pub fn grace_period_days(&self) -> Option<u32> {
        self.grace_period_days
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_settings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_disk_encryption: Option<bool>,
}

impl MacOsSettings {
    pub fn custom_settings(&self) -> Option<&[String]> {
        self.custom_settings.as_deref()
    }
    pub fn enable_disk_encryption(&self) -> Option<bool> {
        self.enable_disk_encryption
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_settings: Option<Vec<String>>,
}

impl WindowsSettings {
    pub fn custom_settings(&self) -> Option<&[String]> {
        self.custom_settings.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOsSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_end_user_authentication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macos_setup_assistant: Option<String>,
}

impl MacOsSetup {
    pub fn bootstrap_package(&self) -> Option<&str> {
        self.bootstrap_package.as_deref()
    }
    pub fn enable_end_user_authentication(&self) -> Option<bool> {
        self.enable_end_user_authentication
    }
    pub fn macos_setup_assistant(&self) -> Option<&str> {
        self.macos_setup_assistant.as_deref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetConfigResponse {
    org_info: OrgInfo,
    server_settings: ServerSettings,
    smtp_settings: SmtpSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    sso_settings: Option<SsoSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_expiry_settings: Option<HostExpirySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<Features>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_settings: Option<WebhookSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integrations: Option<Integrations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdm: Option<MdmSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Logging>,
}

impl GetConfigResponse {
    pub fn org_info(&self) -> &OrgInfo {
        &self.org_info
    }
    pub fn server_settings(&self) -> &ServerSettings {
        &self.server_settings
    }
    pub fn smtp_settings(&self) -> &SmtpSettings {
        &self.smtp_settings
    }
    /// SSO settings, when Fleet includes them for the caller and edition.
    pub fn sso_settings(&self) -> Option<&SsoSettings> {
        self.sso_settings.as_ref()
    }
    pub fn host_expiry_settings(&self) -> Option<&HostExpirySettings> {
        self.host_expiry_settings.as_ref()
    }
    pub fn features(&self) -> Option<&Features> {
        self.features.as_ref()
    }
    pub fn agent_options(&self) -> Option<&serde_json::Value> {
        self.agent_options.as_ref()
    }
    pub fn webhook_settings(&self) -> Option<&WebhookSettings> {
        self.webhook_settings.as_ref()
    }
    pub fn integrations(&self) -> Option<&Integrations> {
        self.integrations.as_ref()
    }
    pub fn mdm(&self) -> Option<&MdmSettings> {
        self.mdm.as_ref()
    }
    pub fn license(&self) -> Option<&License> {
        self.license.as_ref()
    }
    pub fn logging(&self) -> Option<&Logging> {
        self.logging.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    tier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

impl License {
    pub fn tier(&self) -> &str {
        &self.tier
    }
    pub fn organization(&self) -> Option<&str> {
        self.organization.as_deref()
    }
    pub fn device_count(&self) -> Option<u32> {
        self.device_count
    }
    pub fn expiration(&self) -> Option<&str> {
        self.expiration.as_deref()
    }
    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Logging {
    debug: bool,
    json: bool,
    result: LoggingDestination,
    status: LoggingDestination,
}

impl Logging {
    pub fn debug(&self) -> bool {
        self.debug
    }
    pub fn json(&self) -> bool {
        self.json
    }
    pub fn result(&self) -> &LoggingDestination {
        &self.result
    }
    pub fn status(&self) -> &LoggingDestination {
        &self.status
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoggingDestination {
    plugin: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    config: HashMap<String, serde_json::Value>,
}

impl LoggingDestination {
    pub fn plugin(&self) -> &str {
        &self.plugin
    }
    pub fn config(&self) -> &HashMap<String, serde_json::Value> {
        &self.config
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnrollSecret {
    secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
}

impl EnrollSecret {
    pub fn secret(&self) -> &str {
        &self.secret
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn team_id(&self) -> Option<u32> {
        self.team_id
    }
    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnrollSecretsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Vec<EnrollSecret>>,
}

impl EnrollSecretsResponse {
    pub fn secrets(&self) -> Option<&[EnrollSecret]> {
        self.secrets.as_deref()
    }
}

// Request structs keep public fields for easy construction
#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyEnrollSecretsRequest {
    pub secrets: Vec<EnrollSecretRequest>,
}

impl ModifyEnrollSecretsRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.secrets.is_empty() {
            return Err(crate::FleetError::Validation(
                "enroll secrets must not be empty".into(),
            ));
        }
        if self
            .secrets
            .iter()
            .any(|secret| secret.secret.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "enroll secret values must not be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnrollSecretRequest {
    pub secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_allows_role_or_edition_specific_sso_settings_to_be_absent() {
        let response: GetConfigResponse = serde_json::from_value(serde_json::json!({
            "org_info": {
                "org_name": "Fleet",
                "org_logo_url": ""
            },
            "server_settings": {
                "server_url": "https://fleet.example.com",
                "live_query_disabled": false
            },
            "smtp_settings": {
                "enable_smtp": false
            }
        }))
        .expect("minimal documented config should deserialize");

        assert!(response.sso_settings().is_none());
    }
}
