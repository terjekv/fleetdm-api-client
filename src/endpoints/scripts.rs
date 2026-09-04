use crate::client::FleetClient;
use crate::error::Result;
use crate::http::{handle_response, send_with_retry};
use crate::models::script::*;

const MAX_SCRIPT_SIZE: usize = 1536 * 1024;

/// Endpoint for script-related operations
pub struct ScriptsEndpoint {
    client: FleetClient,
}

impl ScriptsEndpoint {
    pub(crate) fn new(client: FleetClient) -> Self {
        Self { client }
    }

    /// List scripts with optional filters
    pub fn list(&self) -> ListScriptsBuilder<'_> {
        ListScriptsBuilder {
            client: &self.client,
            query: ListScriptsQuery::new(),
        }
    }

    /// Get a specific script by ID
    pub async fn get(&self, script_id: u64) -> Result<GetScriptResponse> {
        let path = crate::paths::script(script_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Create/upload a new script
    pub async fn create(&self, req: CreateScriptRequest) -> Result<CreateScriptResponse> {
        crate::models::common::validate_upload_filename(&req.name)?;
        if req.script_contents.is_empty() {
            return Err(crate::FleetError::Validation(
                "script contents must not be empty".into(),
            ));
        }
        if req.script_contents.len() > MAX_SCRIPT_SIZE {
            return Err(crate::FleetError::Validation(format!(
                "script exceeds Fleet's {MAX_SCRIPT_SIZE}-byte upload limit"
            )));
        }
        let path = crate::paths::SCRIPTS;

        let send_once = || async {
            let mut form = reqwest::multipart::Form::new();
            let part = reqwest::multipart::Part::text(req.script_contents.clone())
                .file_name(req.name.clone());
            form = form.part("script", part);

            if let Some(fleet_id) = req.team_id {
                form = form.text("fleet_id", fleet_id.to_string());
            }

            let request = self
                .client
                .request(reqwest::Method::POST, path)?
                .multipart(form);
            request.send().await.map_err(Into::into)
        };

        let response = if let Some(policy) = self.client.retry_policy() {
            send_with_retry(policy, send_once).await?
        } else {
            send_once().await?
        };
        handle_response(response).await
    }

    /// Replace an existing script's contents.
    pub async fn update(
        &self,
        script_id: u64,
        script: crate::models::common::FileUpload,
    ) -> Result<GetScriptResponse> {
        if script_id == 0 {
            return Err(crate::FleetError::Validation(
                "script_id must be greater than zero".into(),
            ));
        }
        if script.bytes().len() > MAX_SCRIPT_SIZE {
            return Err(crate::FleetError::Validation(format!(
                "script exceeds Fleet's {MAX_SCRIPT_SIZE}-byte upload limit"
            )));
        }
        let path = crate::paths::script(script_id);
        let response = crate::http::send_rebuildable_request(
            || {
                let form = reqwest::multipart::Form::new().part("script", script.to_part()?);
                Ok(self
                    .client
                    .request(reqwest::Method::PATCH, &path)?
                    .multipart(form))
            },
            self.client.retry_policy(),
        )
        .await?;
        handle_response(response).await
    }

    /// Delete a script by ID
    pub async fn delete(&self, script_id: u64) -> Result<()> {
        let path = crate::paths::script(script_id);
        let request = self.client.request(reqwest::Method::DELETE, &path)?;
        crate::http::send_empty_request(request, self.client.retry_policy()).await
    }

    /// Run a script on a host
    pub async fn run(&self, req: RunScriptRequest) -> Result<ScriptExecutionResult> {
        if req.host_id == 0 || req.script_id == 0 {
            return Err(crate::FleetError::Validation(
                "host_id and script_id must be greater than zero".into(),
            ));
        }
        let path = crate::paths::SCRIPTS_RUN;
        let request = self.client.request(reqwest::Method::POST, path)?;
        crate::http::send_request(request.json(&req), self.client.retry_policy()).await
    }

    /// Get the result of a script execution
    pub async fn get_result(&self, execution_id: &str) -> Result<ScriptExecutionResult> {
        if execution_id.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "execution_id must not be empty".into(),
            ));
        }
        let path = crate::paths::script_results(execution_id);
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

/// Builder for constructing list scripts queries
pub struct ListScriptsBuilder<'a> {
    client: &'a FleetClient,
    query: ListScriptsQuery,
}

impl<'a> ListScriptsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.query = self.query.page(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.query = self.query.per_page(per_page);
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

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.query = self.query.after(after);
        self
    }

    /// Execute the list scripts query
    pub async fn send(self) -> Result<ListScriptsResponse> {
        self.query.validate()?;
        let params = self.query.to_query_params();
        let path = crate::http::append_query_params(crate::paths::SCRIPTS, &params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_scripts_query_builder() {
        let query = ListScriptsQuery::new().page(1).per_page(25).team_id(5);

        let params = query.to_query_params();

        assert!(params.contains(&("page".to_string(), "1".to_string())));
        assert!(params.contains(&("per_page".to_string(), "25".to_string())));
        assert!(params.contains(&("team_id".to_string(), "5".to_string())));
    }
}
