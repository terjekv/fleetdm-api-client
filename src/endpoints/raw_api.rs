use crate::client::FleetClient;
use crate::error::{FleetError, Result};
use crate::http::{
    MAX_RAW_RESPONSE_SIZE, append_query, encode_path_segment, handle_response, read_limited_body,
    send_with_retry,
};
use crate::models::{ApiQuery, ApiRequestBody, ApiResponse};

/// Generated authenticated transport wrappers for routes in docs/fleet-rest-api-main.md.
///
/// Successful responses preserve status, headers, and bytes. Public authentication
/// routes are exposed through `FleetClient::builder(...).auth()` instead. Curated
/// endpoint modules remain the preferred API for supported workflows.
pub struct RawApiEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> RawApiEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    async fn send(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let path = append_query(path, query);
        let send_once = || async {
            let mut request = self.client.request(method.clone(), &path)?;
            if let Some(b) = body {
                request = b.apply(request)?;
            }
            request.send().await.map_err(Into::into)
        };
        let response = if let Some(policy) = self.client.retry_policy() {
            send_with_retry(policy, send_once).await?
        } else {
            send_once().await?
        };

        if !response.status().is_success() {
            let result: Result<serde_json::Value> = handle_response(response).await;
            return match result {
                Err(error) => Err(error),
                Ok(_) => Err(FleetError::Http(
                    "error response unexpectedly decoded as success".into(),
                )),
            };
        }

        let status = response.status();
        let headers = response.headers().clone();
        let bytes = read_limited_body(response, MAX_RAW_RESPONSE_SIZE).await?;
        Ok(ApiResponse::from_parts(status, headers, bytes))
    }

    /// `DELETE /api/v1/fleet/bootstrap/:fleet_id`
    pub async fn ep_delete_api_v1_fleet_bootstrap_by_fleet_id(
        &self,
        fleet_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/bootstrap/:fleet_id".to_string();
        let fleet_id_encoded = encode_path_segment(fleet_id.as_ref());
        path = path.replace(":fleet_id", &fleet_id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/certificate_authorities/:id`
    pub async fn ep_delete_api_v1_fleet_certificate_authorities_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/certificate_authorities/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/certificates/:id`
    pub async fn ep_delete_api_v1_fleet_certificates_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/certificates/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/conditional-access/microsoft`
    pub async fn ep_delete_api_v1_fleet_conditional_access_microsoft(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::DELETE,
            "/api/v1/fleet/conditional-access/microsoft",
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/fleet/configuration_profiles/:profile_uuid`
    pub async fn ep_delete_api_v1_fleet_configuration_profiles_by_profile_uuid(
        &self,
        profile_uuid: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/configuration_profiles/:profile_uuid".to_string();
        let profile_uuid_encoded = encode_path_segment(profile_uuid.as_ref());
        path = path.replace(":profile_uuid", &profile_uuid_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/custom_variables/:id`
    pub async fn ep_delete_api_v1_fleet_custom_variables_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/custom_variables/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/enrollment_profiles/automatic`
    pub async fn ep_delete_api_v1_fleet_enrollment_profiles_automatic(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::DELETE,
            "/api/v1/fleet/enrollment_profiles/automatic",
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/fleet/fleets/:id`
    pub async fn ep_delete_api_v1_fleet_fleets_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/hosts/:id`
    pub async fn ep_delete_api_v1_fleet_hosts_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/hosts/:id/activities/upcoming/:activity_id`
    pub async fn ep_delete_api_v1_fleet_hosts_by_id_activities_upcoming_by_activity_id(
        &self,
        id: impl AsRef<str>,
        activity_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/activities/upcoming/:activity_id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let activity_id_encoded = encode_path_segment(activity_id.as_ref());
        path = path.replace(":activity_id", &activity_id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/hosts/:id/labels`
    pub async fn ep_delete_api_v1_fleet_hosts_by_id_labels(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/labels".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/hosts/:id/mdm`
    pub async fn ep_delete_api_v1_fleet_hosts_by_id_mdm(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/mdm".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/invites/:id`
    pub async fn ep_delete_api_v1_fleet_invites_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/invites/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/labels/:name`
    pub async fn ep_delete_api_v1_fleet_labels_by_name(
        &self,
        name: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/labels/:name".to_string();
        let name_encoded = encode_path_segment(name.as_ref());
        path = path.replace(":name", &name_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/labels/id/:id`
    pub async fn ep_delete_api_v1_fleet_labels_id_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/labels/id/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/logo`
    pub async fn ep_delete_api_v1_fleet_logo(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::DELETE, "/api/v1/fleet/logo", query, body)
            .await
    }

    /// `DELETE /api/v1/fleet/reports/:name`
    pub async fn ep_delete_api_v1_fleet_reports_by_name(
        &self,
        name: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/reports/:name".to_string();
        let name_encoded = encode_path_segment(name.as_ref());
        path = path.replace(":name", &name_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/reports/id/:id`
    pub async fn ep_delete_api_v1_fleet_reports_id_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/reports/id/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/scim/Groups/:id`
    pub async fn ep_delete_api_v1_fleet_scim_groups_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Groups/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/scim/Users/:id`
    pub async fn ep_delete_api_v1_fleet_scim_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/scripts/:id`
    pub async fn ep_delete_api_v1_fleet_scripts_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scripts/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/sessions/:id`
    pub async fn ep_delete_api_v1_fleet_sessions_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/sessions/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/setup_experience/eula/:token`
    pub async fn ep_delete_api_v1_fleet_setup_experience_eula_by_token(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/setup_experience/eula/:token".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/setup_experience/script`
    pub async fn ep_delete_api_v1_fleet_setup_experience_script(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::DELETE,
            "/api/v1/fleet/setup_experience/script",
            query,
            body,
        )
        .await
    }

    /// `DELETE /api/v1/fleet/software/self_service_categories/:id`
    pub async fn ep_delete_api_v1_fleet_software_self_service_categories_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/self_service_categories/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/software/titles/:id/icon`
    pub async fn ep_delete_api_v1_fleet_software_titles_by_id_icon(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:id/icon".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/software/titles/:software_title_id/available_for_install`
    pub async fn ep_delete_api_v1_fleet_software_titles_by_software_title_id_available_for_install(
        &self,
        software_title_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path =
            "/api/v1/fleet/software/titles/:software_title_id/available_for_install".to_string();
        let software_title_id_encoded = encode_path_segment(software_title_id.as_ref());
        path = path.replace(":software_title_id", &software_title_id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/users/:id`
    pub async fn ep_delete_api_v1_fleet_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `DELETE /api/v1/fleet/users/:id/sessions`
    pub async fn ep_delete_api_v1_fleet_users_by_id_sessions(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/users/:id/sessions".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::DELETE, &path, query, body).await
    }

    /// `GET /api/v1/fleet/ab_tokens`
    pub async fn ep_get_api_v1_fleet_ab_tokens(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/ab_tokens", query, None)
            .await
    }

    /// `GET /api/v1/fleet/activities`
    pub async fn ep_get_api_v1_fleet_activities(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/activities",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/android_enterprise`
    pub async fn ep_get_api_v1_fleet_android_enterprise(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/android_enterprise",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/apns`
    pub async fn ep_get_api_v1_fleet_apns(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/apns", query, None)
            .await
    }

    /// `GET /api/v1/fleet/bootstrap`
    pub async fn ep_get_api_v1_fleet_bootstrap(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/bootstrap", query, None)
            .await
    }

    /// `GET /api/v1/fleet/bootstrap/:fleet_id/metadata`
    pub async fn ep_get_api_v1_fleet_bootstrap_by_fleet_id_metadata(
        &self,
        fleet_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/bootstrap/:fleet_id/metadata".to_string();
        let fleet_id_encoded = encode_path_segment(fleet_id.as_ref());
        path = path.replace(":fleet_id", &fleet_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/bootstrap/summary`
    pub async fn ep_get_api_v1_fleet_bootstrap_summary(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/bootstrap/summary",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/carves`
    pub async fn ep_get_api_v1_fleet_carves(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/carves", query, None)
            .await
    }

    /// `GET /api/v1/fleet/carves/:id`
    pub async fn ep_get_api_v1_fleet_carves_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/carves/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/carves/:id/block/:block_id`
    pub async fn ep_get_api_v1_fleet_carves_by_id_block_by_block_id(
        &self,
        id: impl AsRef<str>,
        block_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/carves/:id/block/:block_id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let block_id_encoded = encode_path_segment(block_id.as_ref());
        path = path.replace(":block_id", &block_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/certificate_authorities`
    pub async fn ep_get_api_v1_fleet_certificate_authorities(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/certificate_authorities",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/certificate_authorities/:id`
    pub async fn ep_get_api_v1_fleet_certificate_authorities_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/certificate_authorities/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/certificates`
    pub async fn ep_get_api_v1_fleet_certificates(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/certificates",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/certificates/:id`
    pub async fn ep_get_api_v1_fleet_certificates_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/certificates/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/charts/:metric`
    pub async fn ep_get_api_v1_fleet_charts_by_metric(
        &self,
        metric: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/charts/:metric".to_string();
        let metric_encoded = encode_path_segment(metric.as_ref());
        path = path.replace(":metric", &metric_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/commands`
    pub async fn ep_get_api_v1_fleet_commands(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/commands", query, None)
            .await
    }

    /// `GET /api/v1/fleet/commands/results`
    pub async fn ep_get_api_v1_fleet_commands_results(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/commands/results",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/conditional_access/idp/apple/profile`
    pub async fn ep_get_api_v1_fleet_conditional_access_idp_apple_profile(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/conditional_access/idp/apple/profile",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/conditional_access/idp/signing_cert`
    pub async fn ep_get_api_v1_fleet_conditional_access_idp_signing_cert(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/conditional_access/idp/signing_cert",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/config`
    pub async fn ep_get_api_v1_fleet_config(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/config", query, None)
            .await
    }

    /// `GET /api/v1/fleet/config/certificate`
    pub async fn ep_get_api_v1_fleet_config_certificate(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/config/certificate",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/configuration_profiles`
    pub async fn ep_get_api_v1_fleet_configuration_profiles(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/configuration_profiles",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/configuration_profiles/:profile_uuid`
    pub async fn ep_get_api_v1_fleet_configuration_profiles_by_profile_uuid(
        &self,
        profile_uuid: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/configuration_profiles/:profile_uuid".to_string();
        let profile_uuid_encoded = encode_path_segment(profile_uuid.as_ref());
        path = path.replace(":profile_uuid", &profile_uuid_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/configuration_profiles/:profile_uuid/status`
    pub async fn ep_get_api_v1_fleet_configuration_profiles_by_profile_uuid_status(
        &self,
        profile_uuid: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/configuration_profiles/:profile_uuid/status".to_string();
        let profile_uuid_encoded = encode_path_segment(profile_uuid.as_ref());
        path = path.replace(":profile_uuid", &profile_uuid_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/configuration_profiles/summary`
    pub async fn ep_get_api_v1_fleet_configuration_profiles_summary(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/configuration_profiles/summary",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/custom_variables`
    pub async fn ep_get_api_v1_fleet_custom_variables(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/custom_variables",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/device/:token`
    pub async fn ep_get_api_v1_fleet_device_by_token(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/device/:token".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/device/:token/software`
    pub async fn ep_get_api_v1_fleet_device_by_token_software(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/device/:token/software".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/disk_encryption`
    pub async fn ep_get_api_v1_fleet_disk_encryption(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/disk_encryption",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/enrollment_profiles/automatic`
    pub async fn ep_get_api_v1_fleet_enrollment_profiles_automatic(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/enrollment_profiles/automatic",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/enrollment_profiles/automatic/default`
    pub async fn ep_get_api_v1_fleet_enrollment_profiles_automatic_default(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/enrollment_profiles/automatic/default",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/enrollment_profiles/manual`
    pub async fn ep_get_api_v1_fleet_enrollment_profiles_manual(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/enrollment_profiles/manual",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/enrollment_profiles/ota`
    pub async fn ep_get_api_v1_fleet_enrollment_profiles_ota(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/enrollment_profiles/ota",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/fleets`
    pub async fn ep_get_api_v1_fleet_fleets(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/fleets", query, None)
            .await
    }

    /// `GET /api/v1/fleet/fleets/:fleet_id/policies/:policy_id`
    pub async fn ep_get_api_v1_fleet_fleets_by_fleet_id_policies_by_policy_id(
        &self,
        fleet_id: impl AsRef<str>,
        policy_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:fleet_id/policies/:policy_id".to_string();
        let fleet_id_encoded = encode_path_segment(fleet_id.as_ref());
        path = path.replace(":fleet_id", &fleet_id_encoded);
        let policy_id_encoded = encode_path_segment(policy_id.as_ref());
        path = path.replace(":policy_id", &policy_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/fleets/:fleet_id/policies/count`
    pub async fn ep_get_api_v1_fleet_fleets_by_fleet_id_policies_count(
        &self,
        fleet_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:fleet_id/policies/count".to_string();
        let fleet_id_encoded = encode_path_segment(fleet_id.as_ref());
        path = path.replace(":fleet_id", &fleet_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/fleets/:id`
    pub async fn ep_get_api_v1_fleet_fleets_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/fleets/:id/policies`
    pub async fn ep_get_api_v1_fleet_fleets_by_id_policies(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id/policies".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/fleets/:id/secrets`
    pub async fn ep_get_api_v1_fleet_fleets_by_id_secrets(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id/secrets".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/global/policies`
    pub async fn ep_get_api_v1_fleet_global_policies(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/global/policies",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/global/policies/:id`
    pub async fn ep_get_api_v1_fleet_global_policies_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/global/policies/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/host_summary`
    pub async fn ep_get_api_v1_fleet_host_summary(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/host_summary",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/hosts`
    pub async fn ep_get_api_v1_fleet_hosts(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/hosts", query, None)
            .await
    }

    /// `GET /api/v1/fleet/hosts/:id`
    pub async fn ep_get_api_v1_fleet_hosts_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/activities`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_activities(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/activities".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/activities/upcoming`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_activities_upcoming(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/activities/upcoming".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/certificates`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_certificates(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/certificates".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/configuration_profiles`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_configuration_profiles(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/configuration_profiles".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/device_url`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_device_url(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/device_url".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/encryption_key`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_encryption_key(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/encryption_key".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/health`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_health(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/health".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/macadmins`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_macadmins(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/macadmins".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/managed_account_password`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_managed_account_password(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/managed_account_password".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/mdm`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_mdm(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/mdm".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/recovery_lock_password`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_recovery_lock_password(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/recovery_lock_password".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/reports`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_reports(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/reports".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/reports/:report_id`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_reports_by_report_id(
        &self,
        id: impl AsRef<str>,
        report_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/reports/:report_id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let report_id_encoded = encode_path_segment(report_id.as_ref());
        path = path.replace(":report_id", &report_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/scripts`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_scripts(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/scripts".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/:id/software`
    pub async fn ep_get_api_v1_fleet_hosts_by_id_software(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/software".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/count`
    pub async fn ep_get_api_v1_fleet_hosts_count(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/hosts/count",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/hosts/identifier/:identifier`
    pub async fn ep_get_api_v1_fleet_hosts_identifier_by_identifier(
        &self,
        identifier: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/identifier/:identifier".to_string();
        let identifier_encoded = encode_path_segment(identifier.as_ref());
        path = path.replace(":identifier", &identifier_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/hosts/report`
    pub async fn ep_get_api_v1_fleet_hosts_report(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/hosts/report",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/hosts/summary/mdm`
    pub async fn ep_get_api_v1_fleet_hosts_summary_mdm(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/hosts/summary/mdm",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/invites`
    pub async fn ep_get_api_v1_fleet_invites(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/invites", query, None)
            .await
    }

    /// `GET /api/v1/fleet/invites/:token`
    pub async fn ep_get_api_v1_fleet_invites_by_token(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/invites/:token".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/labels`
    pub async fn ep_get_api_v1_fleet_labels(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/labels", query, None)
            .await
    }

    /// `GET /api/v1/fleet/labels/:id`
    pub async fn ep_get_api_v1_fleet_labels_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/labels/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/labels/:id/hosts`
    pub async fn ep_get_api_v1_fleet_labels_by_id_hosts(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/labels/:id/hosts".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/labels/summary`
    pub async fn ep_get_api_v1_fleet_labels_summary(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/labels/summary",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/macadmins`
    pub async fn ep_get_api_v1_fleet_macadmins(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/macadmins", query, None)
            .await
    }

    /// `GET /api/v1/fleet/me`
    pub async fn ep_get_api_v1_fleet_me(&self, query: Option<ApiQuery<'_>>) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/me", query, None)
            .await
    }

    /// `GET /api/v1/fleet/os_versions`
    pub async fn ep_get_api_v1_fleet_os_versions(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/os_versions",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/os_versions/:id`
    pub async fn ep_get_api_v1_fleet_os_versions_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/os_versions/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/policies/count`
    pub async fn ep_get_api_v1_fleet_policies_count(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/policies/count",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/reports`
    pub async fn ep_get_api_v1_fleet_reports(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/reports", query, None)
            .await
    }

    /// `GET /api/v1/fleet/reports/:id`
    pub async fn ep_get_api_v1_fleet_reports_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/reports/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/reports/:id/report`
    pub async fn ep_get_api_v1_fleet_reports_by_id_report(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/reports/:id/report".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/rest_api`
    pub async fn ep_get_api_v1_fleet_rest_api(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/rest_api", query, None)
            .await
    }

    /// `GET /api/v1/fleet/scim/Groups`
    pub async fn ep_get_api_v1_fleet_scim_groups(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scim/Groups",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scim/Groups/:id`
    pub async fn ep_get_api_v1_fleet_scim_groups_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Groups/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/scim/ResourceTypes`
    pub async fn ep_get_api_v1_fleet_scim_resourcetypes(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scim/ResourceTypes",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scim/Schemas`
    pub async fn ep_get_api_v1_fleet_scim_schemas(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scim/Schemas",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scim/ServiceProviderConfig`
    pub async fn ep_get_api_v1_fleet_scim_serviceproviderconfig(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scim/ServiceProviderConfig",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scim/Users`
    pub async fn ep_get_api_v1_fleet_scim_users(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scim/Users",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scim/Users/:id`
    pub async fn ep_get_api_v1_fleet_scim_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/scim/details`
    pub async fn ep_get_api_v1_fleet_scim_details(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scim/details",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scripts`
    pub async fn ep_get_api_v1_fleet_scripts(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/scripts", query, None)
            .await
    }

    /// `GET /api/v1/fleet/scripts/:id`
    pub async fn ep_get_api_v1_fleet_scripts_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scripts/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/scripts/batch`
    pub async fn ep_get_api_v1_fleet_scripts_batch(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/scripts/batch",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/scripts/batch/:batch_execution_id`
    pub async fn ep_get_api_v1_fleet_scripts_batch_by_batch_execution_id(
        &self,
        batch_execution_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scripts/batch/:batch_execution_id".to_string();
        let batch_execution_id_encoded = encode_path_segment(batch_execution_id.as_ref());
        path = path.replace(":batch_execution_id", &batch_execution_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/scripts/batch/:batch_execution_id/host_results`
    pub async fn ep_get_api_v1_fleet_scripts_batch_by_batch_execution_id_host_results(
        &self,
        batch_execution_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scripts/batch/:batch_execution_id/host_results".to_string();
        let batch_execution_id_encoded = encode_path_segment(batch_execution_id.as_ref());
        path = path.replace(":batch_execution_id", &batch_execution_id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/sessions/:id`
    pub async fn ep_get_api_v1_fleet_sessions_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/sessions/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/setup_experience/eula/:token`
    pub async fn ep_get_api_v1_fleet_setup_experience_eula_by_token(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/setup_experience/eula/:token".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/setup_experience/eula/metadata`
    pub async fn ep_get_api_v1_fleet_setup_experience_eula_metadata(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/setup_experience/eula/metadata",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/setup_experience/script`
    pub async fn ep_get_api_v1_fleet_setup_experience_script(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/setup_experience/script",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/setup_experience/software`
    pub async fn ep_get_api_v1_fleet_setup_experience_software(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/setup_experience/software",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/software/app_store_apps`
    pub async fn ep_get_api_v1_fleet_software_app_store_apps(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/software/app_store_apps",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/software/fleet_maintained_apps`
    pub async fn ep_get_api_v1_fleet_software_fleet_maintained_apps(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/software/fleet_maintained_apps",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/software/fleet_maintained_apps/:id`
    pub async fn ep_get_api_v1_fleet_software_fleet_maintained_apps_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/fleet_maintained_apps/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/software/install/:install_uuid/results`
    pub async fn ep_get_api_v1_fleet_software_install_by_install_uuid_results(
        &self,
        install_uuid: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/install/:install_uuid/results".to_string();
        let install_uuid_encoded = encode_path_segment(install_uuid.as_ref());
        path = path.replace(":install_uuid", &install_uuid_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/software/self_service_categories`
    pub async fn ep_get_api_v1_fleet_software_self_service_categories(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/software/self_service_categories",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/software/titles`
    pub async fn ep_get_api_v1_fleet_software_titles(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/software/titles",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/software/titles/:id`
    pub async fn ep_get_api_v1_fleet_software_titles_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/software/titles/:id/icon`
    pub async fn ep_get_api_v1_fleet_software_titles_by_id_icon(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:id/icon".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/software/titles/:id/package`
    pub async fn ep_get_api_v1_fleet_software_titles_by_id_package(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:id/package".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/software/versions`
    pub async fn ep_get_api_v1_fleet_software_versions(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/software/versions",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/software/versions/:id`
    pub async fn ep_get_api_v1_fleet_software_versions_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/versions/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/spec/enroll_secret`
    pub async fn ep_get_api_v1_fleet_spec_enroll_secret(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/spec/enroll_secret",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/sso`
    pub async fn ep_get_api_v1_fleet_sso(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/sso", query, None)
            .await
    }

    /// `GET /api/v1/fleet/users`
    pub async fn ep_get_api_v1_fleet_users(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/users", query, None)
            .await
    }

    /// `GET /api/v1/fleet/users/:id`
    pub async fn ep_get_api_v1_fleet_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/users/:id/sessions`
    pub async fn ep_get_api_v1_fleet_users_by_id_sessions(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/users/:id/sessions".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `GET /api/v1/fleet/version`
    pub async fn ep_get_api_v1_fleet_version(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::GET, "/api/v1/fleet/version", query, None)
            .await
    }

    /// `GET /api/v1/fleet/vpp_tokens`
    pub async fn ep_get_api_v1_fleet_vpp_tokens(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/vpp_tokens",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/vulnerabilities`
    pub async fn ep_get_api_v1_fleet_vulnerabilities(
        &self,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::GET,
            "/api/v1/fleet/vulnerabilities",
            query,
            None,
        )
        .await
    }

    /// `GET /api/v1/fleet/vulnerabilities/:cve`
    pub async fn ep_get_api_v1_fleet_vulnerabilities_by_cve(
        &self,
        cve: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/vulnerabilities/:cve".to_string();
        let cve_encoded = encode_path_segment(cve.as_ref());
        path = path.replace(":cve", &cve_encoded);
        self.send(reqwest::Method::GET, &path, query, None).await
    }

    /// `PATCH /api/v1/fleet/certificate_authorities/:id`
    pub async fn ep_patch_api_v1_fleet_certificate_authorities_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/certificate_authorities/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/config`
    pub async fn ep_patch_api_v1_fleet_config(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::PATCH, "/api/v1/fleet/config", query, body)
            .await
    }

    /// `PATCH /api/v1/fleet/fleets/:fleet_id/policies/:policy_id`
    pub async fn ep_patch_api_v1_fleet_fleets_by_fleet_id_policies_by_policy_id(
        &self,
        fleet_id: impl AsRef<str>,
        policy_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:fleet_id/policies/:policy_id".to_string();
        let fleet_id_encoded = encode_path_segment(fleet_id.as_ref());
        path = path.replace(":fleet_id", &fleet_id_encoded);
        let policy_id_encoded = encode_path_segment(policy_id.as_ref());
        path = path.replace(":policy_id", &policy_id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/fleets/:id`
    pub async fn ep_patch_api_v1_fleet_fleets_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/fleets/:id/secrets`
    pub async fn ep_patch_api_v1_fleet_fleets_by_id_secrets(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id/secrets".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/fleets/:id/users`
    pub async fn ep_patch_api_v1_fleet_fleets_by_id_users(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id/users".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/global/policies/:id`
    pub async fn ep_patch_api_v1_fleet_global_policies_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/global/policies/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/invites/:id`
    pub async fn ep_patch_api_v1_fleet_invites_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/invites/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/labels/:id`
    pub async fn ep_patch_api_v1_fleet_labels_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/labels/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/reports/:id`
    pub async fn ep_patch_api_v1_fleet_reports_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/reports/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/scim/Groups/:id`
    pub async fn ep_patch_api_v1_fleet_scim_groups_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Groups/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/scim/Users/:id`
    pub async fn ep_patch_api_v1_fleet_scim_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/scripts/:id`
    pub async fn ep_patch_api_v1_fleet_scripts_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scripts/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/setup_experience`
    pub async fn ep_patch_api_v1_fleet_setup_experience(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::PATCH,
            "/api/v1/fleet/setup_experience",
            query,
            body,
        )
        .await
    }

    /// `PATCH /api/v1/fleet/software/self_service_categories/:id`
    pub async fn ep_patch_api_v1_fleet_software_self_service_categories_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/self_service_categories/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/software/titles/:id/package`
    pub async fn ep_patch_api_v1_fleet_software_titles_by_id_package(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:id/package".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/software/titles/:title_id/app_store_app`
    pub async fn ep_patch_api_v1_fleet_software_titles_by_title_id_app_store_app(
        &self,
        title_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:title_id/app_store_app".to_string();
        let title_id_encoded = encode_path_segment(title_id.as_ref());
        path = path.replace(":title_id", &title_id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `PATCH /api/v1/fleet/users/:id`
    pub async fn ep_patch_api_v1_fleet_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PATCH, &path, query, body).await
    }

    /// `POST /api/v1/fleet/automations/reset`
    pub async fn ep_post_api_v1_fleet_automations_reset(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/automations/reset",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/bootstrap`
    pub async fn ep_post_api_v1_fleet_bootstrap(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/bootstrap",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/certificate_authorities`
    pub async fn ep_post_api_v1_fleet_certificate_authorities(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/certificate_authorities",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/certificate_authorities/:id/request_certificate`
    pub async fn ep_post_api_v1_fleet_certificate_authorities_by_id_request_certificate(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/certificate_authorities/:id/request_certificate".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/certificates`
    pub async fn ep_post_api_v1_fleet_certificates(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/certificates",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/change_password`
    pub async fn ep_post_api_v1_fleet_change_password(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/change_password",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/commands/run`
    pub async fn ep_post_api_v1_fleet_commands_run(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/commands/run",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/configuration_profiles`
    pub async fn ep_post_api_v1_fleet_configuration_profiles(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/configuration_profiles",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/configuration_profiles/batch`
    pub async fn ep_post_api_v1_fleet_configuration_profiles_batch(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/configuration_profiles/batch",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/configuration_profiles/resend/batch`
    pub async fn ep_post_api_v1_fleet_configuration_profiles_resend_batch(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/configuration_profiles/resend/batch",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/custom_variables`
    pub async fn ep_post_api_v1_fleet_custom_variables(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/custom_variables",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/device/:token/bypass_conditional_access`
    pub async fn ep_post_api_v1_fleet_device_by_token_bypass_conditional_access(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/device/:token/bypass_conditional_access".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/device/:token/configuration_profiles/:profile_uuid/resend`
    pub async fn ep_post_api_v1_fleet_device_by_token_configuration_profiles_by_profile_uuid_resend(
        &self,
        token: impl AsRef<str>,
        profile_uuid: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path =
            "/api/v1/fleet/device/:token/configuration_profiles/:profile_uuid/resend".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        let profile_uuid_encoded = encode_path_segment(profile_uuid.as_ref());
        path = path.replace(":profile_uuid", &profile_uuid_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/device/:token/refetch`
    pub async fn ep_post_api_v1_fleet_device_by_token_refetch(
        &self,
        token: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/device/:token/refetch".to_string();
        let token_encoded = encode_path_segment(token.as_ref());
        path = path.replace(":token", &token_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/disk_encryption`
    pub async fn ep_post_api_v1_fleet_disk_encryption(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/disk_encryption",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/enrollment_profiles/automatic`
    pub async fn ep_post_api_v1_fleet_enrollment_profiles_automatic(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/enrollment_profiles/automatic",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/fleets`
    pub async fn ep_post_api_v1_fleet_fleets(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/fleets", query, body)
            .await
    }

    /// `POST /api/v1/fleet/fleets/:fleet_id/policies/delete`
    pub async fn ep_post_api_v1_fleet_fleets_by_fleet_id_policies_delete(
        &self,
        fleet_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:fleet_id/policies/delete".to_string();
        let fleet_id_encoded = encode_path_segment(fleet_id.as_ref());
        path = path.replace(":fleet_id", &fleet_id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/fleets/:id/agent_options`
    pub async fn ep_post_api_v1_fleet_fleets_by_id_agent_options(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id/agent_options".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/fleets/:id/policies`
    pub async fn ep_post_api_v1_fleet_fleets_by_id_policies(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/fleets/:id/policies".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/forgot_password`
    pub async fn ep_post_api_v1_fleet_forgot_password(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/forgot_password",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/global/policies`
    pub async fn ep_post_api_v1_fleet_global_policies(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/global/policies",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/global/policies/delete`
    pub async fn ep_post_api_v1_fleet_global_policies_delete(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/global/policies/delete",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/hosts/:id/certificates/:certificate_template_id/resend`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_certificates_by_certificate_template_id_resend(
        &self,
        id: impl AsRef<str>,
        certificate_template_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path =
            "/api/v1/fleet/hosts/:id/certificates/:certificate_template_id/resend".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let certificate_template_id_encoded = encode_path_segment(certificate_template_id.as_ref());
        path = path.replace(":certificate_template_id", &certificate_template_id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/clear_passcode`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_clear_passcode(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/clear_passcode".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/configuration_profiles/:profile_uuid/resend`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_configuration_profiles_by_profile_uuid_resend(
        &self,
        id: impl AsRef<str>,
        profile_uuid: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path =
            "/api/v1/fleet/hosts/:id/configuration_profiles/:profile_uuid/resend".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let profile_uuid_encoded = encode_path_segment(profile_uuid.as_ref());
        path = path.replace(":profile_uuid", &profile_uuid_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/labels`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_labels(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/labels".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/lock`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_lock(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/lock".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/managed_account_password/rotate`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_managed_account_password_rotate(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/managed_account_password/rotate".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/query`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_query(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/query".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/recovery_lock_password/rotate`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_recovery_lock_password_rotate(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/recovery_lock_password/rotate".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/refetch`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_refetch(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/refetch".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/software/:software_title_id/install`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_software_by_software_title_id_install(
        &self,
        id: impl AsRef<str>,
        software_title_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/software/:software_title_id/install".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let software_title_id_encoded = encode_path_segment(software_title_id.as_ref());
        path = path.replace(":software_title_id", &software_title_id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/software/:software_title_id/uninstall`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_software_by_software_title_id_uninstall(
        &self,
        id: impl AsRef<str>,
        software_title_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/software/:software_title_id/uninstall".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        let software_title_id_encoded = encode_path_segment(software_title_id.as_ref());
        path = path.replace(":software_title_id", &software_title_id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/unlock`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_unlock(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/unlock".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/:id/wipe`
    pub async fn ep_post_api_v1_fleet_hosts_by_id_wipe(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/wipe".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/delete`
    pub async fn ep_post_api_v1_fleet_hosts_delete(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/hosts/delete",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/hosts/identifier/:identifier/query`
    pub async fn ep_post_api_v1_fleet_hosts_identifier_by_identifier_query(
        &self,
        identifier: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/identifier/:identifier/query".to_string();
        let identifier_encoded = encode_path_segment(identifier.as_ref());
        path = path.replace(":identifier", &identifier_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/hosts/transfer`
    pub async fn ep_post_api_v1_fleet_hosts_transfer(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/hosts/transfer",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/hosts/transfer/filter`
    pub async fn ep_post_api_v1_fleet_hosts_transfer_filter(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/hosts/transfer/filter",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/invites`
    pub async fn ep_post_api_v1_fleet_invites(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/invites", query, body)
            .await
    }

    /// `POST /api/v1/fleet/labels`
    pub async fn ep_post_api_v1_fleet_labels(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/labels", query, body)
            .await
    }

    /// `POST /api/v1/fleet/login`
    pub async fn ep_post_api_v1_fleet_login(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/login", query, body)
            .await
    }

    /// `POST /api/v1/fleet/logout`
    pub async fn ep_post_api_v1_fleet_logout(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/logout", query, body)
            .await
    }

    /// `POST /api/v1/fleet/managed_local_account`
    pub async fn ep_post_api_v1_fleet_managed_local_account(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/managed_local_account",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/perform_required_password_reset`
    pub async fn ep_post_api_v1_fleet_perform_required_password_reset(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/perform_required_password_reset",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/reports`
    pub async fn ep_post_api_v1_fleet_reports(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/reports", query, body)
            .await
    }

    /// `POST /api/v1/fleet/reports/:id/run`
    pub async fn ep_post_api_v1_fleet_reports_by_id_run(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/reports/:id/run".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/reports/delete`
    pub async fn ep_post_api_v1_fleet_reports_delete(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/reports/delete",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/reset_password`
    pub async fn ep_post_api_v1_fleet_reset_password(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/reset_password",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/scim/Groups`
    pub async fn ep_post_api_v1_fleet_scim_groups(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/scim/Groups",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/scim/Users`
    pub async fn ep_post_api_v1_fleet_scim_users(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/scim/Users",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/scripts`
    pub async fn ep_post_api_v1_fleet_scripts(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/scripts", query, body)
            .await
    }

    /// `POST /api/v1/fleet/scripts/batch/:batch_execution_id/cancel`
    pub async fn ep_post_api_v1_fleet_scripts_batch_by_batch_execution_id_cancel(
        &self,
        batch_execution_id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scripts/batch/:batch_execution_id/cancel".to_string();
        let batch_execution_id_encoded = encode_path_segment(batch_execution_id.as_ref());
        path = path.replace(":batch_execution_id", &batch_execution_id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/scripts/run`
    pub async fn ep_post_api_v1_fleet_scripts_run(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/scripts/run",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/scripts/run/batch`
    pub async fn ep_post_api_v1_fleet_scripts_run_batch(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/scripts/run/batch",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/setup_experience/eula`
    pub async fn ep_post_api_v1_fleet_setup_experience_eula(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/setup_experience/eula",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/setup_experience/script`
    pub async fn ep_post_api_v1_fleet_setup_experience_script(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/setup_experience/script",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/software/app_store_apps`
    pub async fn ep_post_api_v1_fleet_software_app_store_apps(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/software/app_store_apps",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/software/fleet_maintained_apps`
    pub async fn ep_post_api_v1_fleet_software_fleet_maintained_apps(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/software/fleet_maintained_apps",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/software/package`
    pub async fn ep_post_api_v1_fleet_software_package(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/software/package",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/software/self_service_categories`
    pub async fn ep_post_api_v1_fleet_software_self_service_categories(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/software/self_service_categories",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/software/web_apps`
    pub async fn ep_post_api_v1_fleet_software_web_apps(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/software/web_apps",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/spec/enroll_secret`
    pub async fn ep_post_api_v1_fleet_spec_enroll_secret(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/spec/enroll_secret",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/sso`
    pub async fn ep_post_api_v1_fleet_sso(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/sso", query, body)
            .await
    }

    /// `POST /api/v1/fleet/sso/callback`
    pub async fn ep_post_api_v1_fleet_sso_callback(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/sso/callback",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/targets`
    pub async fn ep_post_api_v1_fleet_targets(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/targets", query, body)
            .await
    }

    /// `POST /api/v1/fleet/translate`
    pub async fn ep_post_api_v1_fleet_translate(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/translate",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/users`
    pub async fn ep_post_api_v1_fleet_users(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::POST, "/api/v1/fleet/users", query, body)
            .await
    }

    /// `POST /api/v1/fleet/users/:id/require_password_reset`
    pub async fn ep_post_api_v1_fleet_users_by_id_require_password_reset(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/users/:id/require_password_reset".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::POST, &path, query, body).await
    }

    /// `POST /api/v1/fleet/users/admin`
    pub async fn ep_post_api_v1_fleet_users_admin(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/users/admin",
            query,
            body,
        )
        .await
    }

    /// `POST /api/v1/fleet/users/api_only`
    pub async fn ep_post_api_v1_fleet_users_api_only(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::POST,
            "/api/v1/fleet/users/api_only",
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/fleet/hosts/:id/device_mapping`
    pub async fn ep_put_api_v1_fleet_hosts_by_id_device_mapping(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/hosts/:id/device_mapping".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PUT, &path, query, body).await
    }

    /// `PUT /api/v1/fleet/logo`
    pub async fn ep_put_api_v1_fleet_logo(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(reqwest::Method::PUT, "/api/v1/fleet/logo", query, body)
            .await
    }

    /// `PUT /api/v1/fleet/scim/Groups/:id`
    pub async fn ep_put_api_v1_fleet_scim_groups_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Groups/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PUT, &path, query, body).await
    }

    /// `PUT /api/v1/fleet/scim/Users/:id`
    pub async fn ep_put_api_v1_fleet_scim_users_by_id(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/scim/Users/:id".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PUT, &path, query, body).await
    }

    /// `PUT /api/v1/fleet/setup_experience/software`
    pub async fn ep_put_api_v1_fleet_setup_experience_software(
        &self,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        self.send(
            reqwest::Method::PUT,
            "/api/v1/fleet/setup_experience/software",
            query,
            body,
        )
        .await
    }

    /// `PUT /api/v1/fleet/software/titles/:id/icon`
    pub async fn ep_put_api_v1_fleet_software_titles_by_id_icon(
        &self,
        id: impl AsRef<str>,
        query: Option<ApiQuery<'_>>,
        body: Option<&ApiRequestBody>,
    ) -> Result<ApiResponse> {
        let mut path = "/api/v1/fleet/software/titles/:id/icon".to_string();
        let id_encoded = encode_path_segment(id.as_ref());
        path = path.replace(":id", &id_encoded);
        self.send(reqwest::Method::PUT, &path, query, body).await
    }
}
