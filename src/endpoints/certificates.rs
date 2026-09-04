use crate::client::FleetClient;
use crate::error::Result;
use crate::models::certificate::{
    Certificate, CertificateAuthority, CertificateTemplate, CertificateTemplateResponse,
    CreateCertificateAuthorityRequest, CreateCertificateTemplateRequest,
    ListCertificateAuthoritiesResponse, ListCertificateTemplatesResponse,
    RequestCertificateRequest, RequestCertificateResponse, UpdateCertificateAuthorityRequest,
};
use crate::paths;

pub struct CertificatesEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> CertificatesEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Get the current MDM Apple certificate
    pub async fn get_mdm_apple(&self) -> Result<Certificate> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::CERTIFICATE)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a certificate authority (CA)
    pub async fn create_ca(
        &self,
        request_body: CreateCertificateAuthorityRequest,
    ) -> Result<CertificateAuthority> {
        request_body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::CERTIFICATE_AUTHORITIES)?
            .json(&request_body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Update a certificate authority (CA)
    pub async fn update_ca(
        &self,
        id: u64,
        request_body: UpdateCertificateAuthorityRequest,
    ) -> Result<()> {
        request_body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::PATCH, &paths::certificate_authority(id))?
            .json(&request_body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// List certificate authorities
    pub async fn list_ca(&self) -> Result<ListCertificateAuthoritiesResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::CERTIFICATE_AUTHORITIES)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get certificate authority by ID
    pub async fn get_ca(&self, id: u64) -> Result<CertificateAuthority> {
        let request = self
            .client
            .request(reqwest::Method::GET, &paths::certificate_authority(id))?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Delete certificate authority
    pub async fn delete_ca(&self, id: u64) -> Result<()> {
        let request = self
            .client
            .request(reqwest::Method::DELETE, &paths::certificate_authority(id))?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Create a certificate template
    pub async fn create_certificate(
        &self,
        request_body: CreateCertificateTemplateRequest,
    ) -> Result<CertificateTemplateResponse> {
        request_body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::CERTIFICATES)?
            .json(&request_body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// List certificate templates
    pub async fn list_certificates(&self) -> Result<ListCertificateTemplatesResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::CERTIFICATES)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get certificate template by ID
    pub async fn get_certificate(&self, id: u64) -> Result<CertificateTemplate> {
        let request = self
            .client
            .request(reqwest::Method::GET, &paths::certificate(id))?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Delete certificate template
    pub async fn delete_certificate(&self, id: u64) -> Result<()> {
        let request = self
            .client
            .request(reqwest::Method::DELETE, &paths::certificate(id))?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Request a certificate from a CA
    pub async fn request_certificate(
        &self,
        ca_id: u64,
        request_body: RequestCertificateRequest,
    ) -> Result<RequestCertificateResponse> {
        if ca_id == 0 {
            return Err(crate::FleetError::Validation(
                "certificate authority ID must be greater than zero".into(),
            ));
        }
        request_body.validate()?;
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                &paths::certificate_authority_request_certificate(ca_id),
            )?
            .json(&request_body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
