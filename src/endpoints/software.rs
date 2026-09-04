use crate::client::FleetClient;
use crate::error::{FleetError, Result};
use crate::models::common::{FileDownload, FileDownloadStream};
use crate::models::software::{
    CreateAppStoreAppRequest, CreateAppStoreAppResponse, CreateFleetMaintainedAppRequest,
    CreateFleetMaintainedAppResponse, GetFleetMaintainedAppResponse,
    GetOperatingSystemVersionResponse, GetSoftwareResponse, GetSoftwareTitleResponse,
    ListAppStoreAppsResponse, ListFleetMaintainedAppsResponse, ListOperatingSystemVersionsResponse,
    ListSoftwareQuery, ListSoftwareResponse, ListSoftwareTitlesResponse, SoftwareIconSource,
    SoftwareInstallResult, SoftwarePackageOptions, SoftwarePackageResponse,
    UpdateAppStoreAppRequest, UpdateAppStoreAppResponse, UpdateSoftwareIconRequest,
    UpdateSoftwareIconResponse, UpdateSoftwarePackageRequest, UploadSoftwarePackageRequest,
};
use crate::paths;

fn add_package_options(
    mut form: reqwest::multipart::Form,
    options: &SoftwarePackageOptions,
) -> reqwest::multipart::Form {
    if let Some(fleet_id) = options.fleet_id {
        form = form.text("fleet_id", fleet_id.to_string());
    }
    for (name, value) in [
        ("install_script", options.install_script.as_ref()),
        ("uninstall_script", options.uninstall_script.as_ref()),
        ("pre_install_query", options.pre_install_query.as_ref()),
        ("post_install_script", options.post_install_script.as_ref()),
        ("display_name", options.display_name.as_ref()),
        ("version", options.version.as_ref()),
    ] {
        if let Some(value) = value {
            form = form.text(name, value.clone());
        }
    }
    for (name, value) in [
        ("self_service", options.self_service),
        ("automatic_install", options.automatic_install),
    ] {
        if let Some(value) = value {
            form = form.text(name, value.to_string());
        }
    }
    for (name, values) in [
        ("labels_include_all", &options.labels_include_all),
        ("labels_include_any", &options.labels_include_any),
        ("labels_exclude_any", &options.labels_exclude_any),
        ("categories", &options.categories),
    ] {
        for value in values {
            form = form.text(name, value.clone());
        }
    }
    form
}

/// Endpoint for software-related operations.
pub struct SoftwareEndpoint {
    client: FleetClient,
}

