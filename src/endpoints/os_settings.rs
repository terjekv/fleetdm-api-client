use crate::client::FleetClient;
use crate::error::{FleetError, Result};
use crate::models::common::FileDownload;
use crate::models::os_settings::*;
use crate::paths;

fn with_fleet_id(path: &str, fleet_id: Option<u64>) -> String {
    let params = fleet_id
        .map(|id| vec![("fleet_id".to_string(), id.to_string())])
        .unwrap_or_default();
    crate::http::append_query_params(path, &params)
}

/// OS settings endpoint for managing configuration profiles and disk encryption.
pub struct OsSettingsEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> OsSettingsEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// List configuration profiles.
    pub async fn list_profiles(
        &self,
        fleet_id: Option<u64>,
    ) -> Result<ListConfigurationProfilesResponse> {
        let query = ListConfigurationProfilesQuery {
            fleet_id,
            ..Default::default()
        };
        self.list_profiles_with_options(&query).await
    }

    /// List configuration profiles with pagination and ordering.
    pub async fn list_profiles_with_options(
        &self,
        query: &ListConfigurationProfilesQuery,
    ) -> Result<ListConfigurationProfilesResponse> {
        query.validate()?;
        let path = crate::http::append_query_params(
            paths::CONFIGURATION_PROFILES,
            &query.to_query_params(),
        );
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create a single configuration profile.
    pub async fn create_profile(
        &self,
        req: CreateConfigurationProfileRequest,
    ) -> Result<CreateConfigurationProfileResponse> {
        req.validate()?;
        let response = crate::http::send_rebuildable_request(
            || {
                let mut form =
                    reqwest::multipart::Form::new().part("profile", req.profile.to_part()?);
                if let Some(fleet_id) = req.fleet_id {
                    form = form.text("fleet_id", fleet_id.to_string());
                }
                for label in &req.labels_include_all {
                    form = form.text("labels_include_all", label.clone());
                }
                for label in &req.labels_include_any {
                    form = form.text("labels_include_any", label.clone());
                }
                for label in &req.labels_exclude_any {
                    form = form.text("labels_exclude_any", label.clone());
                }
                Ok(self
                    .client
                    .request(reqwest::Method::POST, paths::CONFIGURATION_PROFILES)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        crate::http::handle_response(response).await
    }

    /// Create configuration profiles in batch.
    pub async fn create_profiles_batch(
        &self,
        body: &BatchConfigurationProfilesRequest,
    ) -> Result<()> {
        let encoded_body = body.validate_and_body()?;
        let path = crate::http::append_query_params(
            paths::CONFIGURATION_PROFILES_BATCH,
            &body.to_query_params(),
        );
        let request = self
            .client
            .request(reqwest::Method::POST, &path)?
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(encoded_body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Resend profiles in batch.
    pub async fn resend_profiles_batch(
        &self,
        body: &ResendConfigurationProfilesRequest,
    ) -> Result<()> {
        body.validate()?;
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                paths::CONFIGURATION_PROFILES_RESEND_BATCH,
            )?
            .json(body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Get a configuration profile by UUID.
    pub async fn get_profile(&self, profile_uuid: &str) -> Result<GetConfigurationProfileResponse> {
        validate_profile_uuid(profile_uuid)?;
        let path = paths::configuration_profile(profile_uuid);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Download the original configuration-profile file.
    pub async fn download_profile(&self, profile_uuid: &str) -> Result<FileDownload> {
        validate_profile_uuid(profile_uuid)?;
        let base = paths::configuration_profile(profile_uuid);
        let path = crate::http::append_query(&base, Some(&[("alt", "media")]));
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        crate::http::handle_file_response(response).await
    }

    /// Get configuration profile install status.
    pub async fn get_profile_status(
        &self,
        profile_uuid: &str,
    ) -> Result<ConfigurationProfileStatus> {
        validate_profile_uuid(profile_uuid)?;
        let path = paths::configuration_profile_status(profile_uuid);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Delete a configuration profile by UUID.
    pub async fn delete_profile(&self, profile_uuid: &str) -> Result<()> {
        validate_profile_uuid(profile_uuid)?;
        let path = paths::configuration_profile(profile_uuid);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Get configuration profile summary.
    pub async fn profiles_summary(
        &self,
        fleet_id: Option<u64>,
    ) -> Result<ConfigurationProfileStatus> {
        let path = with_fleet_id(paths::CONFIGURATION_PROFILES_SUMMARY, fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get disk encryption settings.
    pub async fn get_disk_encryption(
        &self,
        fleet_id: Option<u64>,
    ) -> Result<DiskEncryptionSummary> {
        let path = with_fleet_id(paths::DISK_ENCRYPTION, fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Update disk encryption settings.
    ///
    /// Spec route is `POST /api/v1/fleet/disk_encryption`.
    pub async fn update_disk_encryption(&self, req: UpdateDiskEncryptionRequest) -> Result<()> {
        req.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::DISK_ENCRYPTION)?
            .json(&req);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }
}

fn validate_profile_uuid(profile_uuid: &str) -> Result<()> {
    if profile_uuid.trim().is_empty() {
        return Err(FleetError::Validation(
            "profile UUID must not be empty".into(),
        ));
    }
    Ok(())
}
