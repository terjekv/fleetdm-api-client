//! Centralized API path definitions for the FleetDM API
//!
//! This module provides functions for generating API paths,
//! ensuring consistency and reducing the risk of typos.

// Authentication paths
pub const LOGIN: &str = "/api/v1/fleet/login";
pub const LOGOUT: &str = "/api/v1/fleet/logout";
pub const FORGOT_PASSWORD: &str = "/api/v1/fleet/forgot_password";
pub const CHANGE_PASSWORD: &str = "/api/v1/fleet/change_password";
pub const RESET_PASSWORD: &str = "/api/v1/fleet/reset_password";
pub const PERFORM_REQUIRED_PASSWORD_RESET: &str = "/api/v1/fleet/perform_required_password_reset";
pub const ME: &str = "/api/v1/fleet/me";
pub const SSO: &str = "/api/v1/fleet/sso";
pub const SSO_CALLBACK: &str = "/api/v1/fleet/sso/callback";

// Version path
pub const VERSION: &str = "/api/v1/fleet/version";

// Host paths
pub const HOSTS: &str = "/api/v1/fleet/hosts";
pub const HOSTS_COUNT: &str = "/api/v1/fleet/hosts/count";
pub const HOSTS_DELETE: &str = "/api/v1/fleet/hosts/delete";
pub const HOSTS_TRANSFER: &str = "/api/v1/fleet/hosts/transfer";
pub const HOST_SUMMARY: &str = "/api/v1/fleet/host_summary";

pub fn host(id: u64) -> String {
    format!("{}/{}", HOSTS, id)
}

pub fn host_identifier(identifier: &str) -> String {
    format!(
        "{}/identifier/{}",
        HOSTS,
        crate::http::encode_path_segment(identifier)
    )
}

pub fn host_refetch(id: u64) -> String {
    format!("{}/{}/refetch", HOSTS, id)
}

pub fn host_lock(id: u64) -> String {
    format!("{}/{}/lock", HOSTS, id)
}

pub fn host_unlock(id: u64) -> String {
    format!("{}/{}/unlock", HOSTS, id)
}

pub fn host_wipe(id: u64) -> String {
    format!("{}/{}/wipe", HOSTS, id)
}

// Invitation paths
pub const INVITES: &str = "/api/v1/fleet/invites";

pub fn invite(id: u64) -> String {
    format!("{}/{}", INVITES, id)
}

// Deprecated query paths, retained for compatibility with older Fleet servers.
pub const QUERIES: &str = "/api/v1/fleet/queries";
pub const QUERIES_DELETE: &str = "/api/v1/fleet/queries/delete";

pub fn query(id: u64) -> String {
    format!("{}/{}", QUERIES, id)
}

pub fn query_delete(id: u64) -> String {
    format!("{}/id/{}", QUERIES, id)
}

pub fn query_run(id: u64) -> String {
    format!("{}/{}/run", QUERIES, id)
}

pub fn query_report(id: u64) -> String {
    format!("{}/{}/report", QUERIES, id)
}

// Policy paths
pub const POLICIES: &str = "/api/v1/fleet/global/policies";

pub fn policy(id: u64) -> String {
    format!("{}/{}", POLICIES, id)
}

// Software paths
pub const SOFTWARE: &str = "/api/v1/fleet/software/titles";
pub const SOFTWARE_VERSIONS: &str = "/api/v1/fleet/software/versions";
pub const OS_VERSIONS: &str = "/api/v1/fleet/os_versions";
pub const SOFTWARE_PACKAGE: &str = "/api/v1/fleet/software/package";
pub const APP_STORE_APPS: &str = "/api/v1/fleet/software/app_store_apps";
pub const FLEET_MAINTAINED_APPS: &str = "/api/v1/fleet/software/fleet_maintained_apps";

pub fn software_item(id: u64) -> String {
    format!("{}/{}", SOFTWARE, id)
}

pub fn software_version(id: u64) -> String {
    format!("{}/{}", SOFTWARE_VERSIONS, id)
}

pub fn os_version(id: u64) -> String {
    format!("{}/{}", OS_VERSIONS, id)
}

pub fn software_package(id: u64) -> String {
    format!("{}/{}/package", SOFTWARE, id)
}

pub fn software_app_store_app(id: u64) -> String {
    format!("{}/{}/app_store_app", SOFTWARE, id)
}

pub fn software_icon(id: u64) -> String {
    format!("{}/{}/icon", SOFTWARE, id)
}

pub fn software_available_for_install(id: u64) -> String {
    format!("{}/{}/available_for_install", SOFTWARE, id)
}

pub fn fleet_maintained_app(id: u64) -> String {
    format!("{}/{}", FLEET_MAINTAINED_APPS, id)
}

pub fn host_software_action(host_id: u64, software_title_id: u64, action: &str) -> String {
    format!("{HOSTS}/{host_id}/software/{software_title_id}/{action}")
}

pub fn software_install_result(install_uuid: &str) -> String {
    format!(
        "/api/v1/fleet/software/install/{}/results",
        crate::http::encode_path_segment(install_uuid)
    )
}