impl SoftwareEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    fn with_query(path: &str, query: Option<&[(&str, &str)]>) -> String {
        crate::http::append_query(path, query)
    }

    /// List software versions with a builder pattern.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_token("token")?
    ///     .build();
    ///
    /// let response = client.software().list().per_page(10).send().await?;
    /// for sw in response.software() {
    ///     println!("{} v{}", sw.name(), sw.version());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListSoftwareBuilder<'_> {
        ListSoftwareBuilder {
            client: &self.client,
            query: ListSoftwareQuery::new(),
        }
    }

    pub async fn list_titles(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<ListSoftwareTitlesResponse> {
        let path = Self::with_query(paths::SOFTWARE, query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn get_title(
        &self,
        title_id: u64,
        query: Option<&[(&str, &str)]>,
    ) -> Result<GetSoftwareTitleResponse> {
        let path = Self::with_query(&paths::software_item(title_id), query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn list_versions(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<ListSoftwareResponse> {
        let path = Self::with_query(paths::SOFTWARE_VERSIONS, query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn get_version(
        &self,
        version_id: u64,
        query: Option<&[(&str, &str)]>,
    ) -> Result<GetSoftwareResponse> {
        let path = Self::with_query(&paths::software_version(version_id), query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn list_os_versions(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<ListOperatingSystemVersionsResponse> {
        let path = Self::with_query(paths::OS_VERSIONS, query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn get_os_version(
        &self,
        os_version_id: u64,
        query: Option<&[(&str, &str)]>,
    ) -> Result<GetOperatingSystemVersionResponse> {
        let path = Self::with_query(&paths::os_version(os_version_id), query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn upload_package(
        &self,
        body: UploadSoftwarePackageRequest,
    ) -> Result<SoftwarePackageResponse> {
        body.validate()?;
        let response = crate::http::send_rebuildable_request(
            || {
                let form =
                    reqwest::multipart::Form::new().part("software", body.software.to_part()?);
                let form = add_package_options(form, &body.options);
                Ok(self
                    .client
                    .request(reqwest::Method::POST, paths::SOFTWARE_PACKAGE)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        crate::http::handle_response(response).await
    }

    pub async fn create_app_store_app(
        &self,
        body: &CreateAppStoreAppRequest,
    ) -> Result<CreateAppStoreAppResponse> {
        body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::APP_STORE_APPS)?
            .json(body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn patch_package(
        &self,
        title_id: u64,
        body: UpdateSoftwarePackageRequest,
    ) -> Result<SoftwarePackageResponse> {
        body.validate()?;
        let path = paths::software_package(title_id);
        let response = crate::http::send_rebuildable_request(
            || {
                let mut form =
                    reqwest::multipart::Form::new().text("fleet_id", body.fleet_id.to_string());
                if let Some(software) = &body.software {
                    form = form.part("software", software.to_part()?);
                }
                form = add_package_options(form, &body.options);
                Ok(self
                    .client
                    .request(reqwest::Method::PATCH, &path)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        crate::http::handle_response(response).await
    }

    pub async fn patch_app_store_app(
        &self,
        title_id: u64,
        body: &UpdateAppStoreAppRequest,
    ) -> Result<UpdateAppStoreAppResponse> {
        body.validate()?;
        let path = paths::software_app_store_app(title_id);
        let request = self
            .client
            .request(reqwest::Method::PATCH, &path)?
            .json(body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn put_icon(
        &self,
        title_id: u64,
        body: UpdateSoftwareIconRequest,
    ) -> Result<UpdateSoftwareIconResponse> {
        body.validate()?;
        let base = paths::software_icon(title_id);
        let path = crate::http::append_query_params(
            &base,
            &[("fleet_id".to_string(), body.fleet_id.to_string())],
        );
        let response = crate::http::send_rebuildable_request(
            || {
                let form = match &body.source {
                    SoftwareIconSource::Png(icon) => {
                        reqwest::multipart::Form::new().part("icon", icon.to_part()?)
                    }
                    SoftwareIconSource::Existing {
                        hash_sha256,
                        filename,
                    } => reqwest::multipart::Form::new()
                        .text("hash_sha256", hash_sha256.clone())
                        .text("filename", filename.clone()),
                };
                Ok(self
                    .client
                    .request(reqwest::Method::PUT, &path)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        crate::http::handle_response(response).await
    }

    pub async fn get_icon(&self, title_id: u64, fleet_id: u64) -> Result<FileDownload> {
        let base = paths::software_icon(title_id);
        let path = crate::http::append_query_params(
            &base,
            &[("fleet_id".to_string(), fleet_id.to_string())],
        );
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        crate::http::handle_file_response(response).await
    }

    pub async fn delete_icon(&self, title_id: u64, fleet_id: u64) -> Result<()> {
        let base = paths::software_icon(title_id);
        let path = crate::http::append_query_params(
            &base,
            &[("fleet_id".to_string(), fleet_id.to_string())],
        );
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Download a software package as a stream.
    ///
    /// Fleet permits multi-gigabyte packages, so this endpoint deliberately
    /// avoids buffering the successful response in memory. Read it incrementally
    /// with [`FileDownloadStream::next_chunk`].
    pub async fn download_package(
        &self,
        title_id: u64,
        fleet_id: u64,
    ) -> Result<FileDownloadStream> {
        let base = paths::software_package(title_id);
        let path = crate::http::append_query_params(
            &base,
            &[
                ("fleet_id".to_string(), fleet_id.to_string()),
                ("alt".to_string(), "media".to_string()),
            ],
        );
        let request = self.client.request(reqwest::Method::GET, &path)?;
        let response =
            crate::http::send_prepared_request(request, self.client.retry_policy()).await?;
        crate::http::handle_file_stream_response(response).await
    }

    pub async fn delete_available_for_install(&self, title_id: u64, fleet_id: u64) -> Result<()> {
        let base = paths::software_available_for_install(title_id);
        let path = crate::http::append_query_params(
            &base,
            &[("fleet_id".to_string(), fleet_id.to_string())],
        );
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    pub async fn list_fleet_maintained_apps(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<ListFleetMaintainedAppsResponse> {
        let path = Self::with_query(paths::FLEET_MAINTAINED_APPS, query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn get_fleet_maintained_app(
        &self,
        app_id: u64,
        query: Option<&[(&str, &str)]>,
    ) -> Result<GetFleetMaintainedAppResponse> {
        let base = paths::fleet_maintained_app(app_id);
        let path = Self::with_query(&base, query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn create_fleet_maintained_app(
        &self,
        body: &CreateFleetMaintainedAppRequest,
    ) -> Result<CreateFleetMaintainedAppResponse> {
        body.validate()?;
        let request = self
            .client
            .request(reqwest::Method::POST, paths::FLEET_MAINTAINED_APPS)?
            .json(body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn list_app_store_apps(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<ListAppStoreAppsResponse> {
        let path = Self::with_query(paths::APP_STORE_APPS, query);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    pub async fn install_on_host(&self, host_id: u64, title_id: u64) -> Result<()> {
        validate_action_ids(host_id, title_id)?;
        let path = paths::host_software_action(host_id, title_id, "install");
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    pub async fn uninstall_on_host(&self, host_id: u64, title_id: u64) -> Result<()> {
        validate_action_ids(host_id, title_id)?;
        let path = paths::host_software_action(host_id, title_id, "uninstall");
        let request = self.client.request(reqwest::Method::POST, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    pub async fn install_result(&self, install_uuid: &str) -> Result<SoftwareInstallResult> {
        if install_uuid.trim().is_empty() {
            return Err(FleetError::Validation(
                "install UUID must not be empty".into(),
            ));
        }
        let path = paths::software_install_result(install_uuid);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

fn validate_action_ids(host_id: u64, title_id: u64) -> Result<()> {
    if host_id == 0 || title_id == 0 {
        return Err(FleetError::Validation(
            "host_id and title_id must be greater than zero".into(),
        ));
    }
    Ok(())
}

/// Builder for constructing list software queries
pub struct ListSoftwareBuilder<'a> {
    client: &'a FleetClient,
    query: ListSoftwareQuery,
}

impl<'a> ListSoftwareBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
        self
    }

    pub fn order_key(mut self, key: crate::models::software::SoftwareOrderKey) -> Self {
        self.query = self.query.order_key(key);
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.query = self.query.order_direction(direction);
        self
    }

    pub fn team_id(mut self, team_id: u64) -> Self {
        self.query = self.query.team_id(team_id);
        self
    }

    pub fn fleet_id(mut self, fleet_id: u64) -> Self {
        self.query = self.query.fleet_id(fleet_id);
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = self.query.query(query);
        self
    }

    pub fn vulnerable(mut self, vulnerable: bool) -> Self {
        self.query = self.query.vulnerable(vulnerable);
        self
    }

    /// Execute the list software query
    pub async fn send(self) -> Result<ListSoftwareResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(paths::SOFTWARE_VERSIONS, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
