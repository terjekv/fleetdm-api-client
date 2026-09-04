//! Integrations endpoints for APNs, ABM, VPP, SCIM, and Android Enterprise

use crate::client::FleetClient;
use crate::error::Result;
use crate::models::integrations::*;
use crate::paths;

/// Integrations endpoint management
pub struct IntegrationsEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> IntegrationsEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Get Apple Push Notification service (APNs) certificate information
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example(client: &FleetClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let apns = client.integrations().get_apns().await?;
    /// println!("APNs cert expires: {}", apns.renew_date());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_apns(&self) -> Result<ApnsInfo> {
        let request = self.client.request(reqwest::Method::GET, paths::APNS)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// List Apple Business Manager (ABM) tokens
    ///
    /// Available in Fleet Premium
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example(client: &FleetClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let tokens = client.integrations().list_abm_tokens().await?;
    /// for token in tokens.abm_tokens() {
    ///     println!("ABM token: {} ({})", token.org_name(), token.apple_id());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_abm_tokens(&self) -> Result<AbmTokenListResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::ABM_TOKENS)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// List Volume Purchasing Program (VPP) tokens
    ///
    /// Available in Fleet Premium
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example(client: &FleetClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let tokens = client.integrations().list_vpp_tokens().await?;
    /// for token in tokens.vpp_tokens() {
    ///     println!("VPP token: {} ({} teams)", token.org_name(), token.teams().len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_vpp_tokens(&self) -> Result<VppTokenListResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::VPP_TOKENS)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get identity provider (IdP) details from SCIM
    ///
    /// Returns details about the most recent SCIM (System for Cross-domain Identity Management)
    /// request from your identity provider.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example(client: &FleetClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let scim = client.integrations().get_scim_details().await?;
    /// println!("Last SCIM request: {} at {}",
    ///          scim.last_request().status(),
    ///          scim.last_request().requested_at());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_scim_details(&self) -> Result<ScimDetailsResponse> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::SCIM_DETAILS)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get Android Enterprise information
    ///
    /// **Experimental feature** - This feature is undergoing rapid improvement and may result
    /// in breaking changes. Not recommended for use in automated workflows.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example(client: &FleetClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let android = client.integrations().get_android_enterprise().await?;
    /// println!("Android Enterprise ID: {}", android.android_enterprise_id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_android_enterprise(&self) -> Result<AndroidEnterpriseInfo> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::ANDROID_ENTERPRISE)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
