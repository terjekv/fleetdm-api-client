//! Integration models for APNs, ABM, VPP, SCIM, and Android Enterprise

use serde::{Deserialize, Serialize};

/// Apple Push Notification service (APNs) certificate information
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ApnsInfo {
    /// Certificate common name
    pub common_name: String,
    /// Certificate serial number
    pub serial_number: String,
    /// Certificate issuer
    pub issuer: String,
    /// Certificate renewal date
    pub renew_date: String,
}

impl ApnsInfo {
    pub fn common_name(&self) -> &str {
        &self.common_name
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    pub fn renew_date(&self) -> &str {
        &self.renew_date
    }
}

/// Team assignment for ABM and VPP tokens
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TeamAssignment {
    /// Team name
    pub name: String,
    /// Team ID
    pub id: u64,
}

impl TeamAssignment {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

/// Apple Business Manager (ABM) token information
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AbmToken {
    /// Token ID
    pub id: u64,
    /// Apple ID associated with the token
    pub apple_id: String,
    /// Organization name
    pub org_name: String,
    /// MDM server URL
    pub mdm_server_url: String,
    /// Certificate renewal date
    pub renew_date: String,
    /// Whether terms have expired
    pub terms_expired: bool,
    /// macOS team assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macos_team: Option<TeamAssignment>,
    /// iOS team assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_team: Option<TeamAssignment>,
    /// iPadOS team assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipados_team: Option<TeamAssignment>,
}

impl AbmToken {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn apple_id(&self) -> &str {
        &self.apple_id
    }

    pub fn org_name(&self) -> &str {
        &self.org_name
    }

    pub fn mdm_server_url(&self) -> &str {
        &self.mdm_server_url
    }

    pub fn renew_date(&self) -> &str {
        &self.renew_date
    }

    pub fn terms_expired(&self) -> bool {
        self.terms_expired
    }

    pub fn macos_team(&self) -> Option<&TeamAssignment> {
        self.macos_team.as_ref()
    }

    pub fn ios_team(&self) -> Option<&TeamAssignment> {
        self.ios_team.as_ref()
    }

    pub fn ipados_team(&self) -> Option<&TeamAssignment> {
        self.ipados_team.as_ref()
    }
}

/// Response for listing ABM tokens
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AbmTokenListResponse {
    pub abm_tokens: Vec<AbmToken>,
}

impl AbmTokenListResponse {
    pub fn abm_tokens(&self) -> &[AbmToken] {
        &self.abm_tokens
    }
}

/// Volume Purchasing Program (VPP) token information
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct VppToken {
    /// Token ID
    pub id: u64,
    /// Organization name
    pub org_name: String,
    /// Location/URL
    pub location: String,
    /// Token renewal date
    pub renew_date: String,
    /// Team assignments
    #[serde(default)]
    pub teams: Vec<TeamAssignment>,
}

impl VppToken {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn org_name(&self) -> &str {
        &self.org_name
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn renew_date(&self) -> &str {
        &self.renew_date
    }

    pub fn teams(&self) -> &[TeamAssignment] {
        &self.teams
    }
}

/// Response for listing VPP tokens
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VppTokenListResponse {
    pub vpp_tokens: Vec<VppToken>,
}

impl VppTokenListResponse {
    pub fn vpp_tokens(&self) -> &[VppToken] {
        &self.vpp_tokens
    }
}

/// SCIM request details
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ScimRequestDetails {
    /// When the request was made
    pub requested_at: String,
    /// Request status (success, error, etc.)
    pub status: String,
    /// Additional details about the request
    pub details: String,
}

impl ScimRequestDetails {
    pub fn requested_at(&self) -> &str {
        &self.requested_at
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn details(&self) -> &str {
        &self.details
    }
}

/// SCIM details response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScimDetailsResponse {
    /// Last SCIM request from IdP
    pub last_request: ScimRequestDetails,
}

impl ScimDetailsResponse {
    pub fn last_request(&self) -> &ScimRequestDetails {
        &self.last_request
    }
}

/// Android Enterprise information
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AndroidEnterpriseInfo {
    /// Android Enterprise ID
    pub android_enterprise_id: String,
}

impl AndroidEnterpriseInfo {
    pub fn android_enterprise_id(&self) -> &str {
        &self.android_enterprise_id
    }
}
