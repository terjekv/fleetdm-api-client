//! Setup experience endpoints.

use crate::client::FleetClient;
use crate::error::{FleetError, Result};
use crate::http::handle_file_response;
use crate::models::common::{FileDownload, FileUpload};
use crate::models::setup_experience::{
    AutomaticEnrollmentProfile, EulaMetadata, ListSetupExperienceSoftwareQuery,
    ListSetupExperienceSoftwareResponse, SetupExperienceScript, SetupExperienceScriptRequest,
    UpdateAutomaticEnrollmentProfileRequest, UpdateSetupExperienceRequest,
    UpdateSetupExperienceSoftwareRequest,
};
use crate::paths;

fn with_fleet_id(path: &str, fleet_id: Option<u64>) -> String {
    let params = fleet_id
        .map(|id| vec![("fleet_id".to_string(), id.to_string())])
        .unwrap_or_default();
    crate::http::append_query_params(path, &params)
}

pub struct SetupExperienceEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> SetupExperienceEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// PATCH /api/v1/fleet/setup_experience
    pub async fn update(&self, body: &UpdateSetupExperienceRequest) -> Result<()> {
        body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::PATCH, paths::SETUP_EXPERIENCE)?
            .json(body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// POST /api/v1/fleet/setup_experience/eula
    pub async fn create_eula(&self, eula: FileUpload) -> Result<()> {
        if !eula.filename().to_ascii_lowercase().ends_with(".pdf") {
            return Err(FleetError::Validation("EULA must be a PDF file".into()));
        }
        if eula.bytes().len() > 25 * 1024 * 1024 {
            return Err(FleetError::Validation(
                "EULA exceeds Fleet's 25 MiB upload limit".into(),
            ));
        }
        let response = crate::http::send_rebuildable_request(
            || {
                let form = reqwest::multipart::Form::new().part("eula", eula.to_part()?);
                Ok(self
                    .client
                    .request(reqwest::Method::POST, paths::SETUP_EXPERIENCE_EULA)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        crate::http::handle_empty_response(response).await
    }

    /// GET /api/v1/fleet/setup_experience/eula/metadata
    pub async fn eula_metadata(&self) -> Result<EulaMetadata> {
        let request = self
            .client
            .request(reqwest::Method::GET, paths::SETUP_EXPERIENCE_EULA_METADATA)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// GET /api/v1/fleet/setup_experience/eula/:token
    pub async fn get_eula(&self, token: &str) -> Result<FileDownload> {
        validate_nonempty("EULA token", token)?;
        let path = paths::setup_experience_eula(token);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        handle_file_response(response).await
    }

    /// DELETE /api/v1/fleet/setup_experience/eula/:token
    pub async fn delete_eula(&self, token: &str) -> Result<()> {
        validate_nonempty("EULA token", token)?;
        let path = paths::setup_experience_eula(token);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// GET /api/v1/fleet/setup_experience/software
    pub async fn get_software(
        &self,
        fleet_id: Option<u64>,
    ) -> Result<ListSetupExperienceSoftwareResponse> {
        let query = ListSetupExperienceSoftwareQuery {
            fleet_id,
            ..Default::default()
        };
        self.get_software_with_options(&query).await
    }

    /// List setup-experience software with filtering, ordering, and pagination.
    pub async fn get_software_with_options(
        &self,
        query: &ListSetupExperienceSoftwareQuery,
    ) -> Result<ListSetupExperienceSoftwareResponse> {
        query.validate()?;
        let path = crate::http::append_query_params(
            paths::SETUP_EXPERIENCE_SOFTWARE,
            &query.to_query_params(),
        );
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// PUT /api/v1/fleet/setup_experience/software
    pub async fn set_software(&self, body: &UpdateSetupExperienceSoftwareRequest) -> Result<()> {
        body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::PUT, paths::SETUP_EXPERIENCE_SOFTWARE)?
            .json(body);
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// GET /api/v1/fleet/setup_experience/script
    pub async fn get_script(&self, fleet_id: Option<u64>) -> Result<SetupExperienceScript> {
        let path = with_fleet_id(paths::SETUP_EXPERIENCE_SCRIPT, fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// POST /api/v1/fleet/setup_experience/script
    pub async fn create_script(&self, body: SetupExperienceScriptRequest) -> Result<()> {
        if !body.script.filename().to_ascii_lowercase().ends_with(".sh") {
            return Err(FleetError::Validation(
                "setup-experience script must be a .sh file".into(),
            ));
        }
        let response = crate::http::send_rebuildable_request(
            || {
                let mut form =
                    reqwest::multipart::Form::new().part("script", body.script.to_part()?);
                if let Some(fleet_id) = body.fleet_id {
                    form = form.text("fleet_id", fleet_id.to_string());
                }
                Ok(self
                    .client
                    .request(reqwest::Method::POST, paths::SETUP_EXPERIENCE_SCRIPT)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        crate::http::handle_empty_response(response).await
    }

    /// Download the configured setup-experience script.
    pub async fn download_script(&self, fleet_id: Option<u64>) -> Result<FileDownload> {
        let mut params = fleet_id
            .map(|id| vec![("fleet_id".to_string(), id.to_string())])
            .unwrap_or_default();
        params.push(("alt".to_string(), "media".to_string()));
        let path = crate::http::append_query_params(paths::SETUP_EXPERIENCE_SCRIPT, &params);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        handle_file_response(response).await
    }

    /// DELETE /api/v1/fleet/setup_experience/script
    pub async fn delete_script(&self, fleet_id: Option<u64>) -> Result<()> {
        let path = with_fleet_id(paths::SETUP_EXPERIENCE_SCRIPT, fleet_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// GET /api/v1/fleet/enrollment_profiles/automatic
    pub async fn enrollment_profile_automatic(
        &self,
        fleet_id: Option<u64>,
    ) -> Result<AutomaticEnrollmentProfile> {
        let path = with_fleet_id(paths::ENROLLMENT_PROFILES_AUTOMATIC, fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// POST /api/v1/fleet/enrollment_profiles/automatic
    pub async fn set_enrollment_profile_automatic(
        &self,
        body: &UpdateAutomaticEnrollmentProfileRequest,
    ) -> Result<AutomaticEnrollmentProfile> {
        body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::ENROLLMENT_PROFILES_AUTOMATIC)?
            .json(body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// DELETE /api/v1/fleet/enrollment_profiles/automatic
    pub async fn delete_enrollment_profile_automatic(&self, fleet_id: Option<u64>) -> Result<()> {
        let path = with_fleet_id(paths::ENROLLMENT_PROFILES_AUTOMATIC, fleet_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// GET /api/v1/fleet/enrollment_profiles/manual
    pub async fn enrollment_profile_manual(&self, fleet_id: Option<u64>) -> Result<FileDownload> {
        let path = with_fleet_id(paths::ENROLLMENT_PROFILES_MANUAL, fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        handle_file_response(response).await
    }

    /// GET /api/v1/fleet/enrollment_profiles/ota
    pub async fn enrollment_profile_ota(&self, fleet_id: Option<u64>) -> Result<FileDownload> {
        let path = with_fleet_id(paths::ENROLLMENT_PROFILES_OTA, fleet_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        handle_file_response(response).await
    }
}

fn validate_nonempty(name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(FleetError::Validation(format!("{name} must not be empty")));
    }
    Ok(())
}