// Fleet paths
pub const FLEETS: &str = "/api/v1/fleet/fleets";

pub fn fleet(id: u64) -> String {
    format!("{}/{}", FLEETS, id)
}

pub fn fleet_policies(fleet_id: u64) -> String {
    format!("{}/{}/policies", FLEETS, fleet_id)
}

pub fn fleet_policy(fleet_id: u64, policy_id: u64) -> String {
    format!("{}/{}/policies/{}", FLEETS, fleet_id, policy_id)
}

pub fn fleet_policies_delete(fleet_id: u64) -> String {
    format!("{}/{}/policies/delete", FLEETS, fleet_id)
}

pub fn fleet_secrets(fleet_id: u64) -> String {
    format!("{}/{}/secrets", FLEETS, fleet_id)
}

pub fn fleet_users(fleet_id: u64) -> String {
    format!("{}/{}/users", FLEETS, fleet_id)
}

pub fn fleet_agent_options(fleet_id: u64) -> String {
    format!("{}/{}/agent_options", FLEETS, fleet_id)
}

// Report paths
pub const REPORTS: &str = "/api/v1/fleet/reports";
pub const REPORTS_DELETE: &str = "/api/v1/fleet/reports/delete";

pub fn report(id: u64) -> String {
    format!("{}/{}", REPORTS, id)
}

pub fn report_name(name: &str) -> String {
    format!("{}/{}", REPORTS, crate::http::encode_path_segment(name))
}

pub fn report_delete(id: u64) -> String {
    format!("{}/id/{}", REPORTS, id)
}

pub fn report_run(id: u64) -> String {
    format!("{}/{}/run", REPORTS, id)
}

pub fn report_data(id: u64) -> String {
    format!("{}/{}/report", REPORTS, id)
}

pub fn host_reports(host_id: u64) -> String {
    format!("{}/{}/reports", HOSTS, host_id)
}

pub fn host_report(host_id: u64, report_id: u64) -> String {
    format!("{}/{}/reports/{}", HOSTS, host_id, report_id)
}

// Team paths
pub const TEAMS: &str = "/api/v1/fleet/teams";

pub fn team(id: u64) -> String {
    format!("{}/{}", TEAMS, id)
}

pub fn team_policies(team_id: u64) -> String {
    format!("{}/{}/policies", TEAMS, team_id)
}

pub fn team_policy(team_id: u64, policy_id: u64) -> String {
    format!("{}/{}/policies/{}", TEAMS, team_id, policy_id)
}

pub fn team_policies_delete(team_id: u64) -> String {
    format!("{}/{}/policies/delete", TEAMS, team_id)
}

// User paths
pub const USERS: &str = "/api/v1/fleet/users";
pub const USERS_ADMIN: &str = "/api/v1/fleet/users/admin";

pub fn user(id: u64) -> String {
    format!("{}/{}", USERS, id)
}

// Label paths
pub const LABELS: &str = "/api/v1/fleet/labels";

pub fn label(id: u64) -> String {
    format!("{}/{}", LABELS, id)
}

pub fn label_delete(id: u64) -> String {
    format!("{}/id/{}", LABELS, id)
}

// Script paths
pub const SCRIPTS: &str = "/api/v1/fleet/scripts";
pub const SCRIPTS_RUN: &str = "/api/v1/fleet/scripts/run";

pub fn script(id: u64) -> String {
    format!("{}/{}", SCRIPTS, id)
}

pub fn script_results(execution_id: &str) -> String {
    format!(
        "{}/results/{}",
        SCRIPTS,
        crate::http::encode_path_segment(execution_id)
    )
}

// Activity paths
pub const ACTIVITIES: &str = "/api/v1/fleet/activities";

// Configuration paths
pub const CONFIG: &str = "/api/v1/fleet/config";
pub const ENROLL_SECRETS_SPEC: &str = "/api/v1/fleet/spec/enroll_secret";

// Session paths
pub const SESSIONS: &str = "/api/v1/fleet/sessions";

pub fn session(id: u64) -> String {
    format!("{}/{}", SESSIONS, id)
}

// Vulnerability paths
pub const VULNERABILITIES: &str = "/api/v1/fleet/vulnerabilities";

pub fn vulnerability(cve: &str) -> String {
    format!(
        "{}/{}",
        VULNERABILITIES,
        crate::http::encode_path_segment(cve)
    )
}

// Target paths
pub const TARGETS: &str = "/api/v1/fleet/targets";

// Translator paths
pub const TRANSLATOR: &str = "/api/v1/fleet/translate";

// File carving paths
pub const CARVES: &str = "/api/v1/fleet/carves";

pub fn carve(id: u64) -> String {
    format!("{}/{}", CARVES, id)
}

pub fn carve_block(carve_id: u64, block_id: u64) -> String {
    format!("{}/{}/block/{}", CARVES, carve_id, block_id)
}

// Certificate paths
pub const CERTIFICATE: &str = "/api/v1/fleet/config/certificate";
pub const CERTIFICATE_AUTHORITIES: &str = "/api/v1/fleet/certificate_authorities";
pub const CERTIFICATES: &str = "/api/v1/fleet/certificates";

