use serde::{Deserialize, Serialize};

// MDM Apple Certificate (existing)
#[derive(Clone, Serialize, Deserialize)]
pub struct Certificate {
    certificate_chain: String,
    private_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
}

impl Certificate {
    pub fn certificate_chain(&self) -> &str {
        &self.certificate_chain
    }
    pub fn private_key(&self) -> &str {
        &self.private_key
    }
    pub fn certificate(&self) -> Option<&str> {
        self.certificate.as_deref()
    }
}

// Certificate Authority (CA) models

#[derive(Clone, Serialize, Deserialize)]
pub struct CertificateAuthority {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub ca_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_common_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_user_principal_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_seat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_url: Option<String>,
}

impl CertificateAuthority {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ca_type(&self) -> &str {
        &self.ca_type
    }
}

// Create CA request models
#[derive(Clone, Serialize)]
pub struct CreateCertificateAuthorityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digicert: Option<DigiCertConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndes_scep_proxy: Option<NdesScepProxyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_scep_proxy: Option<CustomScepProxyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_est_proxy: Option<CustomEstProxyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hydrant: Option<HydrantConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smallstep: Option<SmallstepConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DigiCertConfig {
    pub name: String,
    pub url: String,
    pub api_token: String,
    pub profile_id: String,
    pub certificate_common_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_user_principal_names: Option<Vec<String>>,
    pub certificate_seat_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NdesScepProxyConfig {
    pub url: String,
    pub admin_url: String,
    pub password: String,
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomScepProxyConfig {
    pub name: String,
    pub url: String,
    pub challenge: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomEstProxyConfig {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HydrantConfig {
    pub name: String,
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SmallstepConfig {
    pub url: String,
    pub challenge_url: String,
    pub username: String,
    pub password: String,
}

// Update CA request
#[derive(Clone, Serialize)]
pub struct UpdateCertificateAuthorityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digicert: Option<DigiCertConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndes_scep_proxy: Option<NdesScepProxyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_scep_proxy: Option<CustomScepProxyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_est_proxy: Option<CustomEstProxyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hydrant: Option<HydrantConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smallstep: Option<SmallstepConfig>,
}

impl CreateCertificateAuthorityRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        validate_ca_choice(
            self.digicert.as_ref(),
            self.ndes_scep_proxy.as_ref(),
            self.custom_scep_proxy.as_ref(),
            self.custom_est_proxy.as_ref(),
            self.hydrant.as_ref(),
            self.smallstep.as_ref(),
        )
    }
}

impl UpdateCertificateAuthorityRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        validate_ca_choice(
            self.digicert.as_ref(),
            self.ndes_scep_proxy.as_ref(),
            self.custom_scep_proxy.as_ref(),
            self.custom_est_proxy.as_ref(),
            self.hydrant.as_ref(),
            self.smallstep.as_ref(),
        )
    }
}

