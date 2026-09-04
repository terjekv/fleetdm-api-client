use crate::client::FleetClient;
use crate::error::Result;
use crate::models::command::*;
use crate::paths;

/// MDM commands endpoint for managing device commands
pub struct CommandsEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> CommandsEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Run an MDM command on one or more hosts
    ///
    /// # Arguments
    /// * `command` - The MDM command to run (e.g., "DeviceLock", "EraseDevice", "RestartDevice")
    /// * `host_ids` - List of host IDs to run the command on
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_token("token")?
    ///     .build();
    ///
    /// let result = client.commands().run("RestartDevice", vec![1, 2, 3]).await?;
    /// println!("Command UUID: {}", result.command_uuid());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run(
        &self,
        command: impl Into<String>,
        host_ids: Vec<u64>,
    ) -> Result<RunCommandResponse> {
        let command = command.into();
        if command.trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "command must not be empty".into(),
            ));
        }
        if host_ids.is_empty() {
            return Err(crate::FleetError::Validation(
                "host IDs must not be empty".into(),
            ));
        }
        let req_body = RunCommandRequest { command, host_ids };
        let request = self
            .client
            .request(reqwest::Method::POST, paths::COMMANDS_RUN)?
            .json(&req_body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get the results of an MDM command
    ///
    /// # Arguments
    /// * `command_uuid` - The UUID of the command to get results for
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_token("token")?
    ///     .build();
    ///
    /// let results = client.commands().results("command-uuid-here").await?;
    /// for result in results.results() {
    ///     println!("{}: {}", result.hostname(), result.status());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn results(&self, command_uuid: impl AsRef<str>) -> Result<CommandResultsResponse> {
        if command_uuid.as_ref().trim().is_empty() {
            return Err(crate::FleetError::Validation(
                "command UUID must not be empty".into(),
            ));
        }
        let params = [("command_uuid", command_uuid.as_ref())];
        let path = crate::http::append_query(&paths::command_results(), Some(&params));
        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// List all MDM commands with optional filtering
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::FleetClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_token("token")?
    ///     .build();
    ///
    /// let commands = client.commands().list()
    ///     .per_page(10)
    ///     .request_type("RestartDevice")
    ///     .send()
    ///     .await?;
    /// for cmd in commands.commands() {
    ///     println!("{}: {} on {}", cmd.command_uuid(), cmd.request_type(), cmd.hostname());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListCommandsBuilder<'_> {
        ListCommandsBuilder {
            client: self.client,
            page: None,
            per_page: None,
            order_key: None,
            order_direction: None,
            host_identifier: None,
            request_type: None,
            command_status: None,
            after: None,
        }
    }
}

/// Builder for constructing list commands queries
pub struct ListCommandsBuilder<'a> {
    client: &'a FleetClient,
    page: Option<u32>,
    per_page: Option<u32>,
    order_key: Option<String>,
    order_direction: Option<String>,
    host_identifier: Option<String>,
    request_type: Option<String>,
    command_status: Option<String>,
    after: Option<String>,
}

impl<'a> ListCommandsBuilder<'a> {
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn order_key(mut self, key: impl Into<String>) -> Self {
        self.order_key = Some(key.into());
        self
    }

    pub fn order_direction(mut self, direction: crate::models::common::OrderDirection) -> Self {
        self.order_direction = Some(direction.to_string());
        self
    }

    /// Filter commands by host identifier (hostname, UUID, or serial number)
    pub fn host_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.host_identifier = Some(identifier.into());
        self
    }

    /// Filter commands by request type (e.g., "RestartDevice", "DeviceLock")
    pub fn request_type(mut self, request_type: impl Into<String>) -> Self {
        self.request_type = Some(request_type.into());
        self
    }

    /// Filter commands by status
    pub fn command_status(mut self, status: impl Into<String>) -> Self {
        self.command_status = Some(status.into());
        self
    }

    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Execute the list commands query
    pub async fn send(self) -> Result<ListCommandsResponse> {
        crate::models::common::validate_list_options(
            self.per_page,
            self.order_key.is_some(),
            self.order_direction.is_some(),
            self.after.is_some(),
        )?;
        let mut query_params = vec![];

        if let Some(page) = self.page {
            query_params.push(("page".to_string(), page.to_string()));
        }
        if let Some(per_page) = self.per_page {
            query_params.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(ref order_key) = self.order_key {
            query_params.push(("order_key".to_string(), order_key.clone()));
        }
        if let Some(ref order_direction) = self.order_direction {
            query_params.push(("order_direction".to_string(), order_direction.clone()));
        }
        if let Some(ref host_identifier) = self.host_identifier {
            query_params.push(("host_identifier".to_string(), host_identifier.clone()));
        }
        if let Some(ref request_type) = self.request_type {
            query_params.push(("request_type".to_string(), request_type.clone()));
        }
        if let Some(ref command_status) = self.command_status {
            query_params.push(("command_status".to_string(), command_status.clone()));
        }
        if let Some(ref after) = self.after {
            query_params.push(("after".to_string(), after.clone()));
        }

        let path = crate::http::append_query_params(paths::COMMANDS, &query_params);

        let request = self.client.request(reqwest::Method::GET, &path)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}