pub fn certificate_authority(id: u64) -> String {
    format!("{}/{}", CERTIFICATE_AUTHORITIES, id)
}

pub fn certificate(id: u64) -> String {
    format!("{}/{}", CERTIFICATES, id)
}

pub fn certificate_authority_request_certificate(id: u64) -> String {
    format!("{}/{}/request_certificate", CERTIFICATE_AUTHORITIES, id)
}

// MDM command paths
pub const COMMANDS: &str = "/api/v1/fleet/commands";
pub const COMMANDS_RUN: &str = "/api/v1/fleet/commands/run";

pub fn command_results() -> String {
    format!("{}/results", COMMANDS)
}

// OS Settings paths
pub const CONFIGURATION_PROFILES: &str = "/api/v1/fleet/configuration_profiles";
pub const CONFIGURATION_PROFILES_BATCH: &str = "/api/v1/fleet/configuration_profiles/batch";
pub const CONFIGURATION_PROFILES_RESEND_BATCH: &str =
    "/api/v1/fleet/configuration_profiles/resend/batch";
pub const DISK_ENCRYPTION: &str = "/api/v1/fleet/disk_encryption";
pub const CONFIGURATION_PROFILES_SUMMARY: &str = "/api/v1/fleet/configuration_profiles/summary";

pub fn configuration_profile(profile_uuid: &str) -> String {
    format!(
        "{}/{}",
        CONFIGURATION_PROFILES,
        crate::http::encode_path_segment(profile_uuid)
    )
}

pub fn configuration_profile_status(profile_uuid: &str) -> String {
    format!(
        "{}/{}/status",
        CONFIGURATION_PROFILES,
        crate::http::encode_path_segment(profile_uuid)
    )
}

// Conditional Access paths
pub const CONDITIONAL_ACCESS_IDP_SIGNING_CERT: &str =
    "/api/v1/fleet/conditional_access/idp/signing_cert";
pub const CONDITIONAL_ACCESS_IDP_APPLE_PROFILE: &str =
    "/api/v1/fleet/conditional_access/idp/apple/profile";
pub const CONDITIONAL_ACCESS_MICROSOFT: &str = "/api/v1/conditional-access/microsoft";

// Setup Experience paths
pub const SETUP_EXPERIENCE: &str = "/api/v1/fleet/setup_experience";
pub const SETUP_EXPERIENCE_EULA: &str = "/api/v1/fleet/setup_experience/eula";
pub const SETUP_EXPERIENCE_EULA_METADATA: &str = "/api/v1/fleet/setup_experience/eula/metadata";
pub const SETUP_EXPERIENCE_SCRIPT: &str = "/api/v1/fleet/setup_experience/script";
pub const SETUP_EXPERIENCE_SOFTWARE: &str = "/api/v1/fleet/setup_experience/software";
pub const ENROLLMENT_PROFILES_AUTOMATIC: &str = "/api/v1/fleet/enrollment_profiles/automatic";
pub const ENROLLMENT_PROFILES_MANUAL: &str = "/api/v1/fleet/enrollment_profiles/manual";
pub const ENROLLMENT_PROFILES_OTA: &str = "/api/v1/fleet/enrollment_profiles/ota";

pub fn setup_experience_eula(token: &str) -> String {
    format!(
        "{}/{}",
        SETUP_EXPERIENCE_EULA,
        crate::http::encode_path_segment(token)
    )
}

// Integration paths
pub const APNS: &str = "/api/v1/fleet/apns";
pub const ABM_TOKENS: &str = "/api/v1/fleet/abm_tokens";
pub const VPP_TOKENS: &str = "/api/v1/fleet/vpp_tokens";
pub const SCIM_DETAILS: &str = "/api/v1/fleet/scim/details";
pub const ANDROID_ENTERPRISE: &str = "/api/v1/fleet/android_enterprise";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_paths() {
        assert_eq!(HOSTS, "/api/v1/fleet/hosts");
        assert_eq!(host(123), "/api/v1/fleet/hosts/123");
        assert_eq!(host_refetch(456), "/api/v1/fleet/hosts/456/refetch");
    }

    #[test]
    fn test_report_paths() {
        assert_eq!(REPORTS, "/api/v1/fleet/reports");
        assert_eq!(report(789), "/api/v1/fleet/reports/789");
        assert_eq!(report_delete(789), "/api/v1/fleet/reports/id/789");
        assert_eq!(report_run(123), "/api/v1/fleet/reports/123/run");
        assert_eq!(report_data(123), "/api/v1/fleet/reports/123/report");
    }

    #[test]
    fn test_label_paths() {
        assert_eq!(LABELS, "/api/v1/fleet/labels");
        assert_eq!(label(42), "/api/v1/fleet/labels/42");
        assert_eq!(label_delete(42), "/api/v1/fleet/labels/id/42");
    }

    #[test]
    fn test_user_paths() {
        assert_eq!(USERS, "/api/v1/fleet/users");
        assert_eq!(USERS_ADMIN, "/api/v1/fleet/users/admin");
        assert_eq!(user(42), "/api/v1/fleet/users/42");
    }
}