fn validate_ca_choice(
    digicert: Option<&DigiCertConfig>,
    ndes: Option<&NdesScepProxyConfig>,
    custom_scep: Option<&CustomScepProxyConfig>,
    custom_est: Option<&CustomEstProxyConfig>,
    hydrant: Option<&HydrantConfig>,
    smallstep: Option<&SmallstepConfig>,
) -> crate::Result<()> {
    let selected = [
        digicert.is_some(),
        ndes.is_some(),
        custom_scep.is_some(),
        custom_est.is_some(),
        hydrant.is_some(),
        smallstep.is_some(),
    ]
    .into_iter()
    .filter(|selected| *selected)
    .count();
    if selected != 1 {
        return Err(crate::FleetError::Validation(
            "exactly one certificate authority type must be configured".into(),
        ));
    }

    let valid = digicert.is_none_or(|config| {
        all_nonempty(&[
            &config.name,
            &config.url,
            &config.api_token,
            &config.profile_id,
            &config.certificate_common_name,
            &config.certificate_seat_id,
        ]) && config
            .certificate_user_principal_names
            .as_ref()
            .is_none_or(|names| names.iter().all(|name| !name.trim().is_empty()))
    }) && ndes.is_none_or(|config| {
        all_nonempty(&[
            &config.url,
            &config.admin_url,
            &config.username,
            &config.password,
        ])
    }) && custom_scep
        .is_none_or(|config| all_nonempty(&[&config.name, &config.url, &config.challenge]))
        && custom_est.is_none_or(|config| {
            all_nonempty(&[
                &config.name,
                &config.url,
                &config.username,
                &config.password,
            ])
        })
        && hydrant.is_none_or(|config| {
            all_nonempty(&[
                &config.name,
                &config.url,
                &config.client_id,
                &config.client_secret,
            ])
        })
        && smallstep.is_none_or(|config| {
            all_nonempty(&[
                &config.url,
                &config.challenge_url,
                &config.username,
                &config.password,
            ])
        });
    if !valid {
        return Err(crate::FleetError::Validation(
            "certificate authority fields must not be empty".into(),
        ));
    }
    Ok(())
}

fn all_nonempty(values: &[&str]) -> bool {
    values.iter().all(|value| !value.trim().is_empty())
}

// Certificate Authority responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateAuthorityResponse {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub ca_type: String,
}

impl CertificateAuthorityResponse {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ca_type(&self) -> &str {
        &self.ca_type
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCertificateAuthoritiesResponse {
    pub certificate_authorities: Vec<CertificateAuthorityResponse>,
}

impl ListCertificateAuthoritiesResponse {
    pub fn certificate_authorities(&self) -> &[CertificateAuthorityResponse] {
        &self.certificate_authorities
    }
}

// Certificate Template models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateTemplate {
    pub id: u64,
    pub name: String,
    pub certificate_authority_id: u64,
    pub certificate_authority_name: String,
    pub subject_name: String,
    pub created_at: String,
}

impl CertificateTemplate {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn certificate_authority_id(&self) -> u64 {
        self.certificate_authority_id
    }
    pub fn subject_name(&self) -> &str {
        &self.subject_name
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCertificateTemplateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fleet_id")]
    pub team_id: Option<u64>,
    pub certificate_authority_id: u64,
    pub subject_name: String,
}

impl CreateCertificateTemplateRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty()
            || self.certificate_authority_id == 0
            || self.subject_name.trim().is_empty()
        {
            return Err(crate::FleetError::Validation(
                "certificate name, authority ID, and subject name are required".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateTemplateResponse {
    pub id: u64,
    pub name: String,
    pub certificate_authority_id: u64,
    pub subject_name: String,
}

impl CertificateTemplateResponse {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCertificateTemplatesResponse {
    #[serde(default)]
    pub certificates: Vec<CertificateTemplate>,
    #[serde(default)]
    pub meta: Option<CertificatesMeta>,
}

impl ListCertificateTemplatesResponse {
    pub fn certificates(&self) -> &[CertificateTemplate] {
        &self.certificates
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CertificatesMeta {
    #[serde(default)]
    pub has_next_results: bool,
    #[serde(default)]
    pub has_previous_results: bool,
}

// Request certificate
#[derive(Clone, Serialize)]
pub struct RequestCertificateRequest {
    pub csr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_oauth_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_client_id: Option<String>,
}

impl RequestCertificateRequest {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if self.csr.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "certificate signing request must not be empty".into(),
            ));
        }
        if self
            .idp_oauth_url
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
            || self
                .idp_token
                .as_ref()
                .is_some_and(|value| value.trim().is_empty())
            || self
                .idp_client_id
                .as_ref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(crate::FleetError::Validation(
                "IdP certificate request values must not be empty when provided".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCertificateResponse {
    pub certificate: String,
}

impl RequestCertificateResponse {
    pub fn certificate(&self) -> &str {
        &self.certificate
    }
}
